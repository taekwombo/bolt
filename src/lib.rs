// TODO: Should it forbid deserializing uint?

mod de;
mod error;
mod marker;
mod marker_bytes;
mod read;
mod ser;
mod value;

pub use de::from_bytes;
pub use ser::to_bytes;
pub use value::{Value, Structure};
