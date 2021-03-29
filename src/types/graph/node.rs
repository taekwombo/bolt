use crate::{
    constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY},
    Value,
};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

const MSG_NODE_SIGNATURE: u8 = 0x4E;
const MSG_NODE_LENGTH: u8 = 0x03;
const MSG_NODE_SERIALIZE_LENGTH: usize = serialize_length!(MSG_NODE_SIGNATURE, MSG_NODE_LENGTH);

#[derive(Debug, PartialEq)]
pub struct Node {
    pub identity: i64,
    pub labels: Vec<String>,
    pub properties: HashMap<String, Value>,
}

impl ser::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, MSG_NODE_SERIALIZE_LENGTH)?;
        ts_serializer.serialize_field(&self.identity)?;
        ts_serializer.serialize_field(&self.labels)?;
        ts_serializer.serialize_field(&self.properties)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(NodeVisitor)
    }
}

struct NodeVisitor;

impl<'de> de::Visitor<'de> for NodeVisitor {
    type Value = Node;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Node type")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_NODE_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                });
                let fields: (i64, Vec<String>, HashMap<String, Value>) = map_access.next_value()?;
                access_check!(map_access, {
                    key(),
                });
                Ok(Node {
                    identity: fields.0,
                    labels: fields.1,
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
    use crate::{constants::marker::TINY_STRUCT, from_bytes, to_bytes};

    const BYTES: &'static [u8] = &[179, 78, 100, 145, 132, 110, 111, 100, 101, 160];

    #[test]
    fn serialize() {
        let result = to_bytes(&Node {
            identity: 100,
            labels: vec![String::from("node")],
            properties: HashMap::new(),
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<Node>(BYTES);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Node {
                identity: 100,
                labels: vec![String::from("node")],
                properties: HashMap::new(),
            }
        );
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<Node>(&BYTES[0..(BYTES.len() - 1)]);
        assert!(result.is_err());
    }
}
