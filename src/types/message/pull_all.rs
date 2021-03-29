use crate::constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

const MSG_PULL_ALL_SIGNATURE: u8 = 0x3F;
const MSG_PULL_ALL_LENGTH: u8 = 0x00;
const MSG_PULL_ALL_SERIALIZE_LENGTH: usize =
    serialize_length!(MSG_PULL_ALL_SIGNATURE, MSG_PULL_ALL_LENGTH);

#[derive(Debug, PartialEq)]
pub struct PullAll;

impl ser::Serialize for PullAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, MSG_PULL_ALL_SERIALIZE_LENGTH)?
            .end()
    }
}

impl<'de> de::Deserialize<'de> for PullAll {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(PullAllVisitor)
    }
}

struct PullAllVisitor;

impl<'de> de::Visitor<'de> for PullAllVisitor {
    type Value = PullAll;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("PullAll message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_PULL_ALL_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                    fields(),
                    key(),
                });
                Ok(PullAll)
            }
            Some(key) => unexpected_key_access!(key),
            None => unexpected_key_access!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, from_bytes, to_bytes};

    const BYTES: &[u8] = &[TINY_STRUCT, MSG_PULL_ALL_SIGNATURE];

    #[test]
    fn serialize() {
        let result = to_bytes(&PullAll);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<PullAll>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PullAll);
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<PullAll>(&[TINY_STRUCT, MSG_PULL_ALL_SIGNATURE + 1]);
        assert!(result.is_err());
    }
}
