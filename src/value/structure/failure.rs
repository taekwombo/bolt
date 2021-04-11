use super::{BoltStructure, Single};
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct Failure {
    metadata: HashMap<String, Value>,
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Failure").field(&self.metadata).finish()
    }
}

impl BoltStructure for Failure {
    const SIG: u8 = 0x7F;
    const LEN: u8 = 0x01;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    // TODO(@krnik): Implement one-elemnt list type like e.g. AsList<T>(T)
    // which will be used to deserialize one-element structure fields
    // more ergonomically. Also it will greatly simplify structure_access
    // macro code.
    // Consider: Error handling clarity.
    type Fields = Single<HashMap<String, Value>>;
}

impl ser::Serialize for Failure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.metadata)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Failure {
    fn deserialize<D>(deserializer: D) -> Result<Failure, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(FailureVisitor)
    }
}

struct FailureVisitor;

impl<'de> de::Visitor<'de> for FailureVisitor {
    type Value = Failure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Failure")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let fields = structure_access!(map_access, Failure);
        Ok(Failure {
            metadata: fields.value(),
        })
    }
}

#[cfg(test)]
mod test_failure {
    use super::*;
    use crate::{
        constants::marker::{TINY_MAP, TINY_STRUCT},
        from_bytes, test, to_bytes,
    };

    const BYTES: &[u8] = &[TINY_STRUCT + 1, Failure::SIG, TINY_MAP];

    fn create_failure() -> Failure {
        Failure {
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn serializefailure() {
        test::ser(&create_failure(), BYTES);
    }

    #[test]
    fn deserialize() {
        test::de(&create_failure(), BYTES);
    }

    #[test]
    fn deserialize_fail() {
        test::de_err::<Failure>(&[TINY_STRUCT, Failure::SIG]);
        test::de_err::<Failure>(&[TINY_STRUCT, Failure::SIG + 1, TINY_MAP]);
    }
}
