use super::{BoltStructure, Empty, Value};
use crate::{
    constants::STRUCTURE_NAME,
    error::{PackstreamError, PackstreamResult},
};
use serde::{
    de, forward_to_deserialize_any,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Ignored;

impl BoltStructure for Ignored {
    const SIG: u8 = 0x7E;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Empty;

    fn into_value(self) -> Value {
        value_map! {}
    }
}

impl fmt::Display for Ignored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Ignored")
    }
}

impl ser::Serialize for Ignored {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?
            .end()
    }
}

impl<'de> de::Deserialize<'de> for Ignored {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(IgnoredVisitor)
    }
}

struct IgnoredVisitor;

impl<'de> de::Visitor<'de> for IgnoredVisitor {
    type Value = Ignored;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Ignored")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        structure_access!(map_access, Ignored);
        Ok(Ignored)
    }
}

impl<'de> de::Deserializer<'de> for Ignored {
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
