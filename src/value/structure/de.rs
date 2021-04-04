use std::fmt;
use serde::de;
use super::*;

impl<'de> de::Deserialize<'de> for Structure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        deserializer.deserialize_map(StructureVisitor)
    }
}

struct StructureVisitor;

impl<'de> de::Visitor<'de> for StructureVisitor {
    type Value = Structure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Structure")
    }
    
    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
        where V: de::MapAccess<'de>
    {
        check!(__key, map_access, constants::STRUCTURE_SIG_KEY);

        match map_access.next_value::<u8>()? {
            AckFailure::SIG => {
                structure_access!(map_access, AckFailure, no_sig_key, fields(0));
                Ok(AckFailure)
            },
            DiscardAll::SIG => {
                structure_access!(map_access, DiscardAll, no_sig_key, fields(0));
                Ok(DiscardAll)
            },
            Failure::SIG => {
                let mut fields = structure_access!(map_access, Failure, no_sig_key, fields(1));
                Ok(Failure {
                    metadata: fields.pop().expect("Field to have one element")
                })

            },
            Ignored::SIG => {
                structure_access!(map_access, Ignored, no_sig_key, fields(0));
                Ok(Ignored)
            },
            Init::SIG => {
                let (client, auth) = structure_access!(map_access, Init, no_sig_key);
                Ok(Init { client, auth })
            },
            Node::SIG => {
                let (identity, labels, properties) = structure_access!(map_access, Node, no_sig_key);
                Ok(Node {
                    identity,
                    labels,
                    properties,
                })
            },
            Path::SIG => {
                let (nodes, relationships, sequence) = structure_access!(map_access, Path, no_sig_key);
                Ok(Path {
                    nodes,
                    relationships,
                    sequence,
                })
            },
            PullAll::SIG => {
                structure_access!(map_access, Path, no_sig_key, fields(0));
                Ok(PullAll)
            },
            Record::SIG => {
                let mut fields = structure_access!(map_access, Record, no_sig_key, fields(1));
                Ok(Record {
                    fields: fields.pop().expect("Fields to have one element")
                })
            },
            Relationship::SIG => {
                let (identity, start_node_identity, end_node_identity, r#type, properties) = structure_access!(map_access, Relationship, no_sig_key);
                Ok(Relationship {
                    identity,
                    start_node_identity,
                    end_node_identity,
                    r#type,
                    properties,
                })
            },
            Reset::SIG => {
                structure_access!(map_access, Reset, no_sig_key, fields(0));
                Ok(Reset)
            },
            Run::SIG => {
                let (statement, parameters) = structure_access!(map_access, Run, no_sig_key);
                Ok(Run {
                    statement,
                    parameters,
                })
            },
            Success::SIG => {
                let fields = structure_access!(map_access, Success, no_sig_key, fields(1));
                Ok(Success {
                    metadata: fields.pop().expect("Fields to have one element")
                })
            },
            UnboundRelationship::SIG => {
                let (identity, r#type, properties) = structure_access!(map_access, UnboundRelationship, no_sig_key);
                Ok(UnboundRelationship {
                    identity,
                    r#type,
                    properties,
                })
            },
            signature @ _ => Err(V::Error::custom(format!(
                "Expected signature of a known Structure, got {}.",
                signature,
            ))),
        }
    }
}
