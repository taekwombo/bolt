use std::fmt;
use crate::packstream::PackstreamStructure;
use serde::de::Error;

mod de;
mod ser;

mod node;
mod path;
mod relationship;
mod unbound_relationship;
mod date;
mod time;
mod local_time;
mod date_time;
mod date_time_zone_id;
mod local_date_time;
mod duration;
mod point_2d;
mod point_3d;

pub use node::Node;
pub use path::Path;
pub use relationship::Relationship;
pub use unbound_relationship::UnboundRelationship;
pub use date::Date;
pub use time::Time;
pub use local_time::LocalTime;
pub use date_time::DateTime;
pub use date_time_zone_id::DateTimeZoneId;
pub use local_date_time::LocalDateTime;
pub use duration::Duration;
pub use point_2d::Point2D;
pub use point_3d::Point3D;

/// Represents any possible [`Bolt Structure`].
///
/// [`Bolt Structure`]: https://boltprotocol.org/v1/#structures
#[derive(PartialEq)]
pub enum Structure {
    Node(Node),
    Path(Path),
    Relationship(Relationship),
    UnboundRelationship(UnboundRelationship),
    Date(Date),
    Time(Time),
    LocalTime(LocalTime),
    DateTime(DateTime),
    DateTimeZoneId(DateTimeZoneId),
    LocalDateTime(LocalDateTime),
    Duration(Duration),
    Point2D(Point2D),
    Point3D(Point3D),
}

impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Node(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Path(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Relationship(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::UnboundRelationship(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Date(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Time(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::LocalTime(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::DateTime(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::DateTimeZoneId(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::LocalDateTime(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Duration(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Point2D(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Point3D(v) => f.debug_tuple("Structure").field(v).finish(),
        }
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Node(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Path(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Relationship(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::UnboundRelationship(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Date(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Time(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::LocalTime(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::DateTime(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::DateTimeZoneId(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::LocalDateTime(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Duration(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Point2D(v) => f.debug_tuple("Structure").field(v).finish(),
            Self::Point3D(v) => f.debug_tuple("Structure").field(v).finish(),
        }
    }
}

impl Structure {
    pub(crate) fn from_map_access_no_sig_key<'de, V>(map_access: &mut V) -> Result<Self, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        match map_access.next_value::<u8>()? {
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
            UnboundRelationship::SIG => {
                let (identity, r#type, properties) =
                    structure_access!(map_access, UnboundRelationship, no_sig_key);
                Ok(Self::from(UnboundRelationship {
                    identity,
                    r#type,
                    properties,
                }))
            }
            Date::SIG => {
                let days = structure_access!(map_access, Date, no_sig_key);

                Ok(Self::from(Date {
                    days: days.value(),
                }))
            },
            Time::SIG => {
                let (nanoseconds, tz_offset_seconds) = structure_access!(map_access, Time, no_sig_key);

                Ok(Self::from(Time {
                    nanoseconds,
                    tz_offset_seconds,
                }))
            },
            LocalTime::SIG => {
                let nanoseconds = structure_access!(map_access, LocalTime, no_sig_key);

                Ok(Self::from(LocalTime {
                    nanoseconds: nanoseconds.value()
                }))
            }
            DateTime::SIG => {
                let (seconds, nanoseconds, tz_offset_seconds) = structure_access!(
                    map_access,
                    DateTime,
                    no_sig_key
                );

                Ok(Self::from(DateTime {
                    seconds,
                    nanoseconds,
                    tz_offset_seconds,
                }))
            }
            DateTimeZoneId::SIG => {
                let (seconds, nanoseconds, tz_id) = structure_access!(
                    map_access,
                    DateTimeZoneId,
                    no_sig_key
                );

                Ok(Self::from(DateTimeZoneId {
                    seconds,
                    nanoseconds,
                    tz_id,
                }))
            }
            LocalDateTime::SIG => {
                let (seconds, nanoseconds) = structure_access!(map_access, LocalDateTime, no_sig_key);

                Ok(Self::from(LocalDateTime {
                    seconds,
                    nanoseconds,
                }))
            }
            Duration::SIG => {
                let (months, days, seconds, nanoseconds) = structure_access!(map_access, Duration, no_sig_key);

                Ok(Self::from(Duration {
                    months,
                    days,
                    seconds,
                    nanoseconds,
                }))
            }
            Point2D::SIG => {
                let (srid, x, y) = structure_access!(map_access, Point2D, no_sig_key);

                Ok(Self::from(Point2D {
                    srid,
                    x,
                    y,
                }))
            }
            Point3D::SIG => {
                let (srid, x, y, z) = structure_access!(map_access, Point3D, no_sig_key);

                Ok(Self::from(Point3D {
                    srid,
                    x,
                    y,
                    z,
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

impl From<Date> for Structure {
    fn from(value: Date) -> Self {
        Self::Date(value)
    }
}

impl From<Time> for Structure {
    fn from(value: Time) -> Self {
        Self::Time(value)
    }
}

impl From<LocalTime> for Structure {
    fn from(value: LocalTime) -> Self {
        Self::LocalTime(value)
    }
}

impl From<DateTime> for Structure {
    fn from(value: DateTime) -> Self {
        Self::DateTime(value)
    }
}

impl From<DateTimeZoneId> for Structure {
    fn from(value: DateTimeZoneId) -> Self {
        Self::DateTimeZoneId(value)
    }
}

impl From<LocalDateTime> for Structure {
    fn from(value: LocalDateTime) -> Self {
        Self::LocalDateTime(value)
    }
}

impl From<Duration> for Structure {
    fn from(value: Duration) -> Self {
        Self::Duration(value)
    }
}

impl From<Point2D> for Structure {
    fn from(value: Point2D) -> Self {
        Self::Point2D(value)
    }
}

impl From<Point3D> for Structure {
    fn from(value: Point3D) -> Self {
        Self::Point3D(value)
    }
}
