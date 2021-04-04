use super::super::BoltStructure;
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct Success {
    metadata: HashMap<String, Value>,
}

impl BoltStructure for Success {
    const SIG: u8 = 0x70;
    const LEN: u8 = 0x01;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Vec<HashMap<String, Value>>;
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
        let mut fields = structure_access!(map_access, Success);
        Ok(Success {
            metadata: fields.pop().expect("Fields to have on element"),
        })
    }
}

#[cfg(test)]
mod test_success {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, from_bytes, test, to_bytes};

    const BYTES: &[u8] = &[
        0xB1, 0x70, 0xA1, 0x86, 0x66, 0x69, 0x65, 0x6C, 0x64, 0x73, 0x92, 0x84, 0x6E, 0x61, 0x6D,
        0x65, 0x83, 0x61, 0x67, 0x65,
    ];

    fn create_success() -> Success {
        let mut metadata = HashMap::new();
        metadata.insert(
            String::from("fields"),
            Value::List(vec![
                Value::String(String::from("name")),
                Value::String(String::from("age")),
            ]),
        );
        Success { metadata }
    }

    #[test]
    fn serialize() {
        test::ser(&create_success(), BYTES);
    }

    #[test]
    fn deserialize() {
        test::de(&create_success(), BYTES);
    }

    #[test]
    fn deserialize_fail() {
        test::de_err::<Success>(&BYTES[0..(BYTES.len() - 1)]);
        test::de_err::<Success>(&[TINY_STRUCT, Success::SIG + 1]);
    }
}
