use super::*;
use packstream_serde::constants::marker::*;
use packstream_serde::message::*;
use std::collections::HashMap;

mod request_message {
    use super::*;

    #[test]
    fn init() {
        // Init {
        //      client: "test"
        //      auth: {
        //          scheme: "basic"
        //          principal: "test"
        //          credentials: "test"
        //      }
        // }
        const BYTES: &[u8] = &[
            0xB2, 0x01, 0x8C, 0x4D, 0x79, 0x43, 0x6C, 0x69, 0x65, 0x6E, 0x74,
            0x2F, 0x31, 0x2E, 0x30, 0xA3, 0x86, 0x73, 0x63, 0x68, 0x65, 0x6D,
            0x65, 0x85, 0x62, 0x61, 0x73, 0x69, 0x63, 0x89, 0x70, 0x72, 0x69,
            0x6E, 0x63, 0x69, 0x70, 0x61, 0x6C, 0x85, 0x6E, 0x65, 0x6F, 0x34,
            0x6A, 0x8B, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6E, 0x74, 0x69, 0x61,
            0x6C, 0x73, 0x86, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74,
        ];

        let client = String::from("test");
        let scheme = String::from("basic");
        let principal = String::from("test");
        let credentials = String::from("test");

        ser_de::<Init>(BYTES);
        ser_de::<RequestMessage>(BYTES);

        de_ser(Init {
            client: client.clone(),
            auth: BasicAuth {
                scheme: scheme.clone(),
                principal: principal.clone(),
                credentials: credentials.clone(),
            }
        });
        de_ser(RequestMessage::Init(Init {
            client: client.clone(),
            auth: BasicAuth {
                scheme: scheme.clone(),
                principal: principal.clone(),
                credentials: credentials.clone(),
            }
        }));

        de_err::<Init>(&BYTES[0..(BYTES.len() - 1)]);
    }


    #[test]
    fn ack_failure() {
        const BYTES: &[u8] = &[TINY_STRUCT + AckFailure::LEN, AckFailure::SIG];

        ser_de::<AckFailure>(BYTES);
        ser_de::<RequestMessage>(BYTES);

        de_ser(AckFailure);
        de_ser(RequestMessage::AckFailure(AckFailure));

        de_err::<AckFailure>(&[TINY_STRUCT, AckFailure::SIG + 1]);
    }

    #[test]
    fn reset() {
        const BYTES: &[u8] = &[TINY_STRUCT + Reset::LEN, Reset::SIG];

        ser_de::<Reset>(BYTES);
        ser_de::<RequestMessage>(BYTES);

        de_ser(Reset);
        de_ser(RequestMessage::Reset(Reset));

        de_err::<Reset>(&[TINY_STRUCT, Reset::SIG + 1]);
    }

    #[test]
    fn run() {
        const BYTES: &[u8] = &[TINY_STRUCT + Run::LEN, Run::SIG, TINY_STRING, TINY_MAP];

        ser_de::<Run>(BYTES);
        ser_de::<RequestMessage>(BYTES);

        de_ser(Run {
            statement: String::new(),
            parameters: HashMap::new(),
        });
        de_ser(RequestMessage::Run(Run {
            statement: String::new(),
            parameters: HashMap::new(),
        }));

        de_err::<Run>(&BYTES[0..(BYTES.len() - 1)]);
    }

    #[test]
    fn discard_all() {
        const BYTES: &[u8] = &[TINY_STRUCT + DiscardAll::LEN, DiscardAll::SIG];

        ser_de::<DiscardAll>(BYTES);
        ser_de::<RequestMessage>(BYTES);

        de_ser(DiscardAll);
        de_ser(RequestMessage::DiscardAll(DiscardAll));

        de_err::<DiscardAll>(&[TINY_STRUCT, DiscardAll::SIG + 1]);
    }

    #[test]
    fn pull_all() {
        const BYTES: &[u8] = &[TINY_STRUCT + PullAll::LEN, PullAll::SIG];

        ser_de::<PullAll>(BYTES);
        ser_de::<RequestMessage>(BYTES);

        de_ser(PullAll);
        de_ser(RequestMessage::PullAll(PullAll));

        de_err::<PullAll>(&[TINY_STRUCT, PullAll::SIG + 1]);
    }
}

mod summary_message {
    use super::*;


    #[test]
    fn success() {
        const BYTES: &[u8] = &[TINY_STRUCT + Success::LEN, Success::SIG, TINY_MAP];

        ser_de::<Success>(BYTES);
        ser_de::<SummaryMessage>(BYTES);

        de_ser(Success {
            metadata: HashMap::new(),
        });
        de_ser(SummaryMessage::Success(Success {
                metadata: HashMap::new(),
        }));

        de_err::<Success>(&BYTES[0..(BYTES.len() - 1)]);
    }


    #[test]
    fn ignored() {
        const BYTES: &[u8] = &[TINY_STRUCT + Ignored::LEN, Ignored::SIG];

        ser_de::<Ignored>(BYTES);
        ser_de::<SummaryMessage>(BYTES);

        de_ser(Ignored);
        de_ser(SummaryMessage::Ignored(Ignored));

        de_err::<Ignored>(&[TINY_STRUCT, Ignored::SIG + 1]);
    }

    #[test]
    fn failure() {
        const BYTES: &[u8] = &[TINY_STRUCT + Failure::LEN, Failure::SIG, TINY_MAP];

        ser_de::<Failure>(BYTES);
        ser_de::<SummaryMessage>(BYTES);

        de_ser(Failure {
            metadata: HashMap::new(),
        });
        de_ser(SummaryMessage::Failure(Failure {
            metadata: HashMap::new(),
        }));

        de_err::<Failure>(&[TINY_STRUCT + 1, Failure::SIG + 1, TINY_MAP]);
    }
}

#[test]
fn record() {
    const BYTES: &[u8] = &[TINY_STRUCT + Record::LEN, Record::SIG, TINY_LIST + 1, 1];

    ser_de::<Record>(BYTES);

    de_ser(Record { fields: Vec::new() });

    de_err::<Record>(&BYTES[0..(BYTES.len() - 1)]);
}


