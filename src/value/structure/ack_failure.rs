use super::{BoltStructure, Empty};
use crate::constants::STRUCTURE_NAME;
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct AckFailure;

impl BoltStructure for AckFailure {
    const SIG: u8 = 0x0E;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Empty;
}

impl fmt::Display for AckFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AckFailure")
    }
}

impl ser::Serialize for AckFailure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?
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
        formatter.write_str("AckFailure")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        structure_access!(map_access, AckFailure);
        Ok(AckFailure)
    }
}

#[cfg(test)]
mod test_ack_failure {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, test};

    const BYTES: &[u8] = &[TINY_STRUCT + AckFailure::LEN, AckFailure::SIG];

    #[test]
    fn bytes() {
        test::ser_de::<AckFailure>(BYTES);
        test::de_ser(AckFailure);
        test::de_err::<AckFailure>(&[TINY_STRUCT, AckFailure::SIG + 1]);
    }
}
