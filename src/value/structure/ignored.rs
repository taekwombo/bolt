use super::{BoltStructure, Empty};
use crate::constants::STRUCTURE_NAME;
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Ignored;

impl BoltStructure for Ignored {
    const SIG: u8 = 0x7E;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Empty;
}

impl fmt::Display for Ignored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Ignored")
    }
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
        structure_access!(map_access, Ignored);
        Ok(Ignored)
    }
}

#[cfg(test)]
mod test_ignored {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, test};

    const BYTES: &[u8] = &[TINY_STRUCT + Ignored::LEN, Ignored::SIG];

    #[test]
    fn bytes() {
        test::ser_de::<Ignored>(BYTES);
        test::de_ser(Ignored);
        test::de_err::<Ignored>(&[TINY_STRUCT, Ignored::SIG + 1]);
    }
}
