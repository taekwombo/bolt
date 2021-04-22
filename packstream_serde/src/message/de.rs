use super::{RequestMessage, SummaryMessage};
use crate::{
    constants::STRUCTURE_SIG_KEY,
    error::{PackstreamError, PackstreamResult},
};
use serde::{de, forward_to_deserialize_any};
use std::fmt;

struct RequestMessageVisitor;

impl<'de> de::Visitor<'de> for RequestMessageVisitor {
    type Value = RequestMessage;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("RequestMessage")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        check!(__key, map_access, STRUCTURE_SIG_KEY);
        RequestMessage::from_map_access_no_sig_key(&mut map_access)
    }
}

impl<'de> de::Deserialize<'de> for RequestMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(RequestMessageVisitor)
    }
}

impl<'de> de::Deserializer<'de> for RequestMessage {
    type Error = PackstreamError;

    fn deserialize_any<V>(self, visitor: V) -> PackstreamResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Init(de) => de.deserialize_any(visitor),
            Self::AckFailure(de) => de.deserialize_any(visitor),
            Self::Reset(de) => de.deserialize_any(visitor),
            Self::Run(de) => de.deserialize_any(visitor),
            Self::DiscardAll(de) => de.deserialize_any(visitor),
            Self::PullAll(de) => de.deserialize_any(visitor),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct SummaryMessageVisitor;

impl<'de> de::Visitor<'de> for SummaryMessageVisitor {
    type Value = SummaryMessage;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("SummaryMessage")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        check!(__key, map_access, STRUCTURE_SIG_KEY);
        SummaryMessage::from_map_access_no_sig_key(&mut map_access)
    }
}

impl<'de> de::Deserialize<'de> for SummaryMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(SummaryMessageVisitor)
    }
}

impl<'de> de::Deserializer<'de> for SummaryMessage {
    type Error = PackstreamError;

    fn deserialize_any<V>(self, visitor: V) -> PackstreamResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Success(de) => de.deserialize_any(visitor),
            Self::Ignored(de) => de.deserialize_any(visitor),
            Self::Failure(de) => de.deserialize_any(visitor),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
