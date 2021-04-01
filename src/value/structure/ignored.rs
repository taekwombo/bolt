use super::super::BoltStructure;
use crate::constants::STRUCTURE_NAME;
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

const MSG_IGNORED_SIGNATURE: u8 = 0x7E;
const MSG_IGNORED_LENGTH: u8 = 0x00;
const MSG_IGNORED_SERIALIZE_LENGTH: usize =
    serialize_length!(MSG_IGNORED_SIGNATURE, MSG_IGNORED_LENGTH);

#[derive(Debug, PartialEq)]
pub struct Ignored;

impl BoltStructure for Ignored {
    const SIG: u8 = 0x7E;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Vec<()>;
}

impl ser::Serialize for Ignored {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?
            .end()
    }
}

impl<'de> de::Deserialize<'de> for Ignored {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(IgnoredVisitor)
    }
}

struct IgnoredVisitor;

impl<'de> de::Visitor<'de> for IgnoredVisitor {
    type Value = Ignored;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Ignored")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        structure_access!(map_access, Ignored, fields(0));
        Ok(Ignored)
    }
}

#[cfg(test)]
mod ignored_test {
    use super::*;
    use crate::{test, constants::marker::TINY_STRUCT, from_bytes, to_bytes};

    const BYTES: &[u8] = &[TINY_STRUCT, Ignored::SIG];

    #[test]
    fn serialize() {
        test::ser(&Ignored, BYTES);
    }

    #[test]
    fn deserialize() {
        test::de(&Ignored, BYTES);
    }

    #[test]
    fn deserialize_fail() {
        test::de_err::<Ignored>(&[TINY_STRUCT, Ignored::SIG + 1]);
        test::de_err::<Ignored>(&[TINY_STRUCT, Ignored::SIG, 0]);
    }
}
