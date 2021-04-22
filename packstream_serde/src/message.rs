//!  # Bolt Message Structures
//!  [List of Bolt Messages](https://7687.org/bolt/bolt-protocol-message-specification-1.html#messages)
//!

use std::fmt;
use crate::packstream::PackstreamStructure;
use serde::de::Error;

mod init;
mod ack_failure;
mod reset;
mod run;
mod discard_all;
mod pull_all;
mod success;
mod ignored;
mod failure;
mod record;

pub use init::{BasicAuth, Init};
pub use ack_failure::AckFailure;
pub use reset::Reset;
pub use run::Run;
pub use discard_all::DiscardAll;
pub use pull_all::PullAll;
pub use success::Success;
pub use ignored::Ignored;
pub use failure::Failure;
pub use record::Record;

mod ser;
mod de;

/// Represents request message.
#[derive(PartialEq)]
pub enum RequestMessage {
    Init(Init),
    AckFailure(AckFailure),
    Reset(Reset),
    Run(Run),
    DiscardAll(DiscardAll),
    PullAll(PullAll),
}

impl RequestMessage {
    pub(crate) fn from_map_access_no_sig_key<'de, V>(map_access: &mut V) -> Result<Self, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        match map_access.next_value::<u8>()? {
            Init::SIG => {
                let (client, auth) = structure_access!(map_access, Init, no_sig_key);
                Ok(Self::from(Init { client, auth }))
            }
            AckFailure::SIG => {
                structure_access!(map_access, AckFailure, no_sig_key);
                Ok(Self::from(AckFailure))
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
            DiscardAll::SIG => {
                structure_access!(map_access, DiscardAll, no_sig_key);
                Ok(Self::from(DiscardAll))
            }
            PullAll::SIG => {
                structure_access!(map_access, PullAll, no_sig_key);
                Ok(Self::from(PullAll))
            }
            signature => Err(V::Error::custom(format!(
                "Expected signature of a known Structure, got {}",
                signature,
            ))),
        }
    }
}
impl fmt::Debug for RequestMessage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          Self::Init(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::AckFailure(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::Reset(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::Run(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::DiscardAll(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::PullAll(v) => f.debug_tuple("RequestMessage").field(v).finish(),
      }
  }
}

impl fmt::Display for RequestMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
          Self::Init(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::AckFailure(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::Reset(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::Run(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::DiscardAll(v) => f.debug_tuple("RequestMessage").field(v).finish(),
          Self::PullAll(v) => f.debug_tuple("RequestMessage").field(v).finish(),
        }
    }
}

impl From<Init> for RequestMessage {
    fn from(value: Init) -> Self {
        Self::Init(value)
    }
}

impl From<AckFailure> for RequestMessage {
    fn from(value: AckFailure) -> Self {
        Self::AckFailure(value)
    }
}

impl From<Reset> for RequestMessage {
    fn from(value: Reset) -> Self {
        Self::Reset(value)
    }
}

impl From<Run> for RequestMessage {
    fn from(value: Run) -> Self {
        Self::Run(value)
    }
}

impl From<DiscardAll> for RequestMessage {
    fn from(value: DiscardAll) -> Self {
        Self::DiscardAll(value)
    }
}

impl From<PullAll> for RequestMessage {
    fn from(value: PullAll) -> Self {
        Self::PullAll(value)
    }
}

/// Represents summary message.
#[derive(PartialEq)]
pub enum SummaryMessage {
    Success(Success),
    Ignored(Ignored),
    Failure(Failure),
}

impl SummaryMessage {
    pub(crate) fn from_map_access_no_sig_key<'de, V>(map_access: &mut V) -> Result<Self, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        match map_access.next_value::<u8>()? {
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
            Success::SIG => {
                let fields = structure_access!(map_access, Success, no_sig_key);
                Ok(Self::from(Success {
                    metadata: fields.value(),
                }))
            }
            signature => Err(V::Error::custom(format!(
                "Expected signature of a known Structure, got {}",
                signature,
            ))),
        }
    }
}

impl fmt::Debug for SummaryMessage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          Self::Success(v) => f.debug_tuple("SummaryMessage").field(v).finish(),
          Self::Ignored(v) => f.debug_tuple("SummaryMessage").field(v).finish(),
          Self::Failure(v) => f.debug_tuple("SummaryMessage").field(v).finish(),
      }
  }
}

impl fmt::Display for SummaryMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
          Self::Success(v) => f.debug_tuple("SummaryMessage").field(v).finish(),
          Self::Ignored(v) => f.debug_tuple("SummaryMessage").field(v).finish(),
          Self::Failure(v) => f.debug_tuple("SummaryMessage").field(v).finish(),
        }
    }
}

impl From<Success> for SummaryMessage {
    fn from(value: Success) -> Self {
        Self::Success(value)
    }
}

impl From<Ignored> for SummaryMessage {
    fn from(value: Ignored) -> Self {
        Self::Ignored(value)
    }
}

impl From<Failure> for SummaryMessage {
    fn from(value: Failure) -> Self {
        Self::Failure(value)
    }
}
