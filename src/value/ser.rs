use super::Value;
use crate::error::{SerdeError, SerdeResult};
use serde::ser::{self, Impossible};
use serde_bytes::ByteBuf;
use std::collections::HashMap;

pub fn to_value<T>(value: T) -> SerdeResult<Value>
where
    T: ser::Serialize,
{
    value.serialize(Serializer)
}

impl ser::Serialize for Value {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_unit(),
            Self::Bool(v) => serializer.serialize_bool(*v),
            Self::I64(v) => serializer.serialize_i64(*v),
            Self::F64(v) => serializer.serialize_f64(*v),
            Self::String(v) => serializer.serialize_str(&v),
            Self::List(v) => v.serialize(serializer),
            Self::Map(v) => {
                use ser::SerializeMap;

                let mut map = serializer.serialize_map(Some(v.len()))?;
                for (k, v) in v.iter() {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Self::Bytes(v) => serializer.serialize_bytes(&*v),
            Self::Structure(v) => (*v).serialize(serializer),
        }
    }
}

pub struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = Value;
    type Error = SerdeError;

    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeSeq;
    type SerializeTupleStruct = SerializeSeq;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, value: bool) -> SerdeResult<Self::Ok> {
        Ok(Value::Bool(value))
    }

    fn serialize_i8(self, value: i8) -> SerdeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i16(self, value: i16) -> SerdeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i32(self, value: i32) -> SerdeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> SerdeResult<Self::Ok> {
        Ok(Value::I64(value))
    }

    fn serialize_u8(self, value: u8) -> SerdeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_u16(self, value: u16) -> SerdeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_u32(self, value: u32) -> SerdeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_u64(self, value: u64) -> SerdeResult<Self::Ok> {
        use std::convert::TryFrom;

        // TODO(@krnik) - unify this error with /src/ser.rs conversion errors
        let val_int = i64::try_from(value).map_err(|_| {
            SerdeError::create(format!("Attempt to convert {}u64 into i63 failed", value))
        })?;
        self.serialize_i64(val_int)
    }

    fn serialize_f32(self, value: f32) -> SerdeResult<Self::Ok> {
        self.serialize_f64(value as f64)
    }

    fn serialize_f64(self, value: f64) -> SerdeResult<Self::Ok> {
        Ok(Value::F64(value))
    }

    fn serialize_char(self, value: char) -> SerdeResult<Self::Ok> {
        let mut s = String::new();
        s.push(value);
        Ok(Value::String(s))
    }

    fn serialize_str(self, value: &str) -> SerdeResult<Self::Ok> {
        Ok(Value::String(value.to_owned()))
    }

    fn serialize_bytes(self, value: &[u8]) -> SerdeResult<Self::Ok> {
        Ok(Value::Bytes(ByteBuf::from(value)))
    }

    fn serialize_unit(self) -> SerdeResult<Self::Ok> {
        Ok(Value::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> SerdeResult<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> SerdeResult<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> SerdeResult<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> SerdeResult<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_none(self) -> SerdeResult<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> SerdeResult<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> SerdeResult<Self::SerializeSeq> {
        Ok(SerializeSeq::new(len.unwrap_or(0)))
    }

    fn serialize_tuple(self, len: usize) -> SerdeResult<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> SerdeResult<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariant {
            name: name.to_owned(),
            vec: Vec::new(),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> SerdeResult<Self::SerializeMap> {
        Ok(SerializeMap {
            key: None,
            map: HashMap::new(),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> SerdeResult<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeStructVariant> {
        Ok(SerializeStructVariant {
            name: variant.to_owned(),
            map: HashMap::new(),
        })
    }
}

pub struct SerializeSeq {
    vec: Vec<Value>,
}

impl SerializeSeq {
    fn new(len: usize) -> Self {
        Self {
            vec: Vec::with_capacity(len),
        }
    }
}

impl ser::SerializeSeq for SerializeSeq {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ser::Serialize,
    {
        // TODO: Implement to_value
        // https://github.com/serde-rs/json/blob/master/src/value/mod.rs#L952
        self.vec.push(to_value(&value)?);
        Ok(())
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        Ok(Value::List(self.vec))
    }
}

impl ser::SerializeTuple for SerializeSeq {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ser::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SerializeSeq {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ser::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        serde::ser::SerializeSeq::end(self)
    }
}

pub struct SerializeTupleVariant {
    name: String,
    vec: Vec<Value>,
}

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ser::Serialize,
    {
        self.vec.push(to_value(&value)?);
        Ok(())
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        let mut map = HashMap::new();
        map.insert(self.name, Value::List(self.vec));
        Ok(Value::Map(map))
    }
}

pub struct SerializeMap {
    map: HashMap<String, Value>,
    key: Option<String>,
}

impl ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> SerdeResult<()>
    where
        T: ser::Serialize,
    {
        self.key.replace(key.serialize(MapKeySerializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> SerdeResult<()>
    where
        T: ser::Serialize,
    {
        self.map
            .insert(self.key.take().expect("Key to exist"), to_value(&value)?);
        Ok(())
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        Ok(Value::Map(self.map))
    }
}

impl ser::SerializeStruct for SerializeMap {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> SerdeResult<()>
    where
        T: ser::Serialize,
    {
        ser::SerializeMap::serialize_value(self, value)
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        ser::SerializeMap::end(self)
    }
}

pub struct SerializeStructVariant {
    name: String,
    map: HashMap<String, Value>,
}

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> SerdeResult<()>
    where
        T: ser::Serialize,
    {
        self.map.insert(key.to_owned(), to_value(value)?);
        Ok(())
    }

    fn end(self) -> SerdeResult<Self::Ok> {
        let mut map = HashMap::new();
        map.insert(self.name, Value::Map(self.map));
        Ok(Value::Map(map))
    }
}

struct MapKeySerializer;

fn key_must_be_a_string() -> SerdeError {
    SerdeError::create("key must be a string")
}

impl ser::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = SerdeError;

    type SerializeSeq = Impossible<String, SerdeError>;
    type SerializeTuple = Impossible<String, SerdeError>;
    type SerializeTupleStruct = Impossible<String, SerdeError>;
    type SerializeTupleVariant = Impossible<String, SerdeError>;
    type SerializeMap = Impossible<String, SerdeError>;
    type SerializeStruct = Impossible<String, SerdeError>;
    type SerializeStructVariant = Impossible<String, SerdeError>;

    fn serialize_bool(self, _value: bool) -> SerdeResult<Self::Ok> {
        Err(key_must_be_a_string())
    }

    fn serialize_i8(self, value: i8) -> SerdeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_i16(self, value: i16) -> SerdeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_i32(self, value: i32) -> SerdeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_i64(self, value: i64) -> SerdeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u8(self, value: u8) -> SerdeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u16(self, value: u16) -> SerdeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u32(self, value: u32) -> SerdeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u64(self, value: u64) -> SerdeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_f32(self, _value: f32) -> SerdeResult<Self::Ok> {
        Err(key_must_be_a_string())
    }

    fn serialize_f64(self, _value: f64) -> SerdeResult<Self::Ok> {
        Err(key_must_be_a_string())
    }

    fn serialize_char(self, value: char) -> SerdeResult<Self::Ok> {
        Ok({
            let mut s = String::new();
            s.push(value);
            s
        })
    }

    fn serialize_str(self, value: &str) -> SerdeResult<Self::Ok> {
        Ok(value.to_owned())
    }

    fn serialize_bytes(self, _value: &[u8]) -> SerdeResult<Self::Ok> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> SerdeResult<Self::Ok> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> SerdeResult<Self::Ok> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> SerdeResult<Self::Ok> {
        Ok(variant.to_owned())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> SerdeResult<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> SerdeResult<Self::Ok>
    where
        T: ser::Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_none(self) -> SerdeResult<Self::Ok> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> SerdeResult<Self::Ok>
    where
        T: ser::Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> SerdeResult<Self::SerializeSeq> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple(self, _len: usize) -> SerdeResult<Self::SerializeTuple> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeTupleStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeTupleVariant> {
        Err(key_must_be_a_string())
    }

    fn serialize_map(self, _len: Option<usize>) -> SerdeResult<Self::SerializeMap> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeStructVariant> {
        Err(key_must_be_a_string())
    }
}
