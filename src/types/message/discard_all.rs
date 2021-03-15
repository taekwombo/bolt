use crate::constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY};
use serde::{
    de::{self, Error},
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

const MSG_DISCARD_ALL_SIGNATURE: u8 = 0x2F;
const MSG_DISCARD_ALL_LENGTH: u8 = 0x00;
const MSG_DISCARD_ALL_SERIALIZE_LENGTH: usize =
    serialize_length!(MSG_DISCARD_ALL_SIGNATURE, MSG_DISCARD_ALL_LENGTH);

#[derive(Debug, PartialEq)]
pub struct DiscardAll;

impl ser::Serialize for DiscardAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, MSG_DISCARD_ALL_SERIALIZE_LENGTH)?
            .end()
    }
}

impl<'de> de::Deserialize<'de> for DiscardAll {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(DiscardAllVisitor)
    }
}

struct DiscardAllVisitor;

impl<'de> de::Visitor<'de> for DiscardAllVisitor {
    type Value = DiscardAll;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("DiscardAll message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_DISCARD_ALL_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                    fields(),
                    key(),
                });
                Ok(DiscardAll)
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

    const BYTES: &'static [u8] = &[TINY_STRUCT, MSG_DISCARD_ALL_SIGNATURE];

    #[test]
    fn serialize() {
        let result = to_bytes(&DiscardAll);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<DiscardAll>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DiscardAll);
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<DiscardAll>(&[TINY_STRUCT, MSG_DISCARD_ALL_SIGNATURE + 1]);
        assert!(result.is_err());
    }
}
