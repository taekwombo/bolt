use super::{BoltStructure, Single};
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub fields: Vec<Value>,
}

impl BoltStructure for Record {
    const SIG: u8 = 0x71;
    const LEN: u8 = 0x01;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = Single<Vec<Value>>;
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Record").field(&self.fields).finish()
    }
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
        let fields = structure_access!(map_access, Record);
        Ok(Record {
            fields: fields.value(),
        })
    }
}

#[cfg(test)]
mod test_record {
    use super::*;
    use crate::{
        constants::marker::{TINY_LIST, TINY_STRUCT},
        test,
    };

    const BYTES: &[u8] = &[TINY_STRUCT + Record::LEN, Record::SIG, TINY_LIST + 1, 1];

    #[test]
    fn bytes() {
        test::ser_de::<Record>(BYTES);
        test::de_ser(Record { fields: Vec::new() });
        test::de_err::<Record>(&BYTES[0..(BYTES.len() - 1)]);
    }
}
