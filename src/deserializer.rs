use crate::error::{SerdeError, SerdeResult};
use serde::{de, forward_to_deserialize_any};
use std::borrow::Cow;

#[derive(Debug)]
pub struct StringDe<'a> {
    value: Cow<'a, str>,
}

impl<'a> StringDe<'a> {
    pub(crate) fn new(value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl<'de, T> From<T> for StringDe<'de>
where
    T: Into<Cow<'de, str>>,
{
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
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
