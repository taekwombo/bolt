use super::super::Value;
use crate::{
    constants::{structure, STRUCTURE_NAME},
    error::{PackstreamError, PackstreamResult},
    packstream::PackstreamStructure,
};
use serde::{
    de, forward_to_deserialize_any,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Point2D {
    pub srid: i64,
    pub x: f64,
    pub y: f64,
}

impl PackstreamStructure for Point2D {
    const SIG: u8 = structure::POINT_2D;
    const LEN: u8 = 0x03;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (i64, f64, f64);

    fn into_value(self) -> Value {
        value_map! {
            "srid" => Value::I64(self.srid),
            "x" => Value::F64(self.x),
            "y" => Value::F64(self.y),
        }
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Point2D")
            .field("srid", &self.srid)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl ser::Serialize for Point2D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.srid)?;
        ts_serializer.serialize_field(&self.x)?;
        ts_serializer.serialize_field(&self.y)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Point2D {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(Point2DVisitor)
    }
}

struct Point2DVisitor;

impl<'de> de::Visitor<'de> for Point2DVisitor {
    type Value = Point2D;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Point2D")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (srid, x, y) = structure_access!(map_access, Point2D);
        Ok(Point2D {
            srid,
            x,
            y,
        })
    }
}

impl<'de> de::Deserializer<'de> for Point2D {
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

