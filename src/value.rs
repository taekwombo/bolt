mod de;
mod ser;

use serde_bytes::ByteBuf;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Bytes(ByteBuf), // TODO: Revisit Bytes - use Bytes instead of ByteBuf
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
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

#[derive(Debug, serde_derive::Deserialize, PartialEq)]
pub struct Structure(Vec<Value>);

impl Structure {
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn push<V: Into<Value>>(&mut self, value: V) {
        self.0.push(value.into());
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return f.write_str("Structure()");
        }
        let mut tuple = f.debug_tuple("Structure");
        self.0.iter().for_each(|v| {
            tuple.field(v);
        });
        tuple.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de::from_bytes;
    use crate::ser::to_bytes;

    #[test]
    fn value_test() {
        assert_eq!(Value::default(), Value::Null);
        let mut s: Structure = Structure::empty();
        s.push(Value::I64(10));
        assert_eq!(s, from_bytes::<Structure>(&to_bytes(&s).unwrap()).unwrap());
    }

    #[test]
    fn structure_test() {
        assert_eq!(Structure::empty(), Structure::empty());
    }
}
