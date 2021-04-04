//use super::Value;
//use crate::constants::STRUCTURE_NAME;

//mod de;
//mod ser;

mod ack_failure;
mod discard_all;
mod failure;
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

pub enum Structure {
//    Node(Node),
//    Path(Path),
//    Relationship(Relationship),
//    UnboundRelationship(UnboundRelationship),
//    AckFailure(AckFailure),
//    DiscardAll(DiscardAll),
//    Failure(Failure),
//    Ignored(Ignored),
//    Init(Init),
//    PullAll(PullAll),
//    Record(Record),
//    Reset(Reset),
//    Run(Run),
//    Success(Success),
//    TODO(@krnik): Is this variant necessary?
//    Custom { signature: u8, fields: Vec<Value> },
}
