use crate::{
    constants::{message, STRUCTURE_NAME},
    error::{PackstreamError, PackstreamResult},
    packstream::PackstreamStructure,
    Value,
};
use serde::{
    de, forward_to_deserialize_any,
    ser::{self, SerializeTupleStruct},
};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
pub struct Init {
    pub client: String,
    pub auth: BasicAuth,
}

impl PackstreamStructure for Init {
    const SIG: u8 = message::INIT;
    const LEN: u8 = 0x02;
    const SERIALIZE_LEN: usize = serialize_length!(Self::SIG, Self::LEN);

    type Fields = (String, BasicAuth);

    fn into_value(self) -> Value {
        value_map! {
            "client" => Value::String(self.client),
            "auth" => self.auth.into_value(),
        }
    }
}

impl fmt::Display for Init {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Init")
            .field("client", &self.client)
            .field("auth", &self.auth)
            .finish()
    }
}

#[derive(PartialEq, Deserialize, Serialize)]
pub struct BasicAuth {
    pub scheme: String,
    pub principal: String,
    pub credentials: String,
}

impl BasicAuth {
    fn into_value(self) -> Value {
        let mut map = HashMap::new();
        map.insert(String::from("scheme"), Value::String(self.scheme));
        map.insert(String::from("principal"), Value::String(self.principal));
        map.insert(String::from("credentials"), Value::String(self.credentials));
        Value::Map(map)
    }
}

impl fmt::Debug for BasicAuth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BasicAuth")
            .field("scheme", &self.scheme)
            .field("principal", &self.principal)
            .field("credentials", &"...")
            .finish()
    }
}

impl fmt::Display for BasicAuth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BasicAuth")
            .field("scheme", &self.scheme)
            .field("principal", &self.principal)
            .field("credentials", &"...")
            .finish()
    }
}

impl<'a> ser::Serialize for Init {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, Self::SERIALIZE_LEN)?;
        ts_serializer.serialize_field(&self.client)?;
        ts_serializer.serialize_field(&self.auth)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Init {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(InitVisitor)
    }
}

struct InitVisitor;

impl<'de> de::Visitor<'de> for InitVisitor {
    type Value = Init;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Init")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let (client, auth) = structure_access!(map_access, Init);
        Ok(Init { client, auth })
    }
}

impl<'de> de::Deserializer<'de> for Init {
    type Error = PackstreamError;

    fn deserialize_any<V>(self, visitor: V) -> PackstreamResult<V::Value>
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
