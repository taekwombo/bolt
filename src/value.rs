mod structure;
mod de;
mod ser;

use std::collections::HashMap;
use std::fmt;
use serde_bytes::{Bytes, ByteBuf};

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    // TODO: Revisit Bytes
    Bytes(ByteBuf)
    // Structure(Structure),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => f.debug_tuple("Null").finish(),
            Self::Bool(v) => f.debug_tuple("Bool").field(v).finish(),
            Self::I64(v) => f.debug_tuple("I64").field(v).finish(),
            Self::F64(v) => f.debug_tuple("F64").field(v).finish(),
            Self::String(v) => f.debug_tuple("String").field(v).finish(),
            Self::List(v) => f.debug_tuple("List").field(v).finish(),
            Self::Map(v) => f.debug_tuple("Map").field(v).finish(),
            Self::Bytes(v) => f.debug_tuple("Bytes").field(v).finish(),
            // Self::Structure(v) => write!(f, "{}", v),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_default () {
        assert_eq!(Value::default(), Value::Null);
    }
}
