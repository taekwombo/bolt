mod de;
mod ser;
mod display;
pub mod structure;
pub use de::from_value;
pub use ser::to_value;

use serde_bytes::ByteBuf;
use std::collections::HashMap;
use std::fmt;

pub use structure::Structure;

/// Represents any [Packstream value].
///
/// [Packstream value]: https://7687.org/packstream/packstream-specification-1.html
#[derive(PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Bytes(ByteBuf),
    Structure(Structure),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => f.debug_tuple("Value::Null").finish(),
            Self::Bool(v) => f.debug_tuple("Value::Bool").field(v).finish(),
            Self::I64(v) => f.debug_tuple("Value::I64").field(v).finish(),
            Self::F64(v) => f.debug_tuple("Value::F64").field(v).finish(),
            Self::String(v) => f.debug_tuple("Value::String").field(v).finish(),
            Self::List(v) => f.debug_tuple("Value::List").field(v).finish(),
            Self::Map(v) => f.debug_tuple("Value::Map").field(v).finish(),
            Self::Bytes(v) => f.debug_tuple("Value::Bytes").field(v).finish(),
            Self::Structure(v) => f.debug_tuple("Value::Structure").field(v).finish(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => f.debug_tuple("Null").finish(),
            Self::Bool(v) => fmt::Display::fmt(v, f),
            Self::I64(v) => fmt::Display::fmt(v, f),
            Self::F64(v) => fmt::Display::fmt(v, f),
            Self::String(v) => fmt::Display::fmt(v, f),
            Self::List(v) => display::display_value_list(v, f),
            Self::Map(v) => display::display_value_hash_map(v, f),
            Self::Bytes(v) => f.debug_list().entries(v.iter()).finish(),
            Self::Structure(v) => fmt::Display::fmt(v, f),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}
