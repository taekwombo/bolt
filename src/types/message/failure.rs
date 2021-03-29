use crate::{
    constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY},
    Value,
};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

const MSG_FAILURE_SIGNATURE: u8 = 0x7F;
const MSG_FAILURE_LENGTH: u8 = 0x01;
const MSG_FAILURE_SERIALIZE_LENGTH: usize =
    serialize_length!(MSG_FAILURE_SIGNATURE, MSG_FAILURE_LENGTH);

#[derive(Debug, PartialEq)]
struct Failure {
    metadata: HashMap<String, Value>,
}

impl ser::Serialize for Failure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, MSG_FAILURE_SERIALIZE_LENGTH)?;
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
        formatter.write_str("Failure message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_FAILURE_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                });
                let mut fields: Vec<HashMap<String, Value>> = map_access.next_value()?;
                if fields.len() != MSG_FAILURE_LENGTH as usize {
                    return Err(<V::Error as ::serde::de::Error>::custom(format!(
                        "Expected fields length to be equal {}. Got {} instead.",
                        MSG_FAILURE_LENGTH,
                        fields.len(),
                    )));
                }
                let metadata = fields.pop().expect("Field element to exist");
                access_check!(map_access, {
                    key(),
                });
                Ok(Failure { metadata })
            }
            Some(key) => unexpected_key_access!(key),
            None => unexpected_key_access!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::marker::{TINY_MAP, TINY_STRUCT},
        from_bytes, to_bytes,
    };

    const BYTES: &[u8] = &[TINY_STRUCT + 1, MSG_FAILURE_SIGNATURE, TINY_MAP];

    #[test]
    fn serialize() {
        let value = Failure {
            metadata: HashMap::new(),
        };
        let result = to_bytes(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<Failure>(BYTES);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Failure {
                metadata: HashMap::new()
            }
        );
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<Failure>(&[TINY_STRUCT, MSG_FAILURE_SIGNATURE + 1]);
        assert!(result.is_err());
    }
}
