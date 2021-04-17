use super::{BoltStructure, Value};
use crate::{
    constants::STRUCTURE_NAME,
    error::{SerdeError, SerdeResult},
};
use serde::{
    de, forward_to_deserialize_any,
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

    fn into_value(self) -> Value {
        value_map! {
            "statement" => Value::String(self.statement),
            "parameters" => Value::Map(self.parameters),
        }
    }
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

impl<'de> de::Deserializer<'de> for Run {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.into_value().deserialize_map(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier enum ignored_any
    }
}
