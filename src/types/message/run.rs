use crate::{
    constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY},
    Value,
};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

const MSG_RUN_LENGTH: u8 = 0x02;
const MSG_RUN_SIGNATURE: u8 = 0x10;
const MSG_RUN_SERIALIZE_LENGTH: usize = serialize_length!(MSG_RUN_SIGNATURE, MSG_RUN_LENGTH);

#[derive(Debug, PartialEq)]
pub struct Run<'a> {
    statement: &'a str,
    parameters: HashMap<&'a str, Value>,
}

impl<'a> Run<'a> {
    fn new(statement: &'a str, parameters: HashMap<&'a str, Value>) -> Self {
        Self {
            statement,
            parameters,
        }
    }
}

impl<'a> ser::Serialize for Run<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, MSG_RUN_SERIALIZE_LENGTH)?;
        ts_serializer.serialize_field(&self.statement)?;
        ts_serializer.serialize_field(&self.parameters)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Run<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(RunVisitor)
    }
}

struct RunVisitor;

impl<'de> de::Visitor<'de> for RunVisitor {
    type Value = Run<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Run message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_RUN_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                });
                let fields: (&str, HashMap<&str, Value>) = map_access.next_value()?;
                access_check!(map_access, {
                    key(),
                });
                Ok(Run {
                    statement: fields.0,
                    parameters: fields.1,
                })
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

    const BYTES: &'static [u8] = &[
        0xB2, 0x10, 0x8F, 0x52, 0x45, 0x54, 0x55, 0x52, 0x4E, 0x20, 0x31, 0x20, 0x41, 0x53, 0x20,
        0x6E, 0x75, 0x6D, 0xA0,
    ];

    const STATEMENT: &'static str = "RETURN 1 AS num";

    #[test]
    fn serialize() {
        let value = Run::new(STATEMENT, HashMap::new());
        let result = to_bytes(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<Run<'_>>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Run::new(STATEMENT, HashMap::new()));
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<Run<'_>>(&BYTES[0..(BYTES.len() - 1)]);
        assert!(result.is_err());
    }
}
