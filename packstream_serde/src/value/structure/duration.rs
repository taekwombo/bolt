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
pub struct Duration {
    pub months: i64,
    pub days: i64,
    pub seconds: i64,
    pub nanoseconds: i64,
}

impl PackstreamStructure for Duration {
    const SIG: u8 = structure::DURATION;
    const LEN: u8 = 0x04;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (i64, i64, i64, i64);

    fn into_value(self) -> Value {
        value_map! {
            "months" => Value::I64(self.months),
            "days" => Value::I64(self.days),
            "seconds" => Value::I64(self.seconds),
            "nanoseconds" => Value::I64(self.nanoseconds),
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Duration")
            .field("months", &self.months)
            .field("days", &self.days)
            .field("seconds", &self.seconds)
            .field("nanoseconds", &self.nanoseconds)
            .finish()
    }
}

impl ser::Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.months)?;
        ts_serializer.serialize_field(&self.days)?;
        ts_serializer.serialize_field(&self.seconds)?;
        ts_serializer.serialize_field(&self.nanoseconds)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(DurationVisitor)
    }
}

struct DurationVisitor;

impl<'de> de::Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Duration")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (months, days, seconds, nanoseconds) = structure_access!(map_access, Duration);
        Ok(Duration {
            months,
            days,
            seconds,
            nanoseconds,
        })
    }
}

impl<'de> de::Deserializer<'de> for Duration {
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



