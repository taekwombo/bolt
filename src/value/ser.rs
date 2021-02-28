use super::Value;
use crate::constants::STRUCTURE_NAME;
use serde::ser::{self, SerializeMap, SerializeSeq, SerializeTupleStruct};

impl ser::Serialize for Value {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_unit(),
            Self::Bool(v) => serializer.serialize_bool(*v),
            Self::I64(v) => serializer.serialize_i64(*v),
            Self::F64(v) => serializer.serialize_f64(*v),
            Self::String(v) => serializer.serialize_str(&v),
            Self::List(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(&e)?;
                }
                seq.end()
            }
            Self::Map(v) => {
                let mut map = serializer.serialize_map(Some(v.len()))?;
                for (k, v) in v.iter() {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Self::Bytes(v) => serializer.serialize_bytes(&*v),
            Self::Structure { signature, fields } => {
                let mut tuple = serializer.serialize_tuple_struct(STRUCTURE_NAME, fields.len())?;
                tuple.serialize_field(signature)?;
                for elem in fields.iter() {
                    tuple.serialize_field(elem)?;
                }
                tuple.end()
            }
        }
    }
}
