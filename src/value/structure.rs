use super::Value;

mod graph;
mod message;

pub use graph::*;
pub use message::*;

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
    // TODO(@krnik): Is this variant necessary?
    Custom { signature: u8, fields: Vec<Value> },
}
