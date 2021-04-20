use crate::{
    constants::{message, STRUCTURE_NAME},
    error::{PackstreamError, PackstreamResult},
    packstream::{PackstreamStructure, Single},
    Value,
};
use serde::{
    de, forward_to_deserialize_any,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct Failure {
    pub metadata: HashMap<String, Value>,
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Failure").field(&self.metadata).finish()
    }
}

impl PackstreamStructure for Failure {
    const SIG: u8 = message::FAILURE;
    const LEN: u8 = 0x01;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Single<HashMap<String, Value>>;

    fn into_value(self) -> Value {
        value_map! {
            "metadata" => Value::Map(self.metadata),
        }
    }
}

impl ser::Serialize for Failure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.metadata)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Failure {
    fn deserialize<D>(deserializer: D) -> Result<Failure, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(FailureVisitor)
    }
}

struct FailureVisitor;

impl<'de> de::Visitor<'de> for FailureVisitor {
    type Value = Failure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Failure")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let fields = structure_access!(map_access, Failure);
        Ok(Failure {
            metadata: fields.value(),
        })
    }
}

impl<'de> de::Deserializer<'de> for Failure {
    type Error = PackstreamError;

    fn deserialize_any<V>(self, visitor: V) -> PackstreamResult<V::Value>
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
