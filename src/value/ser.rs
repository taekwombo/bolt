use super::{Structure, Value};
use crate::marker_bytes::STRUCTURE_NAME;
use serde::ser::{self, SerializeMap, SerializeSeq, SerializeTupleStruct};

impl ser::Serialize for Value {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
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
            }
            Value::Map(v) => {
                let mut map = serializer.serialize_map(Some(v.len()))?;
                for (k, v) in v.iter() {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Value::Bytes(v) => serializer.serialize_bytes(&*v),
        }
    }
}

impl ser::Serialize for Structure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let len = if self.0.is_empty() {
            0
        } else {
            self.0.len() - 1
        };

        let mut tuple_struct = serializer.serialize_tuple_struct(STRUCTURE_NAME, len)?;
        for elem in self.0.iter() {
            tuple_struct.serialize_field(elem)?;
        }
        tuple_struct.end()
    }
}
