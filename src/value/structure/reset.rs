use super::super::BoltStructure;
use crate::constants::STRUCTURE_NAME;
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

impl BoltStructure for Reset {
    const SIG: u8 = 0x0F;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Vec<()>;
}

impl ser::Serialize for Reset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?
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
        formatter.write_str("Reset")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        structure_access!(map_access, Reset, fields(0));
        Ok(Reset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test, constants::marker::TINY_STRUCT, from_bytes, to_bytes};

    const BYTES: &[u8] = &[TINY_STRUCT, Reset::SIG];

    #[test]
    fn serialize() {
        test::ser(&Reset, BYTES);
    }

    #[test]
    fn deserialize() {
        test::de(&Reset, BYTES);
    }

    #[test]
    fn deserialize_fail() {
        test::de_err::<Reset>(&[TINY_STRUCT, Reset::SIG + 1]);
        test::de_err::<Reset>(&[TINY_STRUCT, Reset::SIG, 0]);
    }
}
