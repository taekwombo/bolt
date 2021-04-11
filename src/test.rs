use crate::{from_bytes, to_bytes};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub fn ser<S: Serialize>(value: &S, bytes: &[u8]) {
    let result = to_bytes(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), bytes);
}

pub fn de<'de, D>(expected: &D, bytes: &'de [u8])
where
    D: Deserialize<'de> + Debug + PartialEq,
{
    let result = from_bytes::<'de, D>(bytes);
    println!("{:?}", result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), *expected);
}

pub fn de_err<'de, D>(bytes: &'de [u8])
where
    D: Deserialize<'de> + Debug + PartialEq,
{
    let result = from_bytes::<'de, D>(bytes);
    assert!(result.is_err());
}

// Note on $crate - https://doc.rust-lang.org/1.5.0/book/macros.html#the-variable-crate

macro_rules! assert_ser_de {
    ($($value:expr),* $(,)*) => {
        $(
            assert_eq!($value, $crate::from_bytes(&$crate::to_bytes(&$value).unwrap()).unwrap());
        )*
    }
}

// TODO(@krnik) simplify macro calls
macro_rules! assert_ser {
    (ok $var:ident $bytes:expr) => {
        if $var.is_err() {
            eprintln!("{:?}", $var);
        }
        assert!($var.is_ok());
        assert_eq!($var.unwrap(), $bytes);
    };
    (err $var:ident $error:expr) => {
        assert!($var.is_err());
        assert_eq!($var.unwrap_err(), $error);
    };
    ($($ok_err:tt {$($value:expr => $expected:expr),* $(,)*})+) => {
        $(
            $(
               let __v = $crate::to_bytes(&$value);
               assert_ser!($ok_err __v $expected);
            )*
        )+
    }
}

// TODO(@krnik): Simplify assert_de macro calls
macro_rules! assert_de {
    (ok $var:ident $value:expr) => {
        if $var.is_err() {
            eprintln!("{:?}", $var);
        }
        assert!($var.is_ok());
        assert_eq!($var.unwrap(), $value);
    };
    (err $var:ident $value:expr) => {
        assert!($var.is_err());
        assert_eq!($var.unwrap_err(), $value);
    };
    ($($ok_err:tt with $method:ident into $t:ty {$($source:expr => $value:expr),* $(,)*})+) => {
        $(
            $(
                let __v = $crate::$method::<$t>($source);
                assert_de!($ok_err __v $value);
            )*
        )+
    };
}

macro_rules! bytes {
    ($($slice:expr),* $(,)*) => {
        {
            let mut __arr = Vec::new();
            $(__arr.extend_from_slice(&$slice);)*
            __arr
        }
    }
}

macro_rules! map {
    ($($key:literal: $value:expr),* $(,)*) => {
        {
            let mut __map = HashMap::new();
            $(__map.insert($key, $value);)*
            __map
        }
    }
}
