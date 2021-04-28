use super::super::Value;
use super::super::display;
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
pub struct Relationship {
    pub id: i64,
    pub start_node_id: i64,
    pub end_node_id: i64,
    pub r#type: String,
    pub properties: HashMap<String, Value>,
}

impl PackstreamStructure for Relationship {
    const SIG: u8 = structure::RELATIONSHIP;
    const LEN: u8 = 0x05;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (i64, i64, i64, String, HashMap<String, Value>);

    fn into_value(self) -> Value {
        value_map! {
            "id" => Value::I64(self.id),
            "start_node_id" => Value::I64(self.start_node_id),
            "end_node_id" => Value::I64(self.end_node_id),
            "type" => Value::String(self.r#type),
            "properties" => Value::Map(self.properties),
        }
    }
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("[:{} ", self.r#type))?;

        display::display_value_hash_map(&self.properties, f)?;
        
        f.write_str("]")
    }
}

impl ser::Serialize for Relationship {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.id)?;
        ts_serializer.serialize_field(&self.start_node_id)?;
        ts_serializer.serialize_field(&self.end_node_id)?;
        ts_serializer.serialize_field(&self.r#type)?;
        ts_serializer.serialize_field(&self.properties)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Relationship {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(RelationshipVisitor)
    }
}

struct RelationshipVisitor;

impl<'de> de::Visitor<'de> for RelationshipVisitor {
    type Value = Relationship;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Relationship")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (id, start_node_id, end_node_id, r#type, properties) =
            structure_access!(map_access, Relationship);

        Ok(Relationship {
            id,
            start_node_id,
            end_node_id,
            r#type,
            properties,
        })
    }
}

impl<'de> de::Deserializer<'de> for Relationship {
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
