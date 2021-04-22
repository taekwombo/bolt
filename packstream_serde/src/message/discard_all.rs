use crate::{
    constants::{marker, message, STRUCTURE_NAME},
    error::{PackstreamError, PackstreamResult},
    packstream::{PackstreamStructure, Empty, EmptyPackstreamStructure},
    Value,
};
use serde::{
    de, forward_to_deserialize_any,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct DiscardAll;

impl PackstreamStructure for DiscardAll {
    const SIG: u8 = message::DISCARD_ALL;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Empty;

    fn into_value(self) -> Value {
        value_map! {}
    }
}

impl EmptyPackstreamStructure for DiscardAll {
    const MSG: [u8; 2] = [marker::TINY_STRUCT, Self::SIG];
}

impl fmt::Display for DiscardAll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DiscardAll")
    }
}

impl ser::Serialize for DiscardAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?
            .end()
    }
}

struct DiscardAllVisitor;

impl<'de> de::Visitor<'de> for DiscardAllVisitor {
    type Value = DiscardAll;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("DiscardAll")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        structure_access!(map_access, DiscardAll);
        Ok(DiscardAll)
    }
}

impl<'de> de::Deserialize<'de> for DiscardAll {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(DiscardAllVisitor)
    }
}

impl<'de> de::Deserializer<'de> for DiscardAll {
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
