use super::super::BoltStructure;
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Record {
    fields: Vec<Value>,
}

impl BoltStructure for Record {
    const SIG: u8 = 0x71;
    const LEN: u8 = 0x01;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Vec<Vec<Value>>;
}

impl ser::Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
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
        formatter.write_str("Record")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let mut fields = structure_access!(map_access, Record, fields(1));
        Ok(Record {
            // TODO(@krnik): Instead of panic send the error to the consumer
            fields: fields.pop().expect("Fields to have one element"),
        })
    }
}

#[cfg(test)]
mod test_record {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, from_bytes, test, to_bytes};

    const BYTES: &[u8] = &[0xB1, 0x71, 0x93, 0x01, 0x02, 0x03];

    fn get_record() -> Record {
        Record {
            fields: vec![Value::I64(1), Value::I64(2), Value::I64(3)],
        }
    }

    #[test]
    fn serialize() {
        test::ser(&get_record(), BYTES);
    }

    #[test]
    fn deserialize() {
        test::de(&get_record(), BYTES);
    }

    #[test]
    fn deserialize_fail() {
        test::de_err::<Record>(&BYTES[0..(BYTES.len() - 1)]);
        test::de_err::<Record>(&[TINY_STRUCT, Record::SIG + 1]);
        test::de_err::<Record>(&[TINY_STRUCT, Record::SIG, 0]);
    }
}
