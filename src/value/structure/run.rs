use super::BoltStructure;
use crate::{constants::STRUCTURE_NAME, Value};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct Run {
    pub statement: String,
    pub parameters: HashMap<String, Value>,
}

impl BoltStructure for Run {
    const SIG: u8 = 0x10;
    const LEN: u8 = 0x02;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (String, HashMap<String, Value>);
}

impl fmt::Display for Run {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Run")
            .field("statement", &self.statement)
            .field("parameters", &self.parameters)
            .finish()
    }
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
    use crate::{
        constants::marker::{TINY_MAP, TINY_STRING, TINY_STRUCT},
        test,
    };

    const BYTES: &[u8] = &[TINY_STRUCT + Run::LEN, Run::SIG, TINY_STRING, TINY_MAP];

    #[test]
    fn bytes() {
        test::ser_de::<Run>(BYTES);
        test::de_ser(Run {
            statement: String::new(),
            parameters: HashMap::new(),
        });
        test::de_err::<Run>(&BYTES[0..(BYTES.len() - 1)]);
    }
}
