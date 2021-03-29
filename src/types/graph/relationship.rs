use crate::{
    constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY},
    Value,
};
use serde::{
    de::{self, Error},
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

const MSG_RELATIONSHIP_SIGNATURE: u8 = 0x52;
const MSG_RELATIONSHIP_LENGTH: u8 = 0x05;
const MSG_RELATIONSHIP_SERIALIZE_LENGTH: usize =
    serialize_length!(MSG_RELATIONSHIP_SIGNATURE, MSG_RELATIONSHIP_LENGTH);

#[derive(Debug, PartialEq)]
pub struct Relationship {
    pub properties: HashMap<String, Value>,
    pub identity: i64,
    pub start_node_identity: i64,
    pub end_node_identity: i64,
    pub r#type: String,
    pub properties: HashMap<String, Value>,
}

impl ser::Serialize for Relationship {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, MSG_RELATIONSHIP_SERIALIZE_LENGTH)?;
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
        formatter.write_str("Relationship type")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_RELATIONSHIP_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                });
                let fields: (i64, i64, i64, String, HashMap<String, Value>) =
                    map_access.next_value()?;
                access_check!(map_access, {
                    key(),
                });
                Ok(Relationship {
                    identity: fields.0,
                    start_node_identity: fields.1,
                    end_node_identity: fields.2,
                    r#type: fields.3,
                    properties: fields.4,
                })
            }
            Some(key) => unexpected_key_access!(key),
            None => unexpected_key_access!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, from_bytes, to_bytes};

    const BYTES: &'static [u8] = &[181, 82, 100, 101, 102, 132, 110, 111, 100, 101, 160];

    #[test]
    fn serialize() {
        let result = to_bytes(&Relationship {
            identity: 100,
            start_node_identity: 101,
            end_node_identity: 102,
            r#type: String::from("node"),
            properties: HashMap::new(),
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<Relationship>(BYTES);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Relationship {
                identity: 100,
                start_node_identity: 101,
                end_node_identity: 102,
                r#type: String::from("node"),
                properties: HashMap::new(),
            }
        );
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<Relationship>(&BYTES[0..(BYTES.len() - 1)]);
        assert!(result.is_err());
    }
}
