use super::super::BoltStructure;
use crate::constants::STRUCTURE_NAME;
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct PullAll;

impl BoltStructure for PullAll {
    const SIG: u8 = 0x3F;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Vec<()>;
}

impl ser::Serialize for PullAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?
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
        formatter.write_str("PullAll")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        structure_access!(map_access, PullAll, fields(0));
        Ok(PullAll)
    }
}

#[cfg(test)]
mod test_pull_all {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, from_bytes, test, to_bytes};

    const BYTES: &[u8] = &[TINY_STRUCT, PullAll::SIG];

    #[test]
    fn serialize() {
        test::ser(&PullAll, BYTES);
    }

    #[test]
    fn deserialize() {
        test::de(&PullAll, BYTES);
    }

    #[test]
    fn deserialize_fail() {
        test::de_err::<PullAll>(&[TINY_STRUCT, PullAll::SIG + 1]);
        test::de_err::<PullAll>(&[TINY_STRUCT, PullAll::SIG, 0]);
    }
}
