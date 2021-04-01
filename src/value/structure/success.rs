use crate::{
    constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY},
    Value,
};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

const MSG_SUCCESS_LENGTH: u8 = 0x01;
const MSG_SUCCESS_SIGNATURE: u8 = 0x70;
const MSG_SUCCESS_SERIALIZE_LENGTH: usize =
    serialize_length!(MSG_SUCCESS_SIGNATURE, MSG_SUCCESS_LENGTH);

#[derive(Debug, PartialEq)]
pub struct Success {
    metadata: HashMap<String, Value>,
}

impl Success {
    fn new(metadata: HashMap<String, Value>) -> Self {
        Self { metadata }
    }
}

impl ser::Serialize for Success {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, MSG_SUCCESS_SERIALIZE_LENGTH)?;
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
        formatter.write_str("Success message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_SUCCESS_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                });
                let mut fields: Vec<HashMap<String, Value>> = map_access.next_value()?;
                if fields.len() != MSG_SUCCESS_LENGTH as usize {
                    return Err(<V::Error as ::serde::de::Error>::custom(format!(
                        "Expected fields length to be equal {}. Got {} instead.",
                        MSG_SUCCESS_LENGTH,
                        fields.len(),
                    )));
                }
                let metadata = fields.pop().expect("Field element to exist");
                access_check!(map_access, {
                    key(),
                });
                Ok(Success { metadata })
            }
            Some(key) => unexpected_key_access!(key),
            None => unexpected_key_access!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_bytes, to_bytes};

    const BYTES: &[u8] = &[
        0xB1, 0x70, 0xA1, 0x86, 0x66, 0x69, 0x65, 0x6C, 0x64, 0x73, 0x92, 0x84, 0x6E, 0x61, 0x6D,
        0x65, 0x83, 0x61, 0x67, 0x65,
    ];

    fn get_metadata() -> HashMap<String, Value> {
        let mut metadata = HashMap::new();
        metadata.insert(
            String::from("fields"),
            Value::List(vec![
                Value::String(String::from("name")),
                Value::String(String::from("age")),
            ]),
        );
        metadata
    }

    #[test]
    fn serialize() {
        let value = Success::new(get_metadata());
        let result = to_bytes(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<Success>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Success::new(get_metadata()));
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<Success>(&BYTES[0..(BYTES.len() - 1)]);
        assert!(result.is_err());
    }
}
