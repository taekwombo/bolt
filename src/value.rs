mod de;
mod ser;
pub use de::from_value;

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
    Bytes(ByteBuf),
    Structure { signature: u8, fields: Vec<Value> },
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
            Self::Structure { signature, .. } => {
                f.debug_tuple("Structure").field(signature).finish()
            }
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
    use super::Value;
    use crate::constants::marker::*;
    use serde_bytes::ByteBuf;
    use std::collections::HashMap;

    fn buf(capacity: usize) -> ByteBuf {
        ByteBuf::with_capacity(capacity)
    }

    fn structure() -> Value {
        Value::Structure {
            signature: 100,
            fields: vec![],
        }
    }

    #[test]
    fn value_serde() {
        assert_ser_de! {
            Value::default(),
            Value::Null,
            Value::Bool(true),
            Value::Bool(false),
            Value::I64(0),
            Value::F64(0.0),
            Value::String(String::from("")),
            Value::List(vec![]),
            Value::Bytes(ByteBuf::new()),
            Value::Map(HashMap::new()),
            Value::Structure { signature: 0, fields: vec![] },
            Value::Structure {
                signature: 0,
                fields: vec![
                    Value::Bool(true),
                    Value::List(vec![Value::Null, Value::I64(1000)]),
                    Value::Map(HashMap::new()),
                ],
            },
        };

        assert_ser! {
            ok {
                Value::Null => [NULL],
                Value::Bool(true) => [TRUE],
                Value::Bool(false) => [FALSE],
                Value::List(vec![]) => [TINY_LIST],
                Value::Map(HashMap::new()) => [TINY_MAP],
                Value::String(String::from("")) => [TINY_STRING],
                Value::I64(0i64) => [0],
                Value::F64(0.0) => [FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0],
                Value::Bytes(ByteBuf::new()) => [BYTES_8, 0],
                Value::Structure { signature: 0, fields: vec![] } => [TINY_STRUCT, 0],
            }
            err {}
        };

        assert_de! {
            ok with from_bytes into Value {
                &[NULL] => Value::Null,
                &[127] => Value::I64(127),
                &[TINY_STRING + 1, 49] => Value::String(String::from("1")),
                &[FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0] => Value::F64(0.0),
                &[TINY_LIST + 3, 0, 1, 0] =>  Value::List(vec![Value::I64(0), Value::I64(1), Value::I64(0)]),
                &[TINY_MAP] =>  Value::Map(HashMap::new()),
                &[BYTES_8, 1, 1] =>  Value::Bytes({ let mut b = buf(1); b.push(1); b }),
            }
            ok with from_value into String {
                Value::String(String::new()) => String::new(),
                Value::String("123".to_owned()) => "123".to_owned(),
            }
        };
    }

    #[test]
    fn structure_into_map() {
        #[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
        struct S {
            signature: u8,
            fields: Vec<Value>,
        }

        let m: S = crate::from_value(structure()).unwrap();
        println!("{:?}", m);

        let m: HashMap<String, Value> = crate::from_value(Value::Map(map! {})).unwrap();
        println!("{:?}", m);
    }
}
