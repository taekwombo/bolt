use super::BoltStructure;
use super::{Node, UnboundRelationship};
use crate::constants::STRUCTURE_NAME;
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Path {
    pub nodes: Vec<Node>,
    pub relationships: Vec<UnboundRelationship>,
    pub sequence: Vec<i64>,
}

impl BoltStructure for Path {
    const SIG: u8 = 0x50;
    const LEN: u8 = 0x03;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (Vec<Node>, Vec<UnboundRelationship>, Vec<i64>);
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Path")
            .field("nodes", &self.nodes)
            .field("relationships", &self.relationships)
            .field("sequence", &self.sequence)
            .finish()
    }
}

impl ser::Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
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
        formatter.write_str("Path")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (nodes, relationships, sequence) = structure_access!(map_access, Path);
        Ok(Path {
            nodes,
            relationships,
            sequence,
        })
    }
}

#[cfg(test)]
mod test_path {
    use super::*;
    use crate::{
        constants::marker::{TINY_LIST, TINY_STRUCT},
        test,
    };

    const BYTES: &[u8] = &[
        TINY_STRUCT + Path::LEN,
        Path::SIG,
        TINY_LIST,
        TINY_LIST,
        TINY_LIST,
    ];

    #[test]
    fn bytes() {
        test::ser_de::<Path>(BYTES);
        test::de_ser(Path {
            nodes: Vec::new(),
            relationships: Vec::new(),
            sequence: Vec::new(),
        });
        test::de_err::<Path>(&BYTES[0..(BYTES.len() - 1)]);
    }
}
