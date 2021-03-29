use crate::constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_NAME, STRUCTURE_SIG_KEY};
use serde::{
    de,
    ser::{self, SerializeTupleStruct},
};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

const MSG_INIT_LENGTH: u8 = 0x02;
const MSG_INIT_SIGNATURE: u8 = 0x01;
const MSG_INIT_SERIALIZE_LENGTH: usize = serialize_length!(MSG_INIT_SIGNATURE, MSG_INIT_LENGTH);

#[derive(Debug, PartialEq)]
pub struct Init<'a> {
    client: &'a str,
    auth: BasicAuth<'a>,
}

impl<'a> Init<'a> {
    pub fn new(client: &'a str, user: &'a str, password: &'a str) -> Self {
        Self {
            client,
            auth: BasicAuth::new(user, password),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct BasicAuth<'a> {
    scheme: &'a str,
    principal: &'a str,
    credentials: &'a str,
}

impl<'a> BasicAuth<'a> {
    pub fn new(user: &'a str, password: &'a str) -> Self {
        Self {
            scheme: "basic",
            principal: user,
            credentials: password,
        }
    }
}

impl<'a> ser::Serialize for Init<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut ts_serializer =
            serializer.serialize_tuple_struct(STRUCTURE_NAME, MSG_INIT_SERIALIZE_LENGTH)?;
        ts_serializer.serialize_field(&self.client)?;
        ts_serializer.serialize_field(&self.auth)?;
        ts_serializer.end()
    }
}

impl<'de> de::Deserialize<'de> for Init<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(InitVisitor)
    }
}

struct InitVisitor;

impl<'de> de::Visitor<'de> for InitVisitor {
    type Value = Init<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Init message")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                access_check!(map_access, {
                    signature(MSG_INIT_SIGNATURE),
                    key(STRUCTURE_FIELDS_KEY),
                });

                let fields: (&str, BasicAuth) = map_access.next_value()?;
                access_check!(map_access, {
                    key(),
                });
                Ok(Init {
                    client: fields.0,
                    auth: fields.1,
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
        0xB2, 0x01, 0x8C, 0x4D, 0x79, 0x43, 0x6C, 0x69, 0x65, 0x6E, 0x74, 0x2F, 0x31, 0x2E, 0x30,
        0xA3, 0x86, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x65, 0x85, 0x62, 0x61, 0x73, 0x69, 0x63, 0x89,
        0x70, 0x72, 0x69, 0x6E, 0x63, 0x69, 0x70, 0x61, 0x6C, 0x85, 0x6E, 0x65, 0x6F, 0x34, 0x6A,
        0x8B, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6E, 0x74, 0x69, 0x61, 0x6C, 0x73, 0x86, 0x73, 0x65,
        0x63, 0x72, 0x65, 0x74,
    ];

    const CLIENT_NAME: &'static str = "MyClient/1.0";
    const USER: &'static str = "neo4j";
    const PASSWORD: &'static str = "secret";

    #[test]
    fn serialize() {
        let value = Init::new(CLIENT_NAME, USER, PASSWORD);
        let result = to_bytes(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BYTES);
    }

    #[test]
    fn deserialize() {
        let result = from_bytes::<Init<'_>>(BYTES);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Init::new(CLIENT_NAME, USER, PASSWORD));
    }

    #[test]
    fn deserialize_fail() {
        let result = from_bytes::<Init<'_>>(&BYTES[0..(BYTES.len() - 1)]);
        assert!(result.is_err());
    }
}
