//! # Bolt Protocol Serialization
//!
//!
//! The Bolt network protocol is a highly efficient, lightweight client-server protocol designed for database applications ([website]).
//!
//! # Parsing bytes as strongly typed values.
//!
//! ```
//! # use serde_bolt::{from_bytes, error::SerdeResult};
//! # use serde_derive::Deserialize;
//! #[derive(Deserialize)]
//! struct BasicAuth<'a> {
//!     username: &'a str,
//!     password: &'a str,
//! }
//! #
//! # const BYTES: &[u8] = &[162, 136, 117, 115, 101, 114, 110, 97, 109, 101, 132, 74, 111, 104, 110, 136, 112, 97, 115, 115, 119, 111, 114, 100, 131, 68, 111, 101];
//! #
//!
//! fn typed_example (bytes: &[u8]) -> SerdeResult<()> {
//!     let auth: BasicAuth = from_bytes(bytes)?;
//!     println!("Hello {}, leaking your password {} :(", auth.username, auth.password);
//!     return Ok(());
//! }
//! #
//! # fn main() {
//! #   typed_example(BYTES).unwrap();
//! # }
//! #
//! ```
//!
//! # Parsing bytes as loosely-typed value.
//!
//! ```
//! # use serde_bolt::error::SerdeResult;
//! # use serde_bolt::{from_bytes, Value};
//! #
//! # const BYTES: &[u8] = &[162, 136, 117, 115, 101, 114, 110, 97, 109, 101, 132, 74, 111, 104, 110, 136, 112, 97, 115, 115, 119, 111, 114, 100, 131, 68, 111, 101];
//! #
//!
//! fn untyped_example(bytes: &[u8]) -> SerdeResult<()> {
//!     match from_bytes(bytes)? {
//!         Value::Map(map) => {
//!             println!("{:?}", map);
//!             Ok(())
//!         }
//!         _ => panic!("This should be a map, trust me."),
//!     }
//! }
//!
//! #
//! # fn main() {
//! #   untyped_example(BYTES).unwrap();
//! # }
//! #
//! ```
//! [website]: https://boltprotocol.org/.
//! [`Value`]: value::Value

#![warn(missing_docs)]

#[macro_use]
mod macros;

pub mod constants;
mod de;
pub mod error;
pub mod marker;
pub mod read;
mod ser;
pub mod value;

#[doc(inline)]
pub use de::from_bytes;
#[doc(inline)]
pub use ser::to_bytes;
#[doc(inline)]
pub use value::{from_value, to_value, Structure, Value};
