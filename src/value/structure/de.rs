use std::fmt;
use serde::de::{self, Error};
use crate::constants::STRUCTURE_SIG_KEY;
use super::*;
struct StructureVisitor;

impl<'de> de::Visitor<'de> for StructureVisitor {
    type Value = Structure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Structure")
    }
    
    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
        where V: de::MapAccess<'de>
    {
        check!(__key, map_access, STRUCTURE_SIG_KEY);

        match map_access.next_value::<u8>()? {
            AckFailure::SIG => {
                structure_access!(map_access, AckFailure, no_sig_key);
                Ok(Self::Value::from(AckFailure))
            },
            DiscardAll::SIG => {
                structure_access!(map_access, DiscardAll, no_sig_key);
                Ok(Self::Value::from(DiscardAll))
            },
            Failure::SIG => {
                let fields = structure_access!(map_access, Failure, no_sig_key);
                Ok(Self::Value::from(Failure { metadata: fields.value() }))

            },
            Ignored::SIG => {
                structure_access!(map_access, Ignored, no_sig_key);
                Ok(Self::Value::from(Ignored))
            },
            Init::SIG => {
                let (client, auth) = structure_access!(map_access, Init, no_sig_key);
                Ok(Self::Value::from(Init { client, auth }))
            },
            Node::SIG => {
                let (identity, labels, properties) = structure_access!(map_access, Node, no_sig_key);
                Ok(Self::Value::from(Node {
                    identity,
                    labels,
                    properties,
                }))
            },
            Path::SIG => {
                let (nodes, relationships, sequence) = structure_access!(map_access, Path, no_sig_key);
                Ok(Self::Value::from(Path {
                    nodes,
                    relationships,
                    sequence,
                }))
            },
            PullAll::SIG => {
                structure_access!(map_access, Path, no_sig_key);
                Ok(Self::Value::from(PullAll))
            },
            Record::SIG => {
                let fields = structure_access!(map_access, Record, no_sig_key);
                Ok(Self::Value::from(Record { fields: fields.value() }))
            },
            Relationship::SIG => {
                let (identity, start_node_identity, end_node_identity, r#type, properties) = structure_access!(map_access, Relationship, no_sig_key);
                Ok(Self::Value::from(Relationship {
                    identity,
                    start_node_identity,
                    end_node_identity,
                    r#type,
                    properties,
                }))
            },
            Reset::SIG => {
                structure_access!(map_access, Reset, no_sig_key);
                Ok(Self::Value::from(Reset))
            },
            Run::SIG => {
                let (statement, parameters) = structure_access!(map_access, Run, no_sig_key);
                Ok(Self::Value::from(Run {
                    statement,
                    parameters,
                }))
            },
            Success::SIG => {
                let fields = structure_access!(map_access, Success, no_sig_key);
                Ok(Self::Value::from(Success { metadata: fields.value() }))
            },
            UnboundRelationship::SIG => {
                let (identity, r#type, properties) = structure_access!(map_access, UnboundRelationship, no_sig_key);
                Ok(Self::Value::from(UnboundRelationship {
                    identity,
                    r#type,
                    properties,
                }))
            },
            signature @ _ => Err(V::Error::custom(format!(
                "Expected signature of a known Structure, got {}.",
                signature,
            ))),
        }
    }
}

impl<'de> de::Deserialize<'de> for Structure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        deserializer.deserialize_map(StructureVisitor)
    }
}

