use serde_bytes;
use serde_bolt::{from_bytes, to_bytes, Value, Structure};

macro_rules! ser_and_de {
    ($($value:expr => $type:ty),* $(,)*) => {
        $(assert_eq!(from_bytes::<$type>(&to_bytes(&$value).unwrap()).unwrap(), $value);)*
    };
}

fn create_map (v: &[&str]) -> std::collections::HashMap<String, Value> {
    let mut map = std::collections::HashMap::new();
    for key in v {
        map.insert((*key).to_owned(), Value::I64(map.len() as i64));
    }
    map
}

fn create_byte_buf () -> serde_bytes::ByteBuf {
    let mut buf = serde_bytes::ByteBuf::new();
    for i in 0..127u8 {
        buf.push(i);
    }
    buf
}

#[test]
fn serialize_and_deserialize () {
    let mut structure = Structure::empty();
    for i in 0..16u8 {
        structure.push(Value::I64(i as i64));
    }

    ser_and_de! {
        -0x80 => i8,
        -0x8000 => i16,
        -0x8000_0000 => i32,
        -0x8000_0000_0000_0000i64 => i64,
        0x7F => i8,
        0x7FFF => i16,
        0x7FFF_FFFF => i32,
        0x7FFF_FFFF_FFFF_FFFFi64 => i64,
        "borrowed" => &str,
        "owned" => String,
        () => (),
        Option::<()>::None => Option<()>,
        true => bool,
        false => bool,
        Value::Null => Value,
        Value::Bool(true) => Value,
        Value::Bool(false) => Value,
        Value::I64(0x7FFF) => Value,
        Value::F64(1000f64) => Value,
        Value::String(String::from("owned")) => Value,
        Value::Map(create_map(&vec!["one", "two", "three"])) => Value,
        Value::Bytes(create_byte_buf()) => Value,
        structure => Structure,
    };
}
