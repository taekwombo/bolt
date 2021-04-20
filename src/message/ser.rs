use super::{RequestMessage, SummaryMessage};
use serde::ser;

impl ser::Serialize for RequestMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Self::AckFailure(v) => v.serialize(serializer),
            Self::DiscardAll(v) => v.serialize(serializer),
            Self::Init(v) => v.serialize(serializer),
            Self::PullAll(v) => v.serialize(serializer),
            Self::Reset(v) => v.serialize(serializer),
            Self::Run(v) => v.serialize(serializer),
        }
    }
}

impl ser::Serialize for SummaryMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Self::Success(v) => v.serialize(serializer),
            Self::Ignored(v) => v.serialize(serializer),
            Self::Failure(v) => v.serialize(serializer),
        }
    }
}

