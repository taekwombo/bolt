use super::*;
use serde::Serialize;
use packstream_serde::to_value;

fn ser<T: PartialEq + Debug + Serialize>(value: &T, expected: &Value) {
    let result = to_value(value);
    assert!(result.is_ok());
    assert_eq!(&result.unwrap(), expected);
}

#[test]
fn unit() {
    ser(&(), &Value::Null);
}

#[test]
fn bool() {
    ser(&true, &Value::Bool(true));
    ser(&false, &Value::Bool(false));
}

#[test]
fn i64() {
    ser(&0, &Value::I64(0));
    ser(&std::i64::MAX, &Value::I64(std::i64::MAX));
    ser(&std::i64::MIN, &Value::I64(std::i64::MIN));
}

#[test]
fn f64() {
    ser(&0.0, &Value::F64(0.0));
}

#[test]
fn string() {
    ser(&"test", &Value::String(String::from("test")));
    ser(&String::from("test"), &Value::String(String::from("test")));
}

#[test]
fn list() {
    ser(&[0], &Value::List(vec![Value::I64(0)]));
    ser(&vec![0], &Value::List(vec![Value::I64(0)]));
}

#[test]
fn map() {
    ser(
        &map! { "test" => 0 },
        &Value::Map(map! { "test" => Value::I64(0) }),
    );
}

#[test]
fn bytes() {
    use serde_bytes::ByteBuf;

    ser(
        &ByteBuf::from(vec![0]),
        &Value::Bytes(ByteBuf::from(vec![0])),
    );

    #[derive(serde_derive::Serialize, Debug, PartialEq)]
    struct NewType(#[serde(with = "serde_bytes")] Vec<u8>);

    ser(&NewType(vec![0]), &Value::Bytes(ByteBuf::from(vec![0])));
}

#[test]
fn r#enum() {
    #[derive(serde_derive::Serialize, Debug, PartialEq)]
    enum Test {
        Unit,
        Tuple(u8, u8),
        NewType(u8),
        Struct { test: String },
    }

    ser(&Test::Unit, &Value::String(String::from("Unit")));
    ser(
        &Test::Tuple(0, 0),
        &Value::Map(map! { "Tuple" => Value::List(vec![Value::I64(0), Value::I64(0)]) }),
    );
    ser(
        &Test::NewType(0),
        &Value::Map(map! { "NewType" => Value::I64(0) }),
    );
    ser(
        &Test::Struct {
            test: String::from("test"),
        },
        &Value::Map(
            map! { "Struct" => Value::Map(map! { "test" => Value::String(String::from("test")) }) },
        ),
    );
}

#[test]
fn r#struct() {
    #[derive(Debug, PartialEq, serde_derive::Serialize)]
    struct TestStruct {
        str_prop: String,
        num_prop: u64,
    }

    ser(
        &TestStruct {
            str_prop: String::from("test"),
            num_prop: 1000,
        },
        &Value::Map(map! {
            "str_prop" => Value::String(String::from("test")),
            "num_prop" => Value::I64(1000),
        }),
    );

    #[derive(Debug, PartialEq, serde_derive::Serialize)]
    struct NewType<T: Debug + Serialize>(T);

    ser(&NewType(100u8), &Value::I64(100));
    ser(&NewType(()), &Value::Null);
    ser(&NewType(false), &Value::Bool(false));
    ser(&NewType("str"), &Value::String(String::from("str")));
    ser(
        &NewType(vec![10, 10]),
        &Value::List(vec![Value::I64(10), Value::I64(10)]),
    );
}
