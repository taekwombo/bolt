use super::BoltStructure;
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct UnboundRelationship {
    pub identity: i64,
    pub r#type: String,
    pub properties: HashMap<String, Value>,
}

impl BoltStructure for UnboundRelationship {
    const SIG: u8 = 0x72;
    const LEN: u8 = 0x03;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (i64, String, HashMap<String, Value>);
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

#[cfg(test)]
mod test_unbound_relationship {
    use super::*;
    use crate::{
        constants::marker::{TINY_MAP, TINY_STRING, TINY_STRUCT},
        test,
    };

    const BYTES: &[u8] = &[
        TINY_STRUCT + UnboundRelationship::LEN,
        UnboundRelationship::SIG,
        0,
        TINY_STRING,
        TINY_MAP,
    ];

    #[test]
    fn bytes() {
        test::ser_de::<UnboundRelationship>(BYTES);
        test::de_ser(UnboundRelationship {
            identity: 0,
            r#type: String::new(),
            properties: HashMap::new(),
        });
        test::de_err::<UnboundRelationship>(&BYTES[0..(BYTES.len() - 1)]);
    }
}
