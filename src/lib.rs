//! # Serde Bolt
//!
//! Based on [BoltProtocol](https://boltprotocol.org/)
//!
//! # Parsing bytes as strongly typed values
//!
//! ```
//! # use serde_bolt::{from_bytes, error::SerdeResult};
//! # use serde_derive::{Deserialize};
//!
//! #[derive(Deserialize)]
//! struct BasicAuth<'a> {
//!     username: &'a str,
//!     password: &'a str,
//! }
//!
//! const BYTES: &[u8] = &[162, 136, 117, 115, 101, 114, 110, 97, 109, 101, 132, 74, 111, 104, 110, 136, 112, 97, 115, 115, 119, 111, 114, 100, 131, 68, 111, 101];
//! fn typed_example () -> SerdeResult<()> {
//!     let auth: BasicAuth = from_bytes(&BYTES)?;
//!     println!("Hello {}, leaking your password {} :(", auth.username, auth.password);
//!     return Ok(());
//! }
//!
//! #
//! # fn main() {
//! #   typed_example().unwrap();
//! # }
//! #
//! ```

#[macro_use]
mod macros;

pub mod constants;
mod de;
pub mod error;
mod marker;
mod read;
mod ser;
pub mod value;

pub use de::from_bytes;
pub use ser::to_bytes;
pub use value::{from_value, to_value, Structure, Value};
