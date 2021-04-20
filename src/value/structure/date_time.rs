use super::super::Value;
use crate::{
    constants::STRUCTURE_NAME,
    constants::structure,
    error::{PackstreamError, PackstreamResult},
    packstream::PackstreamStructure,
};
use serde::{
    de, forward_to_deserialize_any,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct DateTime {
    pub seconds: i64,
    pub nanoseconds: i64,
    pub tz_offset_seconds: i64,
}

impl PackstreamStructure for DateTime {
    const SIG: u8 = structure::DATE_TIME;
    const LEN: u8 = 0x03;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (i64, i64, i64);

    fn into_value(self) -> Value {
        value_map! {
            "seconds" => Value::I64(self.seconds),
            "nanoseconds" => Value::I64(self.nanoseconds),
            "tz_offset_seconds" => Value::I64(self.tz_offset_seconds),
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DateTime")
            .field("seconds", &self.seconds)
            .field("nanoseconds", &self.nanoseconds)
            .field("tz_offset_seconds", &self.tz_offset_seconds)
            .finish()
    }
}

impl ser::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.seconds)?;
        ts_serializer.serialize_field(&self.nanoseconds)?;
        ts_serializer.serialize_field(&self.tz_offset_seconds)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(DateTimeVisitor)
    }
}

struct DateTimeVisitor;

impl<'de> de::Visitor<'de> for DateTimeVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("DateTime")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (seconds, nanoseconds, tz_offset_seconds) = structure_access!(map_access, DateTime);
        Ok(DateTime {
            seconds,
            nanoseconds,
            tz_offset_seconds,
        })
    }
}

impl<'de> de::Deserializer<'de> for DateTime {
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


