use super::{BoltStructure, Value};
use crate::{
    constants::STRUCTURE_NAME,
    error::{SerdeError, SerdeResult},
};
use serde::{
    de, forward_to_deserialize_any,
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

    fn into_value(self) -> Value {
        value_map! {
            "identity" => Value::I64(self.identity),
            "labels" => Value::List(self.labels.into_iter().map(Value::String).collect()),
            "properties" => Value::Map(self.properties),
        }
    }
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

impl<'de> de::Deserializer<'de> for Node {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
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

#[cfg(test)]
mod test_node {
    use super::*;
    use crate::{
        constants::marker::{TINY_LIST, TINY_MAP, TINY_STRUCT},
        test,
    };

    const BYTES: &[u8] = &[TINY_STRUCT + Node::LEN, Node::SIG, 0, TINY_LIST, TINY_MAP];

    #[test]
    fn bytes() {
        test::ser_de::<Node>(BYTES);
        test::de_ser(Node {
            identity: 0,
            labels: Vec::new(),
            properties: HashMap::new(),
        });
        test::de_err::<Node>(&BYTES[0..(BYTES.len() - 1)]);
    }
}
