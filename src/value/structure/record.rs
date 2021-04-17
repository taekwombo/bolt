use super::{BoltStructure, Single, Value};
use crate::{
    constants::STRUCTURE_NAME,
    error::{SerdeError, SerdeResult},
};
use serde::{
    de, forward_to_deserialize_any,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub fields: Vec<Value>,
}

impl BoltStructure for Record {
    const SIG: u8 = 0x71;
    const LEN: u8 = 0x01;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Single<Vec<Value>>;

    fn into_value(self) -> Value {
        value_map! {
            "fields" => Value::List(self.fields),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Record").field(&self.fields).finish()
    }
}

impl ser::Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.fields)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Record {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(RecordVisitor)
    }
}

struct RecordVisitor;

impl<'de> de::Visitor<'de> for RecordVisitor {
    type Value = Record;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Record")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let fields = structure_access!(map_access, Record);
        Ok(Record {
            fields: fields.value(),
        })
    }
}

impl<'de> de::Deserializer<'de> for Record {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.into_value().deserialize_map(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier enum ignored_any
    }
}
