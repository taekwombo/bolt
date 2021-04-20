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
            Self::Date(v) => v.serialize(serializer),
            Self::Time(v) => v.serialize(serializer),
            Self::LocalTime(v) => v.serialize(serializer),
            Self::DateTime(v) => v.serialize(serializer),
            Self::DateTimeZoneId(v) => v.serialize(serializer),
            Self::LocalDateTime(v) => v.serialize(serializer),
            Self::Duration(v) => v.serialize(serializer),
            Self::Point2D(v) => v.serialize(serializer),
            Self::Point3D(v) => v.serialize(serializer),
        }
    }
}
