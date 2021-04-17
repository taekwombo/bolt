use serde::{Deserialize, Serialize};
use serde_bolt::constants::marker;
use serde_bolt::{from_bytes, to_bytes, Value};
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

fn ser_err<T>(value: T)
where
    T: Serialize,
{
    let result = to_bytes(&value);
    assert!(result.is_err());
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

pub fn de_err<'de, D>(bytes: &'de [u8])
where
    D: Deserialize<'de> + Debug + PartialEq,
{
    let result = from_bytes::<'de, D>(bytes);
    assert!(result.is_err());
}

mod ser_de_test;
