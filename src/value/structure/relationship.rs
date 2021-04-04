use super::super::BoltStructure;
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct Relationship {
    pub identity: i64,
    pub start_node_identity: i64,
    pub end_node_identity: i64,
    pub r#type: String,
    pub properties: HashMap<String, Value>,
}

impl BoltStructure for Relationship {
    const SIG: u8 = 0x52;
    const LEN: u8 = 0x05;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (i64, i64, i64, String, HashMap<String, Value>);
}

impl ser::Serialize for Relationship {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.identity)?;
        ts_serializer.serialize_field(&self.start_node_identity)?;
        ts_serializer.serialize_field(&self.end_node_identity)?;
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
        let (identity, start_node_identity, end_node_identity, r#type, properties) =
            structure_access!(map_access, Relationship);

        Ok(Relationship {
            identity,
            start_node_identity,
            end_node_identity,
            r#type,
            properties,
        })
    }
}

#[cfg(test)]
mod test_relationship {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, from_bytes, test, to_bytes};

    const BYTES: &[u8] = &[181, 82, 100, 101, 102, 132, 110, 111, 100, 101, 160];

    fn create_relationship() -> Relationship {
        Relationship {
            identity: 100,
            start_node_identity: 101,
            end_node_identity: 102,
            r#type: String::from("node"),
            properties: HashMap::new(),
        }
    }
    #[test]
    fn serialize() {
        test::ser(&create_relationship(), BYTES);
    }

    #[test]
    fn deserialize() {
        test::de(&create_relationship(), BYTES);
    }

    #[test]
    fn deserialize_fail() {
        test::de_err::<Relationship>(&BYTES[0..(BYTES.len() - 1)]);
        test::de_err::<Relationship>(&[TINY_STRUCT, Relationship::SIG + 1]);
    }
}
