use crate::{
    constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY},
    Value,
};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

const MSG_UNBOUND_RELATIONSHIP_SIGNATURE: u8 = 0x52;
const MSG_UNBOUND_RELATIONSHIP_LENGTH: u8 = 0x03;
const MSG_UNBOUND_RELATIONSHIP_SERIALIZE_LENGTH: usize = serialize_length!(
    MSG_UNBOUND_RELATIONSHIP_SIGNATURE,
    MSG_UNBOUND_RELATIONSHIP_LENGTH
);

#[derive(Debug, PartialEq)]
pub struct UnboundRelationship {
    pub identity: i64,
    pub r#type: String,
    pub properties: HashMap<String, Value>,
}

impl ser::Serialize for UnboundRelationship {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer = serializer
            .serialize_tuple_struct(STRUCTURE_NAME, MSG_UNBOUND_RELATIONSHIP_SERIALIZE_LENGTH)?;
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
        formatter.write_str("UnboundRelationship type")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_UNBOUND_RELATIONSHIP_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                });
                let fields: (i64, String, HashMap<String, Value>) = map_access.next_value()?;
                access_check!(map_access, {
                    key(),
                });
                Ok(UnboundRelationship {
                    identity: fields.0,
                    r#type: fields.1,
                    properties: fields.2,
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
    use crate::{from_bytes, to_bytes};

    const BYTES: &[u8] = &[179, 82, 100, 132, 110, 111, 100, 101, 160];

    #[test]
    fn serialize() {
        let result = to_bytes(&UnboundRelationship {
            identity: 100,
            r#type: String::from("node"),
            properties: HashMap::new(),
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<UnboundRelationship>(BYTES);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UnboundRelationship {
                identity: 100,
                r#type: String::from("node"),
                properties: HashMap::new(),
            }
        );
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<UnboundRelationship>(&BYTES[0..(BYTES.len() - 1)]);
        assert!(result.is_err());
    }
}
