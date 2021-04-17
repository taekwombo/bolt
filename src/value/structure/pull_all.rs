use super::{BoltStructure, Empty, Value};
use crate::{
    constants::STRUCTURE_NAME,
    error::{SerdeError, SerdeResult},
};
use serde::{
    de, forward_to_deserialize_any,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct PullAll;

impl BoltStructure for PullAll {
    const SIG: u8 = 0x3F;
    const LEN: u8 = 0x00;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Empty;

    fn into_value(self) -> Value {
        value_map! {}
    }
}

impl fmt::Display for PullAll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AckFailure")
    }
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
        structure_access!(map_access, PullAll);
        Ok(PullAll)
    }
}

impl<'de> de::Deserializer<'de> for PullAll {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.into_value().deserialize_map(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier enum ignored_any
    }
}

#[cfg(test)]
mod test_pull_all {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, test};

    const BYTES: &[u8] = &[TINY_STRUCT + PullAll::LEN, PullAll::SIG];

    #[test]
    fn bytes() {
        test::ser_de::<PullAll>(BYTES);
        test::de_ser(PullAll);
        test::de_err::<PullAll>(&[TINY_STRUCT, PullAll::SIG + 1]);
    }
}
