use crate::constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

const MSG_ACK_FAILURE_SIGNATURE: u8 = 0x0E;
const MSG_ACK_FAILURE_LENGTH: u8 = 0x00;
const MSG_ACK_FAILURE_SERIALIZE_LENGTH: usize =
    serialize_length!(MSG_ACK_FAILURE_SIGNATURE, MSG_ACK_FAILURE_LENGTH);

#[derive(Debug, PartialEq)]
pub struct AckFailure;

impl ser::Serialize for AckFailure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, MSG_ACK_FAILURE_SERIALIZE_LENGTH)?
            .end()
    }
}

impl<'de> de::Deserialize<'de> for AckFailure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(AckFailureVisitor)
    }
}

struct AckFailureVisitor;

impl<'de> de::Visitor<'de> for AckFailureVisitor {
    type Value = AckFailure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("AckFailure message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_ACK_FAILURE_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                    fields(),
                    key(),
                });
                Ok(AckFailure)
            }
            Some(key) => unexpected_key_access!(key),
            _ => unexpected_key_access!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, from_bytes, to_bytes};

    const BYTES: &[u8] = &[TINY_STRUCT, MSG_ACK_FAILURE_SIGNATURE];

    #[test]
    fn serialize() {
        let result = to_bytes(&AckFailure);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<AckFailure>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), AckFailure);
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<AckFailure>(&[TINY_STRUCT, MSG_ACK_FAILURE_SIGNATURE + 1]);
        assert!(result.is_err());
    }
}
