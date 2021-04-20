use super::*;
use crate::{
    constants::STRUCTURE_SIG_KEY,
    error::{PackstreamError, PackstreamResult},
};
use serde::{de, forward_to_deserialize_any};
use std::fmt;
struct StructureVisitor;

impl<'de> de::Visitor<'de> for StructureVisitor {
    type Value = Structure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Structure")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        check!(__key, map_access, STRUCTURE_SIG_KEY);
        Structure::from_map_access_no_sig_key(&mut map_access)
    }
}

impl<'de> de::Deserialize<'de> for Structure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(StructureVisitor)
    }
}

impl<'de> de::Deserializer<'de> for Structure {
    type Error = PackstreamError;

    fn deserialize_any<V>(self, visitor: V) -> PackstreamResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Node(de) => de.deserialize_any(visitor),
            Self::Path(de) => de.deserialize_any(visitor),
            Self::Relationship(de) => de.deserialize_any(visitor),
            Self::UnboundRelationship(de) => de.deserialize_any(visitor),
            Self::Date(de) => de.deserialize_any(visitor),
            Self::Time(de) => de.deserialize_any(visitor),
            Self::LocalTime(de) => de.deserialize_any(visitor),
            Self::DateTime(de) => de.deserialize_any(visitor),
            Self::DateTimeZoneId(de) => de.deserialize_any(visitor),
            Self::LocalDateTime(de) => de.deserialize_any(visitor),
            Self::Duration(de) => de.deserialize_any(visitor),
            Self::Point2D(de) => de.deserialize_any(visitor),
            Self::Point3D(de) => de.deserialize_any(visitor),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
