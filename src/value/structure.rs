use serde::de::Error;
use std::fmt;

mod de;
mod ser;

mod ack_failure;
mod discard_all;
mod failure;
mod fields;
mod ignored;
mod init;
mod node;
mod path;
mod pull_all;
mod record;
mod relationship;
mod reset;
mod run;
mod success;
mod unbound_relationship;

use super::Value;
pub use ack_failure::AckFailure;
pub use discard_all::DiscardAll;
pub use failure::Failure;
pub use fields::{Empty, Single};
pub use ignored::Ignored;
pub use init::Init;
pub use node::Node;
pub use path::Path;
pub use pull_all::PullAll;
pub use record::Record;
pub use relationship::Relationship;
pub use reset::Reset;
pub use run::Run;
pub use success::Success;
pub use unbound_relationship::UnboundRelationship;

pub trait BoltStructure {
    const SIG: u8;
    const LEN: u8;
    const SERIALIZE_LEN: usize;

    type Fields;

    fn into_value(self) -> Value;
}

#[derive(Debug, PartialEq)]
pub enum Structure {
    Node(Node),
    Path(Path),
    Relationship(Relationship),
    UnboundRelationship(UnboundRelationship),
    AckFailure(AckFailure),
    DiscardAll(DiscardAll),
    Failure(Failure),
    Ignored(Ignored),
    Init(Init),
    PullAll(PullAll),
    Record(Record),
    Reset(Reset),
    Run(Run),
    Success(Success),
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Node(v) => f.debug_tuple("Node").field(v).finish(),
            Self::Path(v) => f.debug_tuple("Path").field(v).finish(),
            Self::Relationship(v) => f.debug_tuple("Relationship").field(v).finish(),
            Self::UnboundRelationship(v) => f.debug_tuple("UnboundRelationship").field(v).finish(),
            Self::AckFailure(v) => f.debug_tuple("AckFailure").field(v).finish(),
            Self::DiscardAll(v) => f.debug_tuple("DiscardAll").field(v).finish(),
            Self::Failure(v) => f.debug_tuple("Failure").field(v).finish(),
            Self::Ignored(v) => f.debug_tuple("Ignored").field(v).finish(),
            Self::Init(v) => f.debug_tuple("Init").field(v).finish(),
            Self::PullAll(v) => f.debug_tuple("PullAll").field(v).finish(),
            Self::Record(v) => f.debug_tuple("Record").field(v).finish(),
            Self::Reset(v) => f.debug_tuple("Reset").field(v).finish(),
            Self::Run(v) => f.debug_tuple("Run").field(v).finish(),
            Self::Success(v) => f.debug_tuple("Success").field(v).finish(),
        }
    }
}

impl Structure {
    pub(crate) fn from_map_access_no_sig_key<'de, V>(map_access: &mut V) -> Result<Self, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        match map_access.next_value::<u8>()? {
            AckFailure::SIG => {
                structure_access!(map_access, AckFailure, no_sig_key);
                Ok(Self::from(AckFailure))
            }
            DiscardAll::SIG => {
                structure_access!(map_access, DiscardAll, no_sig_key);
                Ok(Self::from(DiscardAll))
            }
            Failure::SIG => {
                let fields = structure_access!(map_access, Failure, no_sig_key);
                Ok(Self::from(Failure {
                    metadata: fields.value(),
                }))
            }
            Ignored::SIG => {
                structure_access!(map_access, Ignored, no_sig_key);
                Ok(Self::from(Ignored))
            }
            Init::SIG => {
                let (client, auth) = structure_access!(map_access, Init, no_sig_key);
                Ok(Self::from(Init { client, auth }))
            }
            Node::SIG => {
                let (identity, labels, properties) =
                    structure_access!(map_access, Node, no_sig_key);
                Ok(Self::from(Node {
                    identity,
                    labels,
                    properties,
                }))
            }
            Path::SIG => {
                let (nodes, relationships, sequence) =
                    structure_access!(map_access, Path, no_sig_key);
                Ok(Self::from(Path {
                    nodes,
                    relationships,
                    sequence,
                }))
            }
            PullAll::SIG => {
                structure_access!(map_access, Path, no_sig_key);
                Ok(Self::from(PullAll))
            }
            Record::SIG => {
                let fields = structure_access!(map_access, Record, no_sig_key);
                Ok(Self::from(Record {
                    fields: fields.value(),
                }))
            }
            Relationship::SIG => {
                let (identity, start_node_identity, end_node_identity, r#type, properties) =
                    structure_access!(map_access, Relationship, no_sig_key);
                Ok(Self::from(Relationship {
                    identity,
                    start_node_identity,
                    end_node_identity,
                    r#type,
                    properties,
                }))
            }
            Reset::SIG => {
                structure_access!(map_access, Reset, no_sig_key);
                Ok(Self::from(Reset))
            }
            Run::SIG => {
                let (statement, parameters) = structure_access!(map_access, Run, no_sig_key);
                Ok(Self::from(Run {
                    statement,
                    parameters,
                }))
            }
            Success::SIG => {
                let fields = structure_access!(map_access, Success, no_sig_key);
                Ok(Self::from(Success {
                    metadata: fields.value(),
                }))
            }
            UnboundRelationship::SIG => {
                let (identity, r#type, properties) =
                    structure_access!(map_access, UnboundRelationship, no_sig_key);
                Ok(Self::from(UnboundRelationship {
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

impl From<Node> for Structure {
    fn from(value: Node) -> Self {
        Self::Node(value)
    }
}

impl From<Path> for Structure {
    fn from(value: Path) -> Self {
        Self::Path(value)
    }
}

impl From<Relationship> for Structure {
    fn from(value: Relationship) -> Self {
        Self::Relationship(value)
    }
}

impl From<UnboundRelationship> for Structure {
    fn from(value: UnboundRelationship) -> Self {
        Self::UnboundRelationship(value)
    }
}

impl From<AckFailure> for Structure {
    fn from(value: AckFailure) -> Self {
        Self::AckFailure(value)
    }
}

impl From<DiscardAll> for Structure {
    fn from(value: DiscardAll) -> Self {
        Self::DiscardAll(value)
    }
}

impl From<Failure> for Structure {
    fn from(value: Failure) -> Self {
        Self::Failure(value)
    }
}

impl From<Ignored> for Structure {
    fn from(value: Ignored) -> Self {
        Self::Ignored(value)
    }
}

impl From<Init> for Structure {
    fn from(value: Init) -> Self {
        Self::Init(value)
    }
}

impl From<PullAll> for Structure {
    fn from(value: PullAll) -> Self {
        Self::PullAll(value)
    }
}

impl From<Record> for Structure {
    fn from(value: Record) -> Self {
        Self::Record(value)
    }
}

impl From<Reset> for Structure {
    fn from(value: Reset) -> Self {
        Self::Reset(value)
    }
}

impl From<Run> for Structure {
    fn from(value: Run) -> Self {
        Self::Run(value)
    }
}

impl From<Success> for Structure {
    fn from(value: Success) -> Self {
        Self::Success(value)
    }
}
