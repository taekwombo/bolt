use serde::{
    de::{self, Deserialize, Deserializer, Error},
    ser::{self, Serialize, SerializeTuple},
};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Simple<T>(T);

impl<'de, T> Serialize for Simple<T>
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

impl<'de, T: 'de> Deserialize<'de> for Simple<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_seq(SimpleVisitor::new())
    }
}

struct SimpleVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> SimpleVisitor<T> {
    fn new() -> Self {
        SimpleVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, T> de::Visitor<'de> for SimpleVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Simple<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Simple list strucutre")
    }

    fn visit_seq<V>(self, mut seq_access: V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        if let Some(hint) = seq_access.size_hint() {
            if hint != 1 {
                return Err(V::Error::custom(format!(
                    "Expected Simple list structure to have length of 1, instead got {}",
                    hint
                )));
            }
        }

        match seq_access.next_element::<T>()? {
            Some(elem) => {
                if let Some(_) = seq_access.next_element::<T>()? {
                    return Err(V::Error::custom(
                        "Expected Simple structure list to have exactly one element. Got more",
                    ));
                }
                Ok(Simple(elem))
            }
            None => Err(V::Error::custom(
                "Expected Simple structure list to have one element",
            )),
        }
    }
}

#[cfg(test)]
mod test_simple {
    use super::*;
    use crate::{constants::marker::TINY_LIST, from_bytes, test, to_bytes};

    #[test]
    fn simple_test() {
        let result = from_bytes::<Simple<u8>>(&[TINY_LIST + 1, 1]);
        println!("{:?}", result);
    }
}
