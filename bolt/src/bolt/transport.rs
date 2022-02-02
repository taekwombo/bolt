use async_net::{TcpStream, AsyncToSocketAddrs};
use crate::error::{BoltResult, BoltError};
use bytes::{BytesMut, BufMut};
use futures_lite::{AsyncWriteExt, AsyncReadExt};
use semver::Version;

// u16::MAX
const MAX_CHUNK_SIZE: usize = 65_535;
const CLIENT_NAME: &str = "rust-bolt/0.0.1";

const HANDSHAKE: [u8; 20] = [
   0x60, 0x60, 0xB0, 0x17,
   0x00, 0x00, 0x00, 0x01,
   0x00, 0x00, 0x00, 0x00,
   0x00, 0x00, 0x00, 0x00,
   0x00, 0x00, 0x00, 0x00,
];

#[derive(Debug)]
pub struct Transport {
   stream: TcpStream,
   buffer: BytesMut,
}

impl Transport {
   pub async fn negotiate_version (stream: &mut TcpStream) -> BoltResult<Version> {
      let n = stream.write(&HANDSHAKE).await?;

      let mut picked_version: [u8; 4] = [0; 4];
      stream.read(&mut picked_version).await?;

      let [_, patch, minor, major] = picked_version;

      if major == 1 {
         Ok(Version::new(
               major.into(),
               minor.into(),
               patch.into(),
               ))
      } else {
         Err(BoltError::create("Version negotiation failed"))
      }
   }

   pub async fn new<A: AsyncToSocketAddrs> (addr: A, auth: packstream_serde::message::BasicAuth) -> BoltResult<Self> {
      use packstream_serde::{to_bytes, from_bytes, from_value};
      use packstream_serde::message::{Init, BasicAuth, SummaryMessage};

      let mut stream = TcpStream::connect(addr).await?;
      let version = Self::negotiate_version(&mut stream).await?;
      let mut transport = Self { stream, buffer: BytesMut::with_capacity(MAX_CHUNK_SIZE) };

      transport.write(&to_bytes(&Init {
         auth,
         client: String::from(CLIENT_NAME),
      })?).await?;

      match from_bytes::<SummaryMessage>(&transport.read().await?)? {
         SummaryMessage::Success(mut success) => Ok(transport),
         _ => Err(BoltError::create("unable to authorize")),
      }
   }

   // Assumes that the buffer is cleared.
   pub fn append_message<'a> (&mut self, message: &'a [u8]) -> Option<&'a [u8]> {
      use std::convert::TryFrom;

      let available = MAX_CHUNK_SIZE - self.buffer.len();
      let msg_len = message.len();
      let chunk_len = std::cmp::min(msg_len, available - 4);

      self.buffer.put_u16(u16::try_from(msg_len).expect("Chunk length to fit in two bytes"));
      self.buffer.extend_from_slice(&message[0..chunk_len]);

      if chunk_len == msg_len {
         self.buffer.put_u16(0);

         return None;
      }

      Some(&message[chunk_len..])
   }

   pub async fn write (&mut self, message: &[u8]) -> BoltResult<()> {
      self.buffer.clear();

      let mut to_write = message;
      while let Some(remaining) = self.append_message(to_write) {
         self.stream.write_all(&self.buffer).await?;
         self.buffer.clear();
         to_write = remaining; 
      }

      self.stream.write_all(&self.buffer).await?;

      Ok(())
   }

   pub async fn write_batch (&mut self, messages: &[&[u8]]) -> BoltResult<()> {
      self.buffer.clear();

      let mut to_write: &[u8] = &[0; 0];
      for message in messages {
         to_write = *message;

         while let Some(remaining) = self.append_message(to_write) {
            self.stream.write_all(&self.buffer).await?;
            self.buffer.clear();
            to_write = remaining;
         }
      }

      self.stream.write_all(&self.buffer).await?;

      Ok(())
   }

   pub async fn read(&mut self) -> BoltResult<BytesMut> {
      let mut response = BytesMut::new();
      let mut chunk = BytesMut::new();

      let mut header_bytes: [u8; 2] = [0; 2];
      self.stream.read_exact(&mut header_bytes).await?;
      let mut len = usize::from(u16::from_be_bytes(header_bytes));


      loop {
         chunk.resize(len + 2, 0);
         self.stream.read_exact(&mut chunk).await?;

         let marker = u16::from_be_bytes([chunk[len], chunk[len + 1]]);
         response.extend_from_slice(&chunk[0..len]);

         if marker == 0 {
            break;
         } else {
            len = usize::from(marker);
         }
      }

      Ok(response)
   }
}

