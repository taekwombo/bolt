use std::collections::HashMap;
use std::fmt;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap, SerializeTupleStruct};
use super::marker_bytes::STRUCTURE_NAME;

// TODO: Make printing Value great again.
// TODO: Should Structure have Structure<T>(Vec<T>) signature?

#[derive(Debug)]
pub struct Structure(Vec<Value>);

impl Serialize for Structure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut tuple_struct = serializer.serialize_tuple_struct(STRUCTURE_NAME, self.0.len())?;
        for elem in self.0.iter() {
            tuple_struct.serialize_field(elem)?;
        }
        tuple_struct.end()
    }
}

impl Structure {
    fn new () -> Self {
        Self (Vec::new())
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tuple = f.debug_tuple("Structure");
        self.0.iter().for_each(|v| { tuple.field(v); });
        tuple.finish()
    }
}

#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Structure(Structure),
}

impl Serialize for Value {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::I64(v) => serializer.serialize_i64(*v),
            Value::F64(v) => serializer.serialize_f64(*v),
            Value::String(v) => serializer.serialize_str(&v),
            Value::List(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(&e)?;
                }
                seq.end()
            },
            Value::Map(v) => {
                let mut map = serializer.serialize_map(Some(v.len()))?;
                for (k, v) in v.iter() {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            },
            Value::Structure(v) => v.serialize(serializer),
        }
    }
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
            Self::Structure(v) => write!(f, "{}", v),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn value_test () {
//         let s = Structure(vec![
//             Value::Null,
//             Value::List(vec![
//                 Value::String("SIEMANKO".into()),
//                 Value::String("WITAM".into()),
//                 Value::String("W".into()),
//                 Value::String("MOJEJ".into()),
//                 Value::String("KUCHNI".into()),
//             ])
//         ]);
//         println!("SERIALIZED: {:?}", super::super::ser::to_bytes(&s).unwrap());
//     }
// }
