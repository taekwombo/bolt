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
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct UnboundRelationship {
    pub identity: i64,
    pub r#type: String,
    pub properties: HashMap<String, Value>,
}

impl PackstreamStructure for UnboundRelationship {
    const SIG: u8 = structure::UNBOUND_RELATIONSHIP;
    const LEN: u8 = 0x03;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (i64, String, HashMap<String, Value>);

    fn into_value(self) -> Value {
        value_map! {
            "identity" => Value::I64(self.identity),
            "type" => Value::String(self.r#type),
            "properties" => Value::Map(self.properties),
        }
    }
}

impl fmt::Display for UnboundRelationship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnboundRelationship")
            .field("identity", &self.identity)
            .field("type", &self.r#type)
            .field("properties", &self.properties)
            .finish()
    }
}

impl ser::Serialize for UnboundRelationship {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.identity)?;
        ts_serializer.serialize_field(&self.r#type)?;
        ts_serializer.serialize_field(&self.properties)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for UnboundRelationship {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(UnboundRelationshipVisitor)
    }
}

struct UnboundRelationshipVisitor;

impl<'de> de::Visitor<'de> for UnboundRelationshipVisitor {
    type Value = UnboundRelationship;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("UnboundRelationship")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (identity, r#type, properties) = structure_access!(map_access, UnboundRelationship);
        Ok(UnboundRelationship {
            identity,
            r#type,
            properties,
        })
    }
}

impl<'de> de::Deserializer<'de> for UnboundRelationship {
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
