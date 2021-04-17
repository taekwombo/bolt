use super::*;
use serde_bolt::constants::marker::*;

#[test]
fn integer() {
    de(&[100], 100i8);
    de(&[240], -16i8);

    // INT_8
    de(&[INT_8, 128], -128i8);
    de(&[INT_8, 128], Value::I64(-128));
    de(&[INT_8, 239], -17i8);
    de(&[INT_8, 239], Value::I64(-17));

    // INT_16
    de(&[INT_16, 0, 128], 128i16);
    de(&[INT_16, 0, 128], Value::I64(128));
    de(&[INT_16, 255, 127], -129i16);
    de(&[INT_16, 255, 127], Value::I64(-129));
    de(&[INT_16, 128, 0], std::i16::MIN);
    de(&[INT_16, 128, 0], Value::I64(std::i16::MIN as i64));
    de(&[INT_16, 127, 255], std::i16::MAX);
    de(&[INT_16, 127, 255], Value::I64(std::i16::MAX as i64));

    // INT_32
    let start_pos = std::i16::MAX as i64 + 1;
    let start_neg = std::i16::MIN as i64 - 1;
    de(&[INT_32, 0, 0, 128, 0], start_pos);
    de(&[INT_32, 0, 0, 128, 0], Value::I64(start_pos));
    de(&[INT_32, 255, 255, 127, 255], start_neg);
    de(&[INT_32, 255, 255, 127, 255], Value::I64(start_neg));
    de(&[INT_32, 128, 0, 0, 0], std::i32::MIN);
    de(&[INT_32, 128, 0, 0, 0], Value::I64(std::i32::MIN as i64));
    de(&[INT_32, 127, 255, 255, 255], std::i32::MAX);
    de(
        &[INT_32, 127, 255, 255, 255],
        Value::I64(std::i32::MAX as i64),
    );

    // INT_64
    let start_pos = std::i32::MAX as i64 + 1;
    let start_neg = std::i32::MIN as i64 - 1;

    de(&[INT_64, 0, 0, 0, 0, 128, 0, 0, 0], start_pos);
    de(&[INT_64, 0, 0, 0, 0, 128, 0, 0, 0], Value::I64(start_pos));
    de(&[INT_64, 255, 255, 255, 255, 127, 255, 255, 255], start_neg);
    de(
        &[INT_64, 255, 255, 255, 255, 127, 255, 255, 255],
        Value::I64(start_neg),
    );
    de(&[INT_64, 128, 0, 0, 0, 0, 0, 0, 0], std::i64::MIN);
    de(
        &[INT_64, 128, 0, 0, 0, 0, 0, 0, 0],
        Value::I64(std::i64::MIN),
    );
    de(
        &[INT_64, 127, 255, 255, 255, 255, 255, 255, 255],
        std::i64::MAX,
    );
    de(
        &[INT_64, 127, 255, 255, 255, 255, 255, 255, 255],
        Value::I64(std::i64::MAX),
    );
}

#[test]
fn float() {
    de(&[FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0], 0.0);
    de(&[FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0], Value::F64(0.0));

    de(&[FLOAT_64, 64, 89, 6, 102, 102, 102, 102, 102], 100.100);
    de(
        &[FLOAT_64, 64, 89, 6, 102, 102, 102, 102, 102],
        Value::F64(100.100),
    );
}

#[test]
fn bool() {
    de(&[TRUE], true);
    de(&[TRUE], Value::Bool(true));
    de(&[FALSE], false);
    de(&[FALSE], Value::Bool(false));
}

#[test]
fn unit() {
    de(&[NULL], ());
    de(&[NULL], Value::Null);

    #[derive(Deserialize, Debug, PartialEq)]
    struct UnitStruct;
    de(&[NULL], UnitStruct);
}

#[test]
fn string() {
    de(&[TINY_STRING], String::new());
    de(&[TINY_STRING], Value::String(String::new()));

    de(&[TINY_STRING + 1, 49], String::from("1"));
    de(&[TINY_STRING + 1, 49], Value::String(String::from("1")));
}

#[test]
fn list() {
    de(&[TINY_LIST + 2, 10, 20], [10, 20]);
    de(
        &[LIST_8, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 16],
    );
    de(&[TINY_LIST], Value::List(Vec::new()));
    de(
        &[TINY_LIST + 2, 192, 194],
        Value::List(vec![Value::Null, Value::Bool(true)]),
    );
}

#[test]
fn map() {
    use std::collections::HashMap;

    de(&[TINY_MAP], HashMap::<(), ()>::new());
    de(&[TINY_MAP], Value::Map(HashMap::new()));
    de(
        &bytes!([TINY_MAP + 1, TINY_STRING + 3], b"has".to_vec(), [NULL]),
        Value::Map(map! { "has" => Value::Null }),
    );
}

#[test]
fn r#struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct De {
        bit: u8,
    }

    de(
        &[TINY_MAP + 1, TINY_STRING + 3, 98, 105, 116, 0],
        De { bit: 0 },
    );
}

#[test]
fn r#enum() {
    use marker::{NULL, TINY_MAP, TINY_STRING};

    #[derive(Deserialize, Debug, PartialEq)]
    enum De {
        Var1,
        Var3,
        Var4(u8),
    }

    de(
        &bytes!([TINY_MAP + 1, TINY_STRING + 4], b"Var1".to_vec(), [NULL]),
        De::Var1,
    );
    de(
        &bytes!([TINY_MAP + 1, TINY_STRING + 4], b"Var3".to_vec(), [NULL]),
        De::Var3,
    );
    de(
        &bytes!([TINY_MAP + 1, TINY_STRING + 4], b"Var4".to_vec(), [10]),
        De::Var4(10),
    );
}

//#[test]
//fn structure() {
//    use super::marker::{TINY_MAP, TINY_STRING, TINY_STRUCT};

//    de(
//        &[TINY_STRUCT, 10],
//        Value::Structure {
//            signature: 10,
//            fields: Vec::new(),
//        },
//    );
//    de(
//        &[TINY_STRUCT, 0],
//        Value::Structure {
//            signature: 0,
//            fields: Vec::with_capacity(10),
//        },
//    );
//    de(
//        &bytes!(
//            [TINY_STRUCT + 1, 155, TINY_MAP + 1, TINY_STRING + 3],
//            b"key".to_vec(),
//            [100]
//        ),
//        Value::Structure {
//            signature: 155,
//            fields: vec![Value::Map(map! { "key" => Value::I64(100) })],
//        },
//    );
//}

#[test]
fn skip_unknown() {
    let map_a = map! { "key1" => 0, "key2" => 1, "key3" => 2 };
    let bytes = to_bytes(&map_a).unwrap();
    ser(map_a, &bytes);

    #[derive(Deserialize, Debug, PartialEq)]
    struct TStruct {
        key1: u8,
        key2: u8,
    }
    de(&bytes, TStruct { key1: 0, key2: 1 });

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(deny_unknown_fields)]
    struct TStructNoUnknown {
        key1: u8,
    }
    de(
        &bytes!([TINY_MAP + 1, TINY_STRING + 4], b"key1".to_vec(), [100]),
        TStructNoUnknown { key1: 100 },
    );
    de_err::<TStructNoUnknown>(&bytes);

    #[derive(Deserialize, Debug, PartialEq)]
    struct TStructOneKey {
        key3: u8,
    }
    de(&bytes, TStructOneKey { key3: 2 });
}
