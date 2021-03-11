use serde::{Deserialize, Serialize};
use serde_bolt::constants::marker;
use serde_bolt::{from_bytes, to_bytes, Value};
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;

macro_rules! bytes {
    ($($slice:expr),* $(,)*) => {
        {
            let mut arr: Vec<u8> = Vec::new();
            $(arr.extend_from_slice(&$slice);)*
            arr
        }
    }
}

macro_rules! map {
   ($($key:literal => $value:expr),* $(,)*) => {
      {
         let mut map = std::collections::HashMap::new();
         $(map.insert(String::from($key), $value);)*
         map
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

fn de<T>(bytes: &[u8], compare: T)
where
    T: for<'de> Deserialize<'de> + PartialEq + Debug,
{
    let result = from_bytes::<T>(bytes);
    if result.is_err() {
        eprintln!("{:?}", result);
    }
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), compare);
}

mod serialize {
    use super::marker::{INT_16, INT_32, INT_64, INT_8};
    use super::*;

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
        use super::marker::FLOAT_64;

        ser(0.0, &[FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0]);
        ser(Value::F64(0.0), &[FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0]);
        ser(100.100, &[FLOAT_64, 64, 89, 6, 102, 102, 102, 102, 102]);
    }

    #[test]
    fn bool() {
        use super::marker::{FALSE, TRUE};

        ser(true, &[TRUE]);
        ser(Value::Bool(true), &[TRUE]);
        ser(false, &[FALSE]);
        ser(Value::Bool(false), &[FALSE]);
    }

    #[test]
    fn unit() {
        use super::marker::NULL;

        ser((), &[NULL]);
        ser(Value::Null, &[NULL]);

        #[derive(Serialize)]
        struct UnitStruct;
        ser(UnitStruct, &[NULL]);
    }

    #[test]
    fn string() {
        use super::marker::TINY_STRING;

        ser("", &[TINY_STRING]);
        ser("1", &[TINY_STRING + 1, 49]);
        ser(String::new(), &[TINY_STRING]);
        ser(String::from("1"), &[TINY_STRING + 1, 49]);
        ser(Value::String(String::from("1")), &[TINY_STRING + 1, 49]);
        ser(Value::String(String::new()), &[TINY_STRING]);
    }

    #[test]
    fn list() {
        use super::marker::{LIST_8, TINY_LIST};

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
        use super::marker::{NULL, TINY_MAP, TINY_STRING};
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
        use super::marker::{TINY_MAP, TINY_STRING};

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
        use marker::{NULL, TINY_MAP, TINY_STRING};

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

    #[test]
    fn structure() {
        use super::marker::{TINY_MAP, TINY_STRING, TINY_STRUCT};
        ser(
            Value::Structure {
                signature: 10,
                fields: Vec::new(),
            },
            &[TINY_STRUCT, 10],
        );
        ser(
            Value::Structure {
                signature: 0,
                fields: Vec::with_capacity(10),
            },
            &[TINY_STRUCT, 0],
        );
        ser(
            Value::Structure {
                signature: 155,
                fields: vec![Value::Map(map! { "key" => Value::I64(100) })],
            },
            &bytes!(
                [TINY_STRUCT + 1, 155, TINY_MAP + 1, TINY_STRING + 3],
                b"key".to_vec(),
                [100]
            ),
        );
    }
}

mod deserialize {
    use super::*;

    #[test]
    fn skip_unknown() {
        let map_a = map! { "key1" => 0, "key2" => 1, "key3" => 2 };
        let bytes = to_bytes(&map_a).unwrap();
        ser(map_a, &bytes);

        #[derive(serde_derive::Deserialize, Debug)]
        struct TStruct {
            key1: u8,
            key2: u8,
        }

        #[derive(serde_derive::Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        struct TStructNoUnknown {
            key1: u8,
        }

        #[derive(serde_derive::Deserialize, Debug)]
        struct TStructOneKey {
            key3: u8,
        }
    }

    #[test]
    fn integer() {
        use super::marker::{INT_16, INT_32, INT_64, INT_8};

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
        use super::marker::FLOAT_64;

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
        use super::marker::{FALSE, TRUE};

        de(&[TRUE], true);
        de(&[TRUE], Value::Bool(true));
        de(&[FALSE], false);
        de(&[FALSE], Value::Bool(false));
    }

    #[test]
    fn unit() {
        use super::marker::NULL;

        de(&[NULL], ());
        de(&[NULL], Value::Null);

        #[derive(Deserialize, Debug, PartialEq)]
        struct UnitStruct;
        de(&[NULL], UnitStruct);
    }

    #[test]
    fn string() {
        use super::marker::TINY_STRING;

        de(&[TINY_STRING], String::new());
        de(&[TINY_STRING], Value::String(String::new()));

        de(&[TINY_STRING + 1, 49], String::from("1"));
        de(&[TINY_STRING + 1, 49], Value::String(String::from("1")));
    }

    #[test]
    fn list() {
        use super::marker::{LIST_8, TINY_LIST};

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
        use super::marker::{NULL, TINY_MAP, TINY_STRING};
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
        use super::marker::{TINY_MAP, TINY_STRING};

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

    #[test]
    fn structure() {
        use super::marker::{TINY_MAP, TINY_STRING, TINY_STRUCT};

        de(
            &[TINY_STRUCT, 10],
            Value::Structure {
                signature: 10,
                fields: Vec::new(),
            },
        );
        de(
            &[TINY_STRUCT, 0],
            Value::Structure {
                signature: 0,
                fields: Vec::with_capacity(10),
            },
        );
        de(
            &bytes!(
                [TINY_STRUCT + 1, 155, TINY_MAP + 1, TINY_STRING + 3],
                b"key".to_vec(),
                [100]
            ),
            Value::Structure {
                signature: 155,
                fields: vec![Value::Map(map! { "key" => Value::I64(100) })],
            },
        );
    }
}
