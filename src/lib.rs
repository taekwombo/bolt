#[cfg(test)]
#[macro_use]
mod test;

#[macro_use]
mod macros;

pub mod constants;
mod de;
mod deserializer;
pub mod error;
mod marker;
mod read;
mod ser;
pub mod value;

pub use de::from_bytes;
pub use ser::to_bytes;
pub use value::{from_value, Value};
