// TODO: Should it forbid deserializing uint?

#[cfg(test)]
#[macro_use] mod macros;
 
mod de;
mod error;
mod marker;
mod constants;
mod read;
mod ser;
mod value;

pub use de::from_bytes;
pub use ser::to_bytes;
pub use value::{Value, Structure};
