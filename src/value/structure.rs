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

use ack_failure::AckFailure;
use discard_all::DiscardAll;
use failure::Failure;
use fields::{Empty, Single};
use ignored::Ignored;
use init::Init;
use node::Node;
use path::Path;
use pull_all::PullAll;
use record::Record;
use relationship::Relationship;
use reset::Reset;
use run::Run;
use success::Success;
use unbound_relationship::UnboundRelationship;

trait BoltStructure {
    const SIG: u8;
    const LEN: u8;
    const SERIALIZE_LEN: usize;

    type Fields;
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
    //    TODO(@krnik): Is this variant necessary?
    //    Custom { signature: u8, fields: Vec<Value> },
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
