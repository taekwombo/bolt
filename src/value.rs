mod structure;
mod de;
mod ser;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
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
            // Self::Structure(v) => write!(f, "{}", v),
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
//     use super::super::marker_bytes::*;

//     #[test]
//     fn build_value () {
//         let s = Structure(1, vec![
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
//         let s: Structure = super::super::de::from_bytes(&[TINY_STRUCT + 2, 1, 1, 1]).unwrap();
//         println!("DESERIALIZED: {:?}", s);
//     }
// }
