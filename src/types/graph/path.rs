use super::{Node, UnboundRelationship};
use crate::{
    constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY},
    Value,
};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

const MSG_PATH_SIGNATURE: u8 = 0x50;
const MSG_PATH_LEGNTH: u8 = 0x03;
const MSG_PATH_SERIALIZE_LENGTH: usize = serialize_length!(MSG_PATH_SIGNATURE, MSG_PATH_LEGNTH);

#[derive(Debug, PartialEq)]
pub struct Path {
    pub nodes: Vec<Node>,
    pub relationships: Vec<UnboundRelationship>,
    pub sequence: Vec<i64>,
}

impl ser::Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, MSG_PATH_SERIALIZE_LENGTH)?;
        ts_serializer.serialize_field(&self.nodes)?;
        ts_serializer.serialize_field(&self.relationships)?;
        ts_serializer.serialize_field(&self.sequence)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(PathVisitor)
    }
}

struct PathVisitor;

impl<'de> de::Visitor<'de> for PathVisitor {
    type Value = Path;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Path type")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_PATH_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                });
                let fields: (Vec<Node>, Vec<UnboundRelationship>, Vec<i64>) =
                    map_access.next_value()?;
                access_check!(map_access, {
                    key(),
                });
                Ok(Path {
                    nodes: fields.0,
                    relationships: fields.1,
                    sequence: fields.2,
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
    use std::collections::HashMap;

    const BYTES: &'static [u8] = &[
        179, 80, 145, 179, 78, 1, 145, 132, 78, 111, 100, 101, 160, 145, 179, 82, 201, 0, 200, 132,
        84, 121, 112, 101, 160, 145, 100,
    ];

    fn get_path() -> Path {
        Path {
            nodes: vec![Node {
                identity: 1,
                labels: vec![String::from("Node")],
                properties: HashMap::new(),
            }],
            relationships: vec![UnboundRelationship {
                identity: 200,
                r#type: String::from("Type"),
                properties: HashMap::new(),
            }],
            sequence: vec![100i64],
        }
    }

    #[test]
    fn serialize() {
        let result = to_bytes(&get_path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<Path>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), get_path());
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<Path>(&BYTES[0..(BYTES.len() - 1)]);
        assert!(result.is_err());
    }
}
