use super::*;
use serde_bolt::error::*;
use serde_bolt::Value;

fn de<T>(value: Value, expected: T)
where
    T: for<'de> Deserialize<'de> + PartialEq + Debug,
{
    let t: Result<T, _> = serde::de::Deserialize::deserialize(value);
    assert!(t.is_ok());
    assert_eq!(t.unwrap(), expected);
}

#[test]
fn null() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Unit;

    de(Value::Null, ());
    de(Value::Null, Unit);
}

#[test]
fn bool() {
    de(Value::Bool(true), true);
    de(Value::Bool(false), false);
}

#[test]
fn i64() {
    let integers = vec![0, -128, 127, std::i64::MIN, std::i64::MAX];

    for int in integers {
        let serialized: SerdeResult<i64> = serde::de::Deserialize::deserialize(Value::I64(int));
        println!("{:?}, {}", serialized, int);
    }
}

#[test]
fn f64() {}

#[test]
fn string() {}

#[test]
fn list() {}

#[test]
fn map() {}

#[test]
fn bytes() {}

// Whole value :)
#[test]
fn r#enum() {}

#[test]
fn r#struct() {}
