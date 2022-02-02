mod transport;

use smol::lock::Mutex;
use transport::Transport;
use async_net::{TcpStream, AsyncToSocketAddrs};
use semver::Version;
use crate::error::{BoltError, BoltResult};
use futures_lite::{AsyncWriteExt, AsyncReadExt};
use bytes::BytesMut;
use super::response::Response;
use packstream_serde::message::Run;

#[derive(Debug)]
pub struct Client {
    transport: Transport,
}

// 1. Run Message -> (A)Success (B)Failure (C)Ignored
//      A: Either -> (A1)PullAll (A2)Discard
//          A1: PullAll -> Rows -> Success
//          A2: Discard -> Success
//      B: AckFailure
//          B1: Success -> next
//          B2: Ignored -> Reset
//      C: Reset if not recoverable
//
//  2. Send Message -> (A)Success (B)Failure (C)Ignored
//      A: DiscardAll
//      B: AckFailure
//      C: Reset if not recoverable

impl Client {
    pub async fn connect<A: AsyncToSocketAddrs, U: AsRef<str>, P: AsRef<str>>(addr: A, usr: U, pwd: P) -> BoltResult<Self> {
        use packstream_serde::message::BasicAuth;

        let mut transport = Transport::new(addr, BasicAuth {
            principal: String::from(usr.as_ref()),
            credentials: String::from(pwd.as_ref()),
            scheme: String::from("basic"),
        }).await?;

        Ok(Self { transport })
    }

    pub async fn send
        <T: for<'de> serde::Deserialize<'de> + std::fmt::Debug>
        (&mut self, message: &Run, pull_all: bool)
        -> BoltResult<Response<T>>
    {
        use packstream_serde::{
            packstream::{PackstreamStructure, EmptyPackstreamStructure},
            message::{Ignored, PullAll, DiscardAll, Success, Failure, Reset, Record, AckFailure, SummaryMessage},
            constants::marker::TINY_STRUCT,
            from_bytes,
            to_bytes,
            Value,
        };

        // Send two messages: request message and message telling server what to do
        // with the request results.
        self.transport.write_batch(&[
           &to_bytes(&message)?,
           if pull_all { &PullAll::MSG } else { &DiscardAll::MSG }
        ]).await?;

        // Read the response for the messages sent do server.
        let response = self.transport.read().await?;

        if Failure::check_header(response[0], response[1]) {

            let second_response = self.transport.read().await?;

            // Second message sent is always ignored until ACK_FAILURE is sent in response to
            // received first message FAILURE.
            debug_assert!(Ignored::check_header(second_response[0], second_response[1]));

            // Send ACK_FAILURE to server.
            self.transport.write(&AckFailure::MSG).await?;

            let ack_response = self.transport.read().await?;

            if Success::check_header(ack_response[0], ack_response[1]) {
                // TODO: return error with metadata from "response" error.
                return Err(BoltError::from(from_bytes::<SummaryMessage>(&response[..])?));
            } else {
                // ACK_FAILURE failed.
                // TODO: RESET connection.
                return Err(BoltError::from(from_bytes::<SummaryMessage>(&response[..])?));
            }
        } else if Ignored::check_header(response[0], response[1]) {
            let second_response = self.transport.read().await?;

            // Second message sent is always ignored until ACK_FAILURE is sent in response to
            // received first message FAILURE.
            debug_assert!(Ignored::check_header(second_response[0], second_response[1]));

            return Err(BoltError::from("Request failed: Ignored"));
        }

        let success = from_bytes::<Success>(&response).expect("message to be valid Success");
        let mut response = Response::from(success);

        if !pull_all {
            // TODO: read success after discard.
            self.transport.read().await?;
            return Ok(response);
        }

        loop {
            let msg = self.transport.read().await?;
            let marker = msg[0];
            let signature = msg[1];

            if Record::check_header(marker, signature) {
                match from_bytes::<Vec<T>>(&msg[2..]) {
                    Ok(values) => response.push_row(values),
                    Err(err) => unimplemented!(),
                }
            } else {
                break;
            }
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use packstream_serde::message::Run;
    use packstream_serde::Value;
    use std::collections::HashMap;
    use packstream_serde::{to_bytes, from_bytes};

    fn test_connection () -> impl std::future::Future<Output = BoltResult<Client>> {
        Client::connect("localhost:7687", "neo4j", "bolt-rs")
    }

    fn run_message (msg: impl Into<String>) -> Run {
        Run {
            statement: msg.into(),
            parameters: HashMap::new(),
        }
    }

    #[test]
    fn connect () {
        smol::block_on(async {
            let bolt = test_connection().await;
            assert!(bolt.is_ok());
        });
    }

    #[test]
    fn syntax_failure () {
        smol::block_on(async {
            let mut bolt = test_connection().await.unwrap();

            let message = run_message("NRUTER 1");
            let response = bolt.send::<Value>(&message, true).await;

            assert!(response.is_err());

            let message = run_message("RETURN 1");
            let response = bolt.send::<u8>(&message, true).await;

            assert!(response.is_ok());
            assert!(response.unwrap().into_rows()[0][0] == 1);
        });
    }

    #[test]
    fn multiple_requests () {
        smol::block_on(async {
            let mut bolt = test_connection().await.unwrap();

            let message_0 = run_message("RETURN { name: 'John' } as X, { name: 'Jane' } as Y");
            let message_1 = run_message("RETURN [1, 2] as X");
            let message_2 = run_message("RETURN [2, 3] as Y");
            let message_3 = run_message("RETURN 2, 3");

            let response_0 = bolt.send::<Value>(&message_0, true).await.unwrap();
            assert!(response_0.into_rows().len() == 1);

            let response_1 = bolt.send::<Vec<u8>>(&message_1, true).await.unwrap();
            assert!(response_1.into_rows().len() == 1);

            let response_2 = bolt.send::<Vec<u8>>(&message_2, true).await.unwrap();
            assert!(response_2.into_rows().len() == 1);

            let response_3 = bolt.send::<u8>(&message_3, true).await.unwrap();
            assert!(response_3.into_rows()[0].len() == 2);
        });
    }

    #[test]
    fn discard_all () {
        smol::block_on(async {
            let mut bolt = test_connection().await.unwrap();

            let message_0 = run_message("RETURN [1, 2] as X");
            let message_1 = run_message("RETURN [2, 3] as Y");

            let response_0 = bolt.send::<()>(&message_0, false).await.expect("t 1");
            let response_1 = bolt.send::<Vec<u8>>(&message_1, true).await.expect("t 2");

            assert!(response_0.into_rows().len() == 0);
            assert!(response_1.into_rows().len() == 1);
        });
    }

    #[test]
    fn returns_table() {
        smol::block_on(async {
            let mut bolt = test_connection().await.unwrap();

            let message_0 = run_message("RETURN 'neo' as name, '4j' as surname");
            let message_1 = run_message("RETURN [1, 2] as X");

            let response_0 = bolt.send::<String>(&message_0, true).await;
            let response_1 = bolt.send::<Vec<u8>>(&message_1, true).await;

            assert!(response_0.is_ok());
            assert!(response_1.is_ok());
        });
    }
    
    //#[test]
    //fn ack_failure_retry_multiple () {
    //}
}
