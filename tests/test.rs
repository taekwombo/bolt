use serde::{Deserialize, Serialize};
use packstream_serde::constants::marker;
use packstream_serde::{from_bytes, to_bytes, Value};
use packstream_serde::packstream::PackstreamStructure;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;

macro_rules! bytes {
    ($($slice:expr),* $(,)*) => {
        {
            let mut __arr: Vec<u8> = Vec::new();
            $(__arr.extend_from_slice(&$slice);)*
            __arr
        }
    }
}

macro_rules! map {
   ($($key:literal => $value:expr),* $(,)*) => {
      {
         let mut __map = std::collections::HashMap::new();
         $(__map.insert(String::from($key), $value);)*
         __map
      }
   }
}

// Serializes the `value` parameter and asserts that its
// equal to the value of `expected` parameter.
fn ser<T>(value: T, expected: &[u8])
where
    T: Serialize,
{
    let bytes = to_bytes(&value);
    if bytes.is_err() {
        eprintln!("{:?}", bytes);
    }
    assert!(bytes.is_ok());
    assert_eq!(expected, bytes.unwrap().as_slice());
}

// Assers that serialization of the `value` parameter
// results in an `Err(_)`
fn ser_err<T>(value: T)
where
    T: Serialize,
{
    let result = to_bytes(&value);
    assert!(result.is_err());
}

// Serializes the `bytes`, then deserializes serialized result
// and compares it to the `bytes` parameter.
pub fn ser_de<'de, T>(bytes: &'de [u8])
where
    T: Deserialize<'de> + Debug + PartialEq + Serialize,
{
    let value = from_bytes::<T>(bytes);
    assert!(value.is_ok());
    let byte_value = to_bytes(&value.expect("To be OK"));
    assert!(byte_value.is_ok());
    assert_eq!(byte_value.expect("To be OK"), bytes);
}

fn de<'de, T>(bytes: &'de [u8], compare: T)
where
    T: Deserialize<'de> + PartialEq + Debug,
{
    let result = from_bytes::<T>(bytes);
    if result.is_err() {
        eprintln!("{:?}", result);
    }
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), compare);
}

// Asserts that the result of deserialization is an Err(_)
pub fn de_err<'de, D>(bytes: &'de [u8])
where
    D: Deserialize<'de> + Debug + PartialEq,
{
    let result = from_bytes::<'de, D>(bytes);
    assert!(result.is_err());
}

// Deserializes the value, then serializes the deserialized result
// and compares it to the `value` parameter.
pub fn de_ser<T>(value: T)
where
    T: for<'de> Deserialize<'de> + Debug + PartialEq + Serialize,
{
    let bytes = to_bytes::<T>(&value);
    assert!(bytes.is_ok());
    let bytes = bytes.expect("To be OK");
    let serialized = from_bytes::<T>(&bytes);
    assert!(serialized.is_ok());
    assert_eq!(serialized.unwrap(), value);
}

mod ser_de_test;
