use super::Value;
use serde::de;
use serde_bytes::ByteBuf;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

struct ValueVisitor;

impl<'de> de::Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("any valid Bolt value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E> {
        Ok(Value::Bytes(ByteBuf::from(bytes)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Value::I64(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Value::I64(i64::try_from(value).unwrap()))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
        Ok(Value::F64(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Value::String(String::from(value)))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<V>(self, mut seq_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        let mut list = Vec::new();
        while let Some(elem) = seq_access.next_element()? {
            list.push(elem);
        }
        Ok(Value::List(list))
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let mut map = HashMap::new();
        while let Some(key) = map_access.next_key()? {
            map.insert(key, map_access.next_value()?);
        }
        Ok(Value::Map(map))
    }
}

impl<'de> de::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de::from_bytes;
    use crate::marker_bytes::*;

    #[test]
    fn deserialize_value() {
        assert!(from_bytes::<Value>(&[NULL]).is_ok());
        assert!(from_bytes::<Value>(&[127]).is_ok());
        assert!(from_bytes::<Value>(&[TINY_STRING + 1, 49]).is_ok());
        assert!(from_bytes::<Value>(&[FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0]).is_ok());
        assert!(from_bytes::<Value>(&[TINY_LIST + 3, 0, 1, 0]).is_ok());
        assert!(from_bytes::<Value>(&[TINY_MAP]).is_ok());
        assert!(from_bytes::<Value>(&[BYTES_8, 1, 1]).is_ok());
    }
}
