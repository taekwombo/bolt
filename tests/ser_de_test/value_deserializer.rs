use super::*;
use serde_bolt::from_value;

fn de<T>(value: Value, expected: T)
where
    T: for<'de> Deserialize<'de> + PartialEq + Debug,
{
    let result = from_value::<T>(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn unit() {
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
    vec![std::i64::MIN, std::i64::MAX, 0, 127, -128]
        .into_iter()
        .for_each(|num| {
            de(Value::I64(num), num);
        })
}

#[test]
fn f64() {
    de(Value::F64(0.0), 0.0);
    de(Value::F64(std::f64::MAX), std::f64::MAX);
    de(Value::F64(std::f64::MIN), std::f64::MIN);
}

#[test]
fn string() {
    de(Value::String(String::from("test")), String::from("test"));
}

#[test]
fn list() {
    de(Value::List(vec![Value::Null, Value::Null]), vec![(), ()]);
}

#[test]
fn map() {
    de(
        Value::Map(map! {
            "test" => Value::Null,
        }),
        map! {
            "test" => ()
        },
    );
}

#[test]
fn bytes() {
    use serde_bytes::ByteBuf;

    de(Value::Bytes(ByteBuf::from(vec![0])), ByteBuf::from(vec![0]));

    #[derive(serde_derive::Deserialize, Debug, PartialEq)]
    struct NewType(#[serde(with = "serde_bytes")] Vec<u8>);

    de(Value::Bytes(ByteBuf::from(vec![0])), NewType(vec![0]));
}

#[test]
fn r#enum() {
    #[derive(serde_derive::Deserialize, Debug, PartialEq)]
    enum Test {
        Unit,
        Tuple(u8, u8),
        NewType(u8),
        Struct { test: String },
    }

    de(Value::Map(map! { "Unit" => Value::Null }), Test::Unit);
    de(
        Value::Map(map! { "Tuple" => Value::List(vec![Value::I64(0), Value::I64(0)]) }),
        Test::Tuple(0, 0),
    );
    de(
        Value::Map(map! { "NewType" => Value::I64(0) }),
        Test::NewType(0),
    );
    de(
        Value::Map(
            map! { "Struct" => Value::Map(map!{ "test" => Value::String(String::from("test")) }) },
        ),
        Test::Struct {
            test: String::from("test"),
        },
    );
}

#[test]
fn r#struct() {
    #[derive(serde_derive::Deserialize, Debug, PartialEq)]
    struct Test {
        prop: u8,
    }

    de(
        Value::Map(map! { "prop" => Value::I64(100) }),
        Test { prop: 100 },
    );
}
