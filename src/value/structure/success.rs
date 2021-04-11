use super::{BoltStructure, Single};
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct Success {
    pub metadata: HashMap<String, Value>,
}

impl BoltStructure for Success {
    const SIG: u8 = 0x70;
    const LEN: u8 = 0x01;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Single<HashMap<String, Value>>;
}

impl fmt::Display for Success {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Success").field(&self.metadata).finish()
    }
}

impl ser::Serialize for Success {
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

impl<'de> de::Deserialize<'de> for Success {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(SuccessVisitor)
    }
}

struct SuccessVisitor;

impl<'de> de::Visitor<'de> for SuccessVisitor {
    type Value = Success;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Success")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let fields = structure_access!(map_access, Success);
        Ok(Success {
            metadata: fields.value(),
        })
    }
}

#[cfg(test)]
mod test_success {
    use super::*;
    use crate::{
        constants::marker::{TINY_MAP, TINY_STRUCT},
        test,
    };

    const BYTES: &[u8] = &[TINY_STRUCT + Success::LEN, Success::SIG, TINY_MAP];

    #[test]
    fn bytes() {
        test::ser_de::<Success>(BYTES);
        test::de_ser(Success {
            metadata: HashMap::new(),
        });
        test::de_err::<Success>(&BYTES[0..(BYTES.len() - 1)]);
    }
}
