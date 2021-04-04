use super::super::BoltStructure;
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct Run {
    statement: String,
    parameters: HashMap<String, Value>,
}

impl BoltStructure for Run {
    const SIG: u8 = 0x10;
    const LEN: u8 = 0x02;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (String, HashMap<String, Value>);
}

impl<'a> ser::Serialize for Run {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Run::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.statement)?;
        ts_serializer.serialize_field(&self.parameters)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Run {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(RunVisitor)
    }
}

struct RunVisitor;

impl<'de> de::Visitor<'de> for RunVisitor {
    type Value = Run;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Run")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (statement, parameters) = structure_access!(map_access, Run);
        Ok(Run {
            statement,
            parameters,
        })
    }
}

#[cfg(test)]
mod test_run {
    use super::*;
    use crate::{constants::marker::TINY_STRUCT, from_bytes, test, to_bytes};

    const BYTES: &[u8] = &[
        0xB2, 0x10, 0x8F, 0x52, 0x45, 0x54, 0x55, 0x52, 0x4E, 0x20, 0x31, 0x20, 0x41, 0x53, 0x20,
        0x6E, 0x75, 0x6D, 0xA0,
    ];

    const STATEMENT: &str = "RETURN 1 AS num";

    fn create_run() -> Run {
        Run {
            statement: STATEMENT.to_owned(),
            parameters: HashMap::new(),
        }
    }

    #[test]
    fn serialize() {
        test::ser(&create_run(), BYTES);
    }

    #[test]
    fn deserialize() {
        test::de(&create_run(), BYTES);
    }

    #[test]
    fn deserialize_fail() {
        test::de_err::<Run>(&BYTES[0..(BYTES.len() - 1)]);
        test::de_err::<Run>(&[TINY_STRUCT, Run::SIG + 1]);
    }
}
