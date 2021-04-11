use crate::constants::{STRUCTURE_FIELDS_KEY, STRUCTURE_SIG_KEY};
use crate::error::{SerdeError, SerdeResult};
use serde::{de, forward_to_deserialize_any};
use std::borrow::Cow;
use std::boxed::Box;

#[derive(Debug, Copy, Clone)]
pub enum StructureStateDe {
    Signature,
    Fields,
    Done,
}

impl<'de> de::Deserializer<'de> for StructureStateDe {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            StructureStateDe::Signature => visitor.visit_borrowed_str(STRUCTURE_SIG_KEY),
            StructureStateDe::Fields => visitor.visit_borrowed_str(STRUCTURE_FIELDS_KEY),
            StructureStateDe::Done => Err(Self::Error::create(
                "Cannot deserialize StructureStateDe::Done",
            )),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier enum ignored_any
    }
}

#[derive(Debug)]
pub struct StringDe<'a> {
    value: Cow<'a, str>,
}

impl<'a> StringDe<'a> {
    pub(crate) fn new(value: impl Into<Cow<'a, str>>) -> Self {
        Self { value: value.into() }
    }
}

impl<'de, T> From<T> for StringDe<'de>
where
    T: Into<Cow<'de, str>>
{
    fn from(value: T) -> Self {
        Self { value: value.into() }
    }
}

impl<'de> de::Deserializer<'de> for StringDe<'de> {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
            Cow::Owned(s) => visitor.visit_string(s),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier enum ignored_any
    }
}

#[derive(Debug)]
pub struct NumberDe {
    value: i64
}

impl<T> From<T> for NumberDe 
where i64: From<T>
{
    fn from(value: T) -> Self {
        Self { value: i64::from(value) }
    }
}

impl<'de> de::Deserializer<'de> for NumberDe {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_i64(self.value)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier enum ignored_any
    }
}

#[derive(Debug)]
pub struct MapDe<T> {
    value: T
}

impl<'de, T> de::Deserializer<'de> for MapDe<T> {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Visitor>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_map(self.value)
    }
}

pub struct ListDe<T> {
    value: Vec<T>
}

impl<'de, T> de::Deserializer<'de> for List<T> {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_seq(self.value)
    }
}

pub enum ValueDe<'de> {
    String(StringDe<'de>),
    Number(NumberDe),
}

impl<'de> ValueDe<'de> {
    pub fn string (value: impl Into<StringDe<'de>>) -> Self {
        Self::String(value.into())
    }

    pub fn number (value: impl Into<NumberDe>) -> Self {
        Self::Number(value.into())
    }
}

impl<'de> de::Deserializer<'de> for ValueDe<'de> {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> SerdeResult<V::Value>
    where
        V: de::Visitor<'de>
    {
        match self {
            ValueDe::String(de) => de.deserialize_any(visitor),
            ValueDe::Number(de) => de.deserialize_any(visitor),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier enum ignored_any
    }
}

type KVField<'a> = (StringDe<'a>, ValueDe<'a>);

pub struct StructureAccess<'a> {
    fields: <Vec<KVField<'a>> as IntoIterator>::IntoIter,
    value: Option<ValueDe<'a>>,
}

impl<'a> StructureAccess<'a> {
    pub fn new (fields: Vec<KVField<'a>>) -> Self {
        Self {
            fields: fields.into_iter(),
            value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for StructureAccess<'de> {
    type Error = SerdeError;

    fn next_key_seed<K>(&mut self, seed: K) -> SerdeResult<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>
    {
        match self.fields.next() {
            Some((key, value)) => {
                self.value.replace(value);
                seed.deserialize(key).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> SerdeResult<V::Value>
    where
        V: de::DeserializeSeed<'de>
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(Self::Error::create("Expected value to exist"))
        }
    }
}

#[cfg(test)]
mod test_deserializer {
    use super::*;

    #[test]
    fn serer () {
        let s: SerdeResult<String> = de::Deserialize::deserialize(ValueDe::String(StringDe::new("ASDASD")));

        println!("{:?}", s);
    }

    #[test]
    fn deser_test() {
        let s: SerdeResult<String> =
            de::Deserialize::deserialize(StructureStateDe::Signature);
        let p: SerdeResult<String> =
            de::Deserialize::deserialize(StructureStateDe::Fields);
        let r: SerdeResult<String> = de::Deserialize::deserialize(StructureStateDe::Done);
        println!("{:?}", s);
        println!("{:?}", p);
        println!("{:?}", r);
    }
}
