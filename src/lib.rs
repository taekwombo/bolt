//! # Packstream Serde Implementation
//! Specification available on the [7687.org] website.
//!
//! # Examples
//! Parsing bytes as strongly typed values.
//!
//! ```
//! # use packstream_serde::{from_bytes, error::PackstreamResult};
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
//! fn typed_example (bytes: &[u8]) -> PackstreamResult<()> {
//!     let auth: BasicAuth = from_bytes(bytes)?;
//!     println!("Hello {}, leaking your password {}", auth.username, auth.password);
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #   typed_example(BYTES).unwrap();
//! # }
//! #
//! ```
//!
//! Parsing bytes as loosely-typed value.
//!
//! ```
//! # use packstream_serde::error::PackstreamResult;
//! # use packstream_serde::{from_bytes, Value};
//! #
//! const BYTES: &[u8] = &[0xC0];
//! 
//! fn untyped_example(bytes: &[u8]) -> PackstreamResult<()> {
//!     match from_bytes(bytes)? {
//!         Value::Null => {
//!             Ok(())
//!         }
//!         _ => panic!("Expected a Null value."),
//!     }
//! }
//!
//! #
//! # fn main() {
//! #   untyped_example(BYTES).unwrap();
//! # }
//! #
//! ```
//! [7687.org]: https://7687.org/.
//! [`Value`]: value::Value

pub mod packstream;
pub use packstream::{PackstreamStructure, EmptyPackstreamStructure};

#[macro_use]
mod macros;

pub mod constants;
pub mod error;
pub mod marker;
pub mod read;
pub mod value;
#[doc(inline)]
pub use value::{from_value, to_value, Structure, Value, structure};

pub mod message;
pub use message::{RequestMessage, SummaryMessage, Record};

mod de;
#[doc(inline)]
pub use de::from_bytes;

mod ser;
#[doc(inline)]
pub use ser::to_bytes;
