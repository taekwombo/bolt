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
pub struct DateTimeZoneId {
    pub seconds: i64,
    pub nanoseconds: i64,
    pub tz_id: String,
}

impl PackstreamStructure for DateTimeZoneId {
    const SIG: u8 = structure::DATE_TIME_ZONE_ID;
    const LEN: u8 = 0x03;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (i64, i64, String);

    fn into_value(self) -> Value {
        value_map! {
            "seconds" => Value::I64(self.seconds),
            "nanoseconds" => Value::I64(self.nanoseconds),
            "tz_id" => Value::String(self.tz_id),
        }
    }
}

impl fmt::Display for DateTimeZoneId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DateTimeZoneId")
            .field("seconds", &self.seconds)
            .field("nanoseconds", &self.nanoseconds)
            .field("tz_id", &self.tz_id)
            .finish()
    }
}

impl ser::Serialize for DateTimeZoneId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.seconds)?;
        ts_serializer.serialize_field(&self.nanoseconds)?;
        ts_serializer.serialize_field(&self.tz_id)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for DateTimeZoneId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(DateTimeZoneIdVisitor)
    }
}

struct DateTimeZoneIdVisitor;

impl<'de> de::Visitor<'de> for DateTimeZoneIdVisitor {
    type Value = DateTimeZoneId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("DateTimeZoneId")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (seconds, nanoseconds, tz_id) = structure_access!(map_access, DateTimeZoneId);
        Ok(DateTimeZoneId {
            seconds,
            nanoseconds,
            tz_id,
        })
    }
}

impl<'de> de::Deserializer<'de> for DateTimeZoneId {
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



