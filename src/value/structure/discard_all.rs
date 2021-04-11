use super::{BoltStructure, Empty};
use crate::constants::STRUCTURE_NAME;
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct DiscardAll;

impl BoltStructure for DiscardAll {
    const SIG: u8 = 0x2F;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Empty;
}

impl fmt::Display for DiscardAll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DiscardAll")
    }
}

impl ser::Serialize for DiscardAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?
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
        formatter.write_str("DiscardAll")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        structure_access!(map_access, DiscardAll);
        Ok(DiscardAll)
    }
}

#[cfg(test)]
mod test_discard_all {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, test};

    const BYTES: &[u8] = &[TINY_STRUCT + DiscardAll::LEN, DiscardAll::SIG];

    #[test]
    fn bytes() {
        test::ser_de::<DiscardAll>(BYTES);
        test::de_ser(DiscardAll);
        test::de_err::<DiscardAll>(&[TINY_STRUCT, DiscardAll::SIG + 1]);
    }

    #[test]
    fn bytes_fail() {}
}
