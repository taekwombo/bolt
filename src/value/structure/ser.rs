use super::Structure;
use serde::ser;

impl ser::Serialize for Structure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Self::Node(v) => v.serialize(serializer),
            Self::Path(v) => v.serialize(serializer),
            Self::Relationship(v) => v.serialize(serializer),
            Self::UnboundRelationship(v) => v.serialize(serializer),
            Self::AckFailure(v) => v.serialize(serializer),
            Self::DiscardAll(v) => v.serialize(serializer),
            Self::Failure(v) => v.serialize(serializer),
            Self::Ignored(v) => v.serialize(serializer),
            Self::Init(v) => v.serialize(serializer),
            Self::PullAll(v) => v.serialize(serializer),
            Self::Record(v) => v.serialize(serializer),
            Self::Reset(v) => v.serialize(serializer),
            Self::Run(v) => v.serialize(serializer),
            Self::Success(v) => v.serialize(serializer),
        }
    }
}
