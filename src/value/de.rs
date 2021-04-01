use super::Value;
use crate::constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_SIG_KEY};
use crate::error::{Error, SerdeResult};
use serde::de::IntoDeserializer;
use serde::{de, forward_to_deserialize_any};
use serde_bytes::ByteBuf;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

pub fn from_value<T>(value: Value) -> SerdeResult<T>
where
    T: de::DeserializeOwned,
{
    T::deserialize(value)
}

mod errors {
    use super::{Error, Value};

    pub(super) fn unexpected_type(expected: &str, actual: &Value) -> Error {
        Error::create(format!(
            "Unexpected type {}, expected {}.",
            actual, expected
        ))
    }
}

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
        match map_access.next_key::<&str>()? {
            Some(key) if key == STRUCTURE_SIG_KEY => {
                let signature: u8 = map_access.next_value::<u8>()?;
                access_check!(map_access, {
                    key(STRUCTURE_FIELDS_KEY),
                });
                let fields: Vec<Value> = map_access.next_value()?;
                access_check!(map_access, {
                    key(),
                });
                Ok(Self::Value::Structure { signature, fields })
            }
            Some(key) => {
                let mut map: HashMap<String, Value> = HashMap::new();
                map.insert(String::from(key), map_access.next_value()?);
                while let Some(key) = map_access.next_key::<String>()? {
                    map.insert(key, map_access.next_value()?);
                }
                Ok(Value::Map(map))
            }
            // TODO(@krnik): Why does this produce empty map??
            None => Ok(Self::Value::Map(HashMap::new())),
        }
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

impl<'de> de::Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Null => visitor.visit_unit(),
            Self::Bool(b) => visitor.visit_bool(b),
            Self::I64(i) => visitor.visit_i64(i),
            Self::F64(f) => visitor.visit_f64(f),
            Self::String(s) => visitor.visit_string(s),
            Self::List(_) => self.deserialize_seq(visitor),
            Self::Map(_) => self.deserialize_map(visitor),
            Self::Bytes(b) => visitor.visit_byte_buf(b.to_vec()),
            Self::Structure { signature, fields } => {
                visitor.visit_map(StructureDeserializer::new(signature, fields))
            }
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::I64(i) => {
                visitor.visit_u64(u64::try_from(i).map_err(|e| Error::create(e.to_string()))?)
            }
            v => Err(errors::unexpected_type("Value::I64", &v)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::I64(i) => visitor.visit_i64(i),
            v => Err(errors::unexpected_type("Value::I64", &v)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::F64(f) => visitor.visit_f64(f),
            v => Err(errors::unexpected_type("Value::F64", &v)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Bool(b) => visitor.visit_bool(b),
            v => Err(errors::unexpected_type("Value::Bool", &v)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::String(s) => visitor.visit_str(s.as_str()),
            v => Err(errors::unexpected_type("Value::String", &v)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::String(s) => visitor.visit_string(s),
            v => Err(errors::unexpected_type("Value::String", &v)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Bytes(b) => visitor.visit_byte_buf(b.to_vec()),
            v => Err(errors::unexpected_type("Value::Bytes", &v)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Null => self.deserialize_unit(visitor),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Null => visitor.visit_unit(),
            v => Err(errors::unexpected_type("Value::Null", &v)),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::List(vec) => visitor.visit_seq(SeqDeserializer {
                iter: vec.into_iter(),
            }),
            v => Err(errors::unexpected_type("Value::List", &v)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // TODO(@krnik): Should it compare tuple size with Value::List size?
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // TODO(@krnik): Should it compare tuple size with Value::List size?
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Map(m) => visitor.visit_map(MapAccess {
                iter: m.into_iter(),
                value: None,
            }),
            Self::Structure { signature, fields } => {
                visitor.visit_map(StructureDeserializer::new(signature, fields))
            }
            v => Err(errors::unexpected_type("Value::Map", &v)),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _keys: &'static [&'static str],
        visitor: V,
    ) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let (variant, value) = match self {
            Self::Map(m) => {
                let mut iter = m.into_iter();
                let res = match iter.next() {
                    None => {
                        return Err(Error::create(
                            "Expected exactly 1 key for enum deserialization",
                        ))
                    }
                    Some(tp) => tp,
                };
                if iter.next().is_some() {
                    return Err(Error::create(
                        "Expected exactly 1 key for enum deserialization",
                    ));
                }
                res
            }
            _ => return Err(Error::create("Map expected for enum deserializaton")),
        };
        visitor.visit_enum(EnumAccess {
            variant,
            value: Some(value),
        })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        drop(self);
        visitor.visit_unit()
    }
}

struct SeqDeserializer {
    iter: std::vec::IntoIter<Value>,
}

impl<'de> de::Deserializer<'de> for SeqDeserializer {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let v = visitor.visit_seq(&mut self)?;
        if self.iter.len() != 0 {
            return Err(Error::create(
                "Value::List must have all of its elements deserialized",
            ));
        }
        Ok(v)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> de::SeqAccess<'de> for SeqDeserializer {
    type Error = Error;

    fn next_element_seed<S>(&mut self, seed: S) -> SerdeResult<Option<S::Value>>
    where
        S: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            None => Ok(None),
            Some(v) => seed.deserialize(v).map(Some),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (upper, Some(lower)) if upper == lower => Some(upper),
            _ => None,
        }
    }
}

struct MapKeyDeserializer {
    key: String,
}

impl<'de> de::Deserializer<'de> for MapKeyDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.key)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct MapAccess {
    iter: <std::collections::HashMap<String, Value> as IntoIterator>::IntoIter,
    value: Option<Value>,
}

impl<'de> de::MapAccess<'de> for MapAccess {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> SerdeResult<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            None => Ok(None),
            Some((key, val)) => {
                self.value = Some(val);
                seed.deserialize(MapKeyDeserializer { key }).map(Some)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> SerdeResult<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            None => Err(Error::create("Value is missing")),
            Some(v) => seed.deserialize(v),
        }
    }
}

