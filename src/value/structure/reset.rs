use super::{BoltStructure, Empty};
use crate::constants::STRUCTURE_NAME;
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Reset;

impl BoltStructure for Reset {
    const SIG: u8 = 0x0F;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Empty;
}

impl fmt::Display for Reset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Reset")
    }
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
        structure_access!(map_access, Reset);
        Ok(Reset)
    }
}

#[cfg(test)]
mod test_reset {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, test};

    const BYTES: &[u8] = &[TINY_STRUCT + Reset::LEN, Reset::SIG];

    #[test]
    fn bytes() {
        test::ser_de::<Reset>(BYTES);
        test::de_ser(Reset);
        test::de_err::<Reset>(&[TINY_STRUCT, Reset::SIG + 1]);
    }
}
