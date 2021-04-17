use super::super::Value;
use serde::{
    de::{self, Deserialize, Deserializer, Error},
    ser::{self, Serialize, SerializeTuple},
};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct Single<T>(pub T);

impl<T> Single<T> {
    pub(crate) fn value(self) -> T {
        self.0
    }
}

impl<T> Serialize for Single<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut t_serializer = serializer.serialize_tuple(1)?;
        t_serializer.serialize_element(&self.0)?;
        t_serializer.end()
    }
}

impl<'de, T: 'de> Deserialize<'de> for Single<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SingleVisitor::new())
    }
}

struct SingleVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> SingleVisitor<T> {
    fn new() -> Self {
        SingleVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, T> de::Visitor<'de> for SingleVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Single<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("One-element strucutre fields")
    }

    fn visit_seq<V>(self, mut seq_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        match seq_access.next_element::<T>()? {
            Some(elem) => {
                if seq_access.next_element::<T>()?.is_some() {
                    return Err(V::Error::custom(
                        "Expected structure fields to have exactly one element, got more",
                    ));
                }
                Ok(Single(elem))
            }
            None => Err(V::Error::custom(
                "Expected structure fields to have one element",
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Empty;

impl ser::Serialize for Empty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_tuple(0)?.end()
    }
}

impl<'de> Deserialize<'de> for Empty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(EmptyVisitor)
    }
}

struct EmptyVisitor;

impl<'de> de::Visitor<'de> for EmptyVisitor {
    type Value = Empty;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Empty structure fields")
    }

    fn visit_seq<V>(self, mut seq_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        match seq_access.next_element::<Value>()? {
            Some(elem) => Err(V::Error::custom(format!(
                "Expected empty structure fields, instead got element {}",
                elem
            ))),
            None => Ok(Empty),
        }
    }
}
