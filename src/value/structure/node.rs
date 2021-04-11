use super::BoltStructure;
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct Node {
    pub identity: i64,
    pub labels: Vec<String>,
    pub properties: HashMap<String, Value>,
}

impl BoltStructure for Node {
    const SIG: u8 = 0x4E;
    const LEN: u8 = 0x03;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (i64, Vec<String>, HashMap<String, Value>);
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
            .field("identity", &self.identity)
            .field("labels", &self.labels)
            .field("properties", &self.properties)
            .finish()
    }
}

impl ser::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
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
        formatter.write_str("Node")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (identity, labels, properties) = structure_access!(map_access, Node);
        Ok(Node {
            identity,
            labels,
            properties,
        })
    }
}

#[cfg(test)]
mod test_node {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, from_bytes, test, to_bytes};

    const BYTES: &[u8] = &[179, 78, 100, 145, 132, 110, 111, 100, 101, 160];

    fn create_node() -> Node {
        Node {
            identity: 100,
            labels: vec![String::from("node")],
            properties: HashMap::new(),
        }
    }

    #[test]
    fn serialize() {
        test::ser(&create_node(), BYTES);
    }

    #[test]
    fn deserialize() {
        test::de(&create_node(), BYTES);
    }

    #[test]
    fn deserialize_fail() {
        test::de_err::<Node>(&BYTES[0..(BYTES.len() - 1)]);
        test::de_err::<Node>(&[TINY_STRUCT, Node::SIG + 1]);
    }
}
