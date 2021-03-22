use crate::{
    constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY},
    Value,
};
use serde::{
    de::{self, Error},
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

const MSG_RECORD_SIGNATURE: u8 = 0x71;
const MSG_RECORD_LENGTH: u8 = 0x01;
const MSG_RECORD_SERIALIZE_LENGTH: usize =
    serialize_length!(MSG_RECORD_SIGNATURE, MSG_RECORD_LENGTH);

#[derive(Debug, PartialEq)]
pub struct Record {
    fields: Vec<Value>,
}

impl Record {
    fn new(fields: Vec<Value>) -> Self {
        Self { fields }
    }
}

impl ser::Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, MSG_RECORD_SERIALIZE_LENGTH)?;
        ts_serializer.serialize_field(&self.fields)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Record {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(RecordVisitor)
    }
}

struct RecordVisitor;

impl<'de> de::Visitor<'de> for RecordVisitor {
    type Value = Record;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Record message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_RECORD_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                });

                let mut fields: Vec<Vec<Value>> = map_access.next_value()?;
                if fields.len() != 1 {
                    return Err(V::Error::custom(format!(
                        "Expected fields length to be equal 1. Got {} instead",
                        fields.len()
                    )));
                }
                access_check!(map_access, {
                    key(),
                });
                Ok(Record {
                    fields: fields.pop().unwrap(),
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

    const BYTES: &'static [u8] = &[0x0B1, 0x071, 0x093, 0x001, 0x002, 0x003];

    fn get_fields() -> Vec<Value> {
        vec![Value::I64(1), Value::I64(2), Value::I64(3)]
    }

    #[test]
    fn serialize() {
        let value = Record::new(get_fields());
        let result = to_bytes(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<Record>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Record::new(get_fields()));
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<Record>(&BYTES[0..(BYTES.len() - 1)]);
        assert!(result.is_err());
    }
}
