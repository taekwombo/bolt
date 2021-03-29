use crate::constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

const MSG_RESET_SIGNATURE: u8 = 0x0F;
const MSG_RESET_LENGTH: u8 = 0x00;
const MSG_RESET_SERIALIZE_LENGTH: usize = serialize_length!(MSG_RESET_SIGNATURE, MSG_RESET_LENGTH);

#[derive(Debug, PartialEq)]
pub struct Reset;

impl ser::Serialize for Reset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, MSG_RESET_SERIALIZE_LENGTH)?
            .end()
    }
}

impl<'de> de::Deserialize<'de> for Reset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(ResetVisitor)
    }
}

struct ResetVisitor;

impl<'de> de::Visitor<'de> for ResetVisitor {
    type Value = Reset;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Reset message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_RESET_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                    fields(),
                    key(),
                });
                Ok(Reset)
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

    const BYTES: &'static [u8] = &[TINY_STRUCT, MSG_RESET_SIGNATURE];

    #[test]
    fn serialize() {
        let result = to_bytes(&Reset);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<Reset>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Reset);
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<Reset>(&[TINY_STRUCT, MSG_RESET_SIGNATURE + 1]);
        assert!(result.is_err());
    }
}
