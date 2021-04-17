use super::*;
use serde_bolt::constants::marker::*;

#[test]
fn integer() {
    // TINY_INT
    ser(0, &[0]);
    ser(Value::I64(0), &[0]);
    ser(-16, &[240]);
    ser(Value::I64(-16), &[240]);
    ser(127, &[127]);
    ser(Value::I64(127), &[127]);

    // INT_8
    ser(-128, &[INT_8, 128]);
    ser(Value::I64(-128), &[INT_8, 128]);
    ser(-17, &[INT_8, 239]);
    ser(Value::I64(-17), &[INT_8, 239]);

    // INT_16
    ser(128, &[INT_16, 0, 128]);
    ser(Value::I64(128), &[INT_16, 0, 128]);
    ser(-129, &[INT_16, 255, 127]);
    ser(Value::I64(-129), &[INT_16, 255, 127]);
    ser(std::i16::MIN, &[INT_16, 128, 0]);
    ser(Value::I64(std::i16::MIN as i64), &[INT_16, 128, 0]);
    ser(std::i16::MAX, &[INT_16, 127, 255]);
    ser(Value::I64(std::i16::MAX as i64), &[INT_16, 127, 255]);

    // INT_32
    let start_pos = std::i16::MAX as i64 + 1;
    let start_neg = std::i16::MIN as i64 - 1;

    ser(start_pos, &[INT_32, 0, 0, 128, 0]);
    ser(Value::I64(start_pos), &[INT_32, 0, 0, 128, 0]);
    ser(start_neg, &[INT_32, 255, 255, 127, 255]);
    ser(Value::I64(start_neg), &[INT_32, 255, 255, 127, 255]);
    ser(std::i32::MIN, &[INT_32, 128, 0, 0, 0]);
    ser(Value::I64(std::i32::MIN as i64), &[INT_32, 128, 0, 0, 0]);
    ser(std::i32::MAX, &[INT_32, 127, 255, 255, 255]);
    ser(
        Value::I64(std::i32::MAX as i64),
        &[INT_32, 127, 255, 255, 255],
    );

    // INT_64
    let start_pos = std::i32::MAX as i64 + 1;
    let start_neg = std::i32::MIN as i64 - 1;

    ser(start_pos, &[INT_64, 0, 0, 0, 0, 128, 0, 0, 0]);
    ser(Value::I64(start_pos), &[INT_64, 0, 0, 0, 0, 128, 0, 0, 0]);
    ser(start_neg, &[INT_64, 255, 255, 255, 255, 127, 255, 255, 255]);
    ser(
        Value::I64(start_neg),
        &[INT_64, 255, 255, 255, 255, 127, 255, 255, 255],
    );
    ser(std::i64::MIN, &[INT_64, 128, 0, 0, 0, 0, 0, 0, 0]);
    ser(
        Value::I64(std::i64::MIN),
        &[INT_64, 128, 0, 0, 0, 0, 0, 0, 0],
    );
    ser(
        std::i64::MAX,
        &[INT_64, 127, 255, 255, 255, 255, 255, 255, 255],
    );
    ser(
        Value::I64(std::i64::MAX),
        &[INT_64, 127, 255, 255, 255, 255, 255, 255, 255],
    );

    ser_err(std::u64::MAX);
}

#[test]
fn float() {
    ser(0.0, &[FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0]);
    ser(Value::F64(0.0), &[FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0]);
    ser(100.100, &[FLOAT_64, 64, 89, 6, 102, 102, 102, 102, 102]);
}

#[test]
fn bool() {
    ser(true, &[TRUE]);
    ser(Value::Bool(true), &[TRUE]);
    ser(false, &[FALSE]);
    ser(Value::Bool(false), &[FALSE]);
}

#[test]
fn unit() {
    ser((), &[NULL]);
    ser(Value::Null, &[NULL]);

    #[derive(Serialize)]
    struct UnitStruct;
    ser(UnitStruct, &[NULL]);
}

#[test]
fn string() {
    ser("", &[TINY_STRING]);
    ser("1", &[TINY_STRING + 1, 49]);
    ser(String::new(), &[TINY_STRING]);
    ser(String::from("1"), &[TINY_STRING + 1, 49]);
    ser(Value::String(String::from("1")), &[TINY_STRING + 1, 49]);
    ser(Value::String(String::new()), &[TINY_STRING]);
}

#[test]
fn list() {
    ser([10, 20], &[TINY_LIST + 2, 10, 20]);
    ser(
        [0; 16],
        &[LIST_8, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
    ser(Value::List(Vec::new()), &[TINY_LIST]);
    ser(
        Value::List(vec![Value::Null, Value::Bool(true)]),
        &[TINY_LIST + 2, 192, 194],
    );
}

#[test]
fn map() {
    use std::collections::HashMap;

    ser(HashMap::<(), ()>::new(), &[TINY_MAP]);
    ser(Value::Map(HashMap::new()), &[TINY_MAP]);
    ser(
        Value::Map(map! { "has" => Value::Null }),
        &bytes!([TINY_MAP + 1, TINY_STRING + 3], b"has".to_vec(), [NULL]),
    );
}

#[test]
fn r#struct() {
    #[derive(Serialize)]
    struct Ser {
        bit: u8,
    }

    ser(
        Ser { bit: 0 },
        &[TINY_MAP + 1, TINY_STRING + 3, 98, 105, 116, 0],
    );
}

#[test]
fn r#enum() {
    #[derive(Serialize)]
    enum Ser {
        Var1,
        Var3,
        Var4(u8),
    }

    ser(
        Ser::Var1,
        &bytes!([TINY_MAP + 1, TINY_STRING + 4], b"Var1".to_vec(), [NULL]),
    );
    ser(
        Ser::Var3,
        &bytes!([TINY_MAP + 1, TINY_STRING + 4], b"Var3".to_vec(), [NULL]),
    );
    ser(
        Ser::Var4(10),
        &bytes!([TINY_MAP + 1, TINY_STRING + 4], b"Var4".to_vec(), [10]),
    );
}

//#[test]
//fn structure() {
//    use super::marker::{TINY_MAP, TINY_STRING, TINY_STRUCT};
//    ser(
//        Value::Structure {
//            signature: 10,
//            fields: Vec::new(),
//        },
//        &[TINY_STRUCT, 10],
//    );
//    ser(
//        Value::Structure {
//            signature: 0,
//            fields: Vec::with_capacity(10),
//        },
//        &[TINY_STRUCT, 0],
//    );
//    ser(
//        Value::Structure {
//            signature: 155,
//            fields: vec![Value::Map(map! { "key" => Value::I64(100) })],
//        },
//        &bytes!(
//            [TINY_STRUCT + 1, 155, TINY_MAP + 1, TINY_STRING + 3],
//            b"key".to_vec(),
//            [100]
//        ),
//    );
//}
