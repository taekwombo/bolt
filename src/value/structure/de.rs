use super::*;
use crate::{
    constants::STRUCTURE_SIG_KEY,
    error::{SerdeError, SerdeResult},
};
use serde::{
    de::{self, Error},
    forward_to_deserialize_any,
};
use std::fmt;
struct StructureVisitor;

impl<'de> de::Visitor<'de> for StructureVisitor {
    type Value = Structure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Structure")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        check!(__key, map_access, STRUCTURE_SIG_KEY);

        match map_access.next_value::<u8>()? {
            AckFailure::SIG => {
                structure_access!(map_access, AckFailure, no_sig_key);
                Ok(Self::Value::from(AckFailure))
            }
            DiscardAll::SIG => {
                structure_access!(map_access, DiscardAll, no_sig_key);
                Ok(Self::Value::from(DiscardAll))
            }
            Failure::SIG => {
                let fields = structure_access!(map_access, Failure, no_sig_key);
                Ok(Self::Value::from(Failure {
                    metadata: fields.value(),
                }))
            }
            Ignored::SIG => {
                structure_access!(map_access, Ignored, no_sig_key);
                Ok(Self::Value::from(Ignored))
            }
            Init::SIG => {
                let (client, auth) = structure_access!(map_access, Init, no_sig_key);
                Ok(Self::Value::from(Init { client, auth }))
            }
            Node::SIG => {
                let (identity, labels, properties) =
                    structure_access!(map_access, Node, no_sig_key);
                Ok(Self::Value::from(Node {
                    identity,
                    labels,
                    properties,
                }))
            }
            Path::SIG => {
                let (nodes, relationships, sequence) =
                    structure_access!(map_access, Path, no_sig_key);
                Ok(Self::Value::from(Path {
                    nodes,
                    relationships,
                    sequence,
                }))
            }
            PullAll::SIG => {
                structure_access!(map_access, Path, no_sig_key);
                Ok(Self::Value::from(PullAll))
            }
            Record::SIG => {
                let fields = structure_access!(map_access, Record, no_sig_key);
                Ok(Self::Value::from(Record {
                    fields: fields.value(),
                }))
            }
            Relationship::SIG => {
                let (identity, start_node_identity, end_node_identity, r#type, properties) =
                    structure_access!(map_access, Relationship, no_sig_key);
                Ok(Self::Value::from(Relationship {
                    identity,
                    start_node_identity,
                    end_node_identity,
                    r#type,
                    properties,
                }))
            }
            Reset::SIG => {
                structure_access!(map_access, Reset, no_sig_key);
                Ok(Self::Value::from(Reset))
            }
            Run::SIG => {
                let (statement, parameters) = structure_access!(map_access, Run, no_sig_key);
                Ok(Self::Value::from(Run {
                    statement,
                    parameters,
                }))
            }
            Success::SIG => {
                let fields = structure_access!(map_access, Success, no_sig_key);
                Ok(Self::Value::from(Success {
                    metadata: fields.value(),
                }))
            }
            UnboundRelationship::SIG => {
                let (identity, r#type, properties) =
                    structure_access!(map_access, UnboundRelationship, no_sig_key);
                Ok(Self::Value::from(UnboundRelationship {
                    identity,
                    r#type,
                    properties,
                }))
            }
            signature => Err(V::Error::custom(format!(
                "Expected signature of a known Structure, got {}",
                signature,
            ))),
        }
    }
}

impl<'de> de::Deserialize<'de> for Structure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(StructureVisitor)
    }
}

impl<'de> de::Deserializer<'de> for Structure {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Node(de) => de.deserialize_any(visitor),
            Self::Path(de) => de.deserialize_any(visitor),
            Self::Relationship(de) => de.deserialize_any(visitor),
            Self::UnboundRelationship(de) => de.deserialize_any(visitor),
            Self::AckFailure(de) => de.deserialize_any(visitor),
            Self::DiscardAll(de) => de.deserialize_any(visitor),
            Self::Failure(de) => de.deserialize_any(visitor),
            Self::Ignored(de) => de.deserialize_any(visitor),
            Self::Init(de) => de.deserialize_any(visitor),
            Self::PullAll(de) => de.deserialize_any(visitor),
            Self::Record(de) => de.deserialize_any(visitor),
            Self::Reset(de) => de.deserialize_any(visitor),
            Self::Run(de) => de.deserialize_any(visitor),
            Self::Success(de) => de.deserialize_any(visitor),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
