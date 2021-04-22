use super::super::Value;
use crate::{
    constants::STRUCTURE_NAME,
    constants::structure,
    error::{PackstreamError, PackstreamResult},
    packstream::{PackstreamStructure, Single},
};
use serde::{
    de, forward_to_deserialize_any,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LocalTime {
    pub nanoseconds: i64,
}

impl PackstreamStructure for LocalTime {
    const SIG: u8 = structure::LOCAL_TIME;
    const LEN: u8 = 0x01;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Single<i64>;

    fn into_value(self) -> Value {
        value_map! {
            "nanoseconds" => Value::I64(self.nanoseconds),
        }
    }
}

impl fmt::Display for LocalTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LocalTime")
            .field("nanoseconds", &self.nanoseconds)
            .finish()
    }
}

impl ser::Serialize for LocalTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.nanoseconds)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for LocalTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(LocalTimeVisitor)
    }
}

struct LocalTimeVisitor;

impl<'de> de::Visitor<'de> for LocalTimeVisitor {
    type Value = LocalTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("LocalTime")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let nanoseconds = structure_access!(map_access, LocalTime);
        Ok(LocalTime { nanoseconds: nanoseconds.value() })
    }
}

impl<'de> de::Deserializer<'de> for LocalTime {
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

