use std::fmt;
use serde::{de::{self, Error}, ser::{self, SerializeTupleStruct}};
use crate::constants::{STRUCTURE_NAME, STRUCTURE_SIG_KEY, STRUCTURE_FIELDS_KEY};

const MSG_IGNORED_SIGNATURE: u8 = 0x7E;
const MSG_IGNORED_LENGTH: u8 = 0x00;
const MSG_IGNORED_SERIALIZE_LENGTH: usize = serialize_length!(MSG_IGNORED_SIGNATURE, MSG_IGNORED_LENGTH);

#[derive(Debug, PartialEq)]
pub struct Ignored;

impl ser::Serialize for Ignored {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer
            .serialize_tuple_struct(
                STRUCTURE_NAME,
                MSG_IGNORED_SERIALIZE_LENGTH,
            )?
            .end()
    }
}

impl<'de> de::Deserialize<'de> for Ignored {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        deserializer.deserialize_map(IgnoredVisitor)
    }
}

struct IgnoredVisitor;

impl<'de> de::Visitor for IgnoredVisitor {
    type Value = Ignored;

    fn expecting(&self, mut formatter: fmt::Formatter) -> fmt::Result {
        formatter.write_str("Ignored message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
        where V: de::MapAccess<'de>
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_IGNORED_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                    fields(),
                    key(),
                });
                Ok(Ignored)
            },
            Some(key) => unexpected_key_access!(key),
            None => unexpected_key_access!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_bytes, to_bytes, constants::marker::TINY_STRUCT};

    const BYTES: &'static [u8] = &[TINY_STRUCT, MSG_IGNORED_SIGNATURE];

    #[test]
    fn serialize () {
        let result = to_bytes(&Ignored);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize () {
        let result = from_bytes::<Ignored>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Ignored);
    }

    #[test]
    fn deserialize_fail () {
        let result = from_bytes::<Ignored>(&[TINY_STRUCT, MSG_IGNORED_SIGNATURE + 1]);
        assert!(result.is_err());
    }
}