#[derive(Debug)]
enum StructureDeserializerState {
    Signature,
    Fields,
    Done,
}

struct StructureDeserializer {
    signature: u8,
    fields: Option<Vec<Value>>,
    state: StructureDeserializerState,
}

impl StructureDeserializer {
    fn new(signature: u8, fields: Vec<Value>) -> Self {
        Self {
            signature,
            fields: Some(fields),
            state: StructureDeserializerState::Signature,
        }
    }
}

impl<'de> de::MapAccess<'de> for StructureDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> SerdeResult<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        println!("{:?}", self.state);
        match self.state {
            StructureDeserializerState::Signature => seed
                .deserialize(MapKeyDeserializer {
                    key: String::from("signature"),
                })
                .map(Some),
            StructureDeserializerState::Fields => seed
                .deserialize(MapKeyDeserializer {
                    key: String::from("fields"),
                })
                .map(Some),
            StructureDeserializerState::Done => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> SerdeResult<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.state {
            StructureDeserializerState::Signature => {
                self.state = StructureDeserializerState::Fields;
                seed.deserialize(self.signature.into_deserializer())
            }
            StructureDeserializerState::Fields => {
                self.state = StructureDeserializerState::Done;
                seed.deserialize(SeqDeserializer {
                    iter: self.fields.take().expect("Value to exist").into_iter(),
                })
            }
            StructureDeserializerState::Done => unreachable!(),
        }
    }
}

struct EnumAccess {
    variant: String,
    value: Option<Value>,
}

impl<'de> de::EnumAccess<'de> for EnumAccess {
    type Variant = VariantAccess;
    type Error = Error;

    fn variant_seed<V>(self, seed: V) -> SerdeResult<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let value_visitor = VariantAccess { value: self.value };
        seed.deserialize(variant).map(|v| (v, value_visitor))
    }
}

struct VariantAccess {
    value: Option<Value>,
}

impl<'de> de::VariantAccess<'de> for VariantAccess {
    type Error = Error;

    fn unit_variant(self) -> SerdeResult<()> {
        match self.value {
            Some(value) => de::Deserialize::deserialize(value),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> SerdeResult<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => Err(Error::create(
                "Unexpected unit variant, expected newtype variant.",
            )),
        }
    }

    fn tuple_variant<V>(self, _size: usize, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Some(Value::List(vec)) => visitor.visit_seq(SeqDeserializer {
                iter: vec.into_iter(),
            }),
            Some(other) => Err(errors::unexpected_type(
                "tuple variant (Value::List)",
                &other,
            )),
            None => Err(Error::create(
                "Unexpected unit variant, expected tuple variant.",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Some(Value::Map(map)) => visitor.visit_map(MapAccess {
                iter: map.into_iter(),
                value: None,
            }),
            Some(other) => Err(errors::unexpected_type(
                "struct variant (Value::Map)",
                &other,
            )),
            None => Err(Error::create(
                "Unexpected unit variant, expected struct variant.",
            )),
        }
    }
}
