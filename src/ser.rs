use std::convert::TryFrom;
use serde::{ser, Serialize};
use super::marker::Marker;
use super::marker_bytes::STRUCTURE_NAME;
use super::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Serializer {
    output: Vec<u8>,
}

impl Serializer {
    pub(crate) fn write_bytes(&mut self, bytes: &[u8]) -> () {
        self.output.extend_from_slice(bytes);
    }
}

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer {
        output: Vec::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a>;
    type SerializeTuple = Compound<'a>;
    type SerializeTupleStruct = Compound<'a>;
    type SerializeTupleVariant = Compound<'a>;
    type SerializeMap = Compound<'a>;
    type SerializeStruct = Compound<'a>;
    type SerializeStructVariant = Compound<'a>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        let marker = if value { Marker::True } else { Marker::False };
        self.output.append(&mut marker.to_vec()?);
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        println!("serialize_i8: {}", value);
        self.output.append(&mut Marker::I64(i64::from(value)).to_vec()?);
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        println!("serialize_i16: {}", value);
        self.output.append(&mut Marker::I64(i64::from(value)).to_vec()?);
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        println!("serialize_i32: {}", value);
        self.output.append(&mut Marker::I64(i64::from(value)).to_vec()?);
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        println!("serialize_i64: {}", value);
        self.output.append(&mut Marker::I64(value).to_vec()?);
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        println!("serialize_u8: {:?}", value);
        self.output.append(&mut Marker::I64(i64::try_from(value).unwrap()).to_vec()?);
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        println!("serialize_u16: {}", value);
        self.output.append(&mut Marker::I64(i64::try_from(value).unwrap()).to_vec()?);
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        println!("serialize_u32: {}", value);
        self.output.append(&mut Marker::I64(i64::try_from(value).unwrap()).to_vec()?);
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        println!("serialize_u64: {}", value);
        let val_int = i64::try_from(value).unwrap();
        self.output.append(&mut Marker::I64(val_int).to_vec()?);
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        self.serialize_f64(f64::from(value))
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        self.output.append(&mut Marker::F64(value).to_vec()?);
        Ok(())
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        println!("serialize_char: {}", value);
        self.serialize_str(&value.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        println!("serialize_str: {}", value);
        self.output.append(&mut Marker::String(value.len()).to_vec()?);
        self.output.extend_from_slice(&value.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        println!("serialize_bytes: {:?}", value);
        self.output.append(&mut Marker::Bytes(value.len()).to_vec()?);
        self.output.extend_from_slice(value);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        println!("serialize_none");
        self.output.append(&mut Marker::Null.to_vec()?);
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        println!("serialize_unit");
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        println!("serialize_unit_struct: {}", name);
        self.serialize_unit()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok> {
        println!("serialize_unit_variant: name={}, variant_index={}, variant={}", name, variant_index, variant);
        self.serialize_unit()
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize
    {
        println!("serialize_newtype_struct: name={}", name);
        value.serialize(self)
    }

    fn serialize_newtype_variant<T> (self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize
    {
        println!("serialize_newtype_variant: name={}", name);
        // TODO: Is it the right way to serialize newtype_variant?
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        println!("serialize_seq: len={:?}", len);
        if let Some(len) = len {
            self.output.append(&mut Marker::List(len).to_vec()?);
            Ok(Compound::new_static(self))
        } else {
            Ok(Compound::new_dyn(self, Marker::List(0)))
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        // TODO: List or structure?
        // For simplicity serialize as list
        self.output.append(&mut Marker::List(len).to_vec()?);
        Ok(Compound::new_static(self))
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        println!("serialize_tuple_struct: name={}, len={}", name, len);
        // TODO: How to serialize? List? Map with {{name}} key -> List value?
        // For simplicity serialize as list
        if name == STRUCTURE_NAME {
            self.output.append(&mut Marker::Struct(len).to_vec()?);
        } else {
            self.output.append(&mut Marker::List(len).to_vec()?);
        }
        Ok(Compound::new_static(self))
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
        println!("serialize_tuple_variant: name={}, variant_index={}, variant={}, len={}", name, variant_index, variant, len);
        // TODO: How to serialize?
        self.output.append(&mut Marker::List(len).to_vec()?);
        Ok(Compound::new_static(self))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        println!("serialize_map: {:?}", len);
        if let Some(len) = len {
            self.output.append(&mut Marker::Map(len).to_vec()?);
            Ok(Compound::new_static(self))
        } else {
            Ok(Compound::new_dyn(self, Marker::Map(0)))
        }
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        println!("serialize_struct: {} - {}", name, len);
        self.output.append(&mut Marker::Map(len).to_vec()?);
        Ok(Compound::new_static(self))
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
        println!("serialize_struct_variant: name={}, variant_index={}, variant={}, len={}", name, variant_index, variant, len);
        self.output.append(&mut Marker::Map(len).to_vec()?);
        Ok(Compound::new_static(self))
    }
}

#[derive(Debug)]
pub enum Compound<'a> {
    DynSized {
        ser: &'a mut Serializer,
        buf: Vec<u8>, // old buffer state
        marker: Marker,
    },
    StaticSized(&'a mut Serializer),
}

impl<'a> Compound<'a> {
    fn new_dyn (ser: &'a mut Serializer, marker: Marker) -> Self {
        let mut buf = Vec::new();
        std::mem::swap(&mut buf, &mut ser.output);
        Self::DynSized {
            ser,
            buf,
            marker,
        }
    }

    fn new_static(ser: &'a mut Serializer) -> Self {
        Self::StaticSized(ser)
    }

    fn end_state (&mut self) {
        if let Compound::DynSized { ser, buf, marker } = self {
            buf.append(&mut marker.to_vec().unwrap());
            buf.append(&mut ser.output);
            std::mem::swap(buf, &mut ser.output);
        }
    }
}

impl<'a> ser::SerializeSeq for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize
    {
        println!("Compound<ser::SerializeSeq>::serialize_element");
        let ser = match self {
            Compound::StaticSized(ser) => ser,
            Compound::DynSized { ser, marker, .. } => {
                marker.inc_size(1)?;
                ser
            }
        };
        value.serialize(&mut **ser)
    }

    fn end(mut self) -> Result<Self::Ok> {
        &mut self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize
    {
        let ser = match self {
            Compound::DynSized { ser, marker, .. } => {
                marker.inc_size(1)?;
                ser
            },
            Compound::StaticSized(ser) => ser,
        };
        value.serialize(&mut **ser)
    }

    fn end(mut self) -> Result<Self::Ok> {
        &mut self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        println!("SerializeTupleStruct::serialize_field");
        let ser = match self {
            Compound::DynSized { ser, marker, .. } => {
                marker.inc_size(1)?;
                ser
            }
            Compound::StaticSized(ser) => ser
        };
        value.serialize(&mut **ser)
    }

    fn end(mut self) -> Result<Self::Ok> {
        &mut self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        println!("SerializeTupleVariant::serialize_field");
        let ser = match self {
            Compound::DynSized { ser, marker, .. } => {
                marker.inc_size(1)?;
                ser
            }
            Compound::StaticSized(ser) => ser,
        };
        value.serialize(&mut **ser)
    }

    fn end(mut self) -> Result<Self::Ok> {
        &mut self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeMap for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        println!("SerializeMap::serialize_key");
        let ser = match self {
            Compound::DynSized { ser, marker, .. } => {
                marker.inc_size(1)?;
                ser
            }
            Compound::StaticSized(ser) => ser,
        };
        value.serialize(&mut **ser)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        println!("SerializeMap::serialize_value");
        let ser = match self {
            Compound::DynSized { ser, marker, .. } => {
                marker.inc_size(1)?;
                ser
            }
            Compound::StaticSized(ser) => ser,
        };
        value.serialize(&mut **ser)
    }

    fn end(mut self) -> Result<Self::Ok> {
        &mut self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        println!("SerializeStruct::serialize_field: key={}", key);
        let ser = match self {
            Compound::DynSized { ser, marker, .. } => {
                marker.inc_size(1)?;
                ser
            }
            Compound::StaticSized(ser) => ser,
        };

        key.serialize(&mut **ser).unwrap();
        value.serialize(&mut **ser)
    }

    fn end(mut self) -> Result<Self::Ok> {
        &mut self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        println!("SerializeStructVariant::serialize_field");
        let ser = match self {
            Compound::DynSized { ser, marker, .. } => {
                marker.inc_size(1)?;
                ser
            }
            Compound::StaticSized(ser) => ser,
        };

        key.serialize(&mut **ser).unwrap();
        value.serialize(&mut **ser)
    }

    fn end(mut self) -> Result<()> {
        println!("SerializeStructVariant::end");
        &mut self.end_state();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    macro_rules! eq_bytes {
        ($input_ref:expr, $arr:expr) => {
            assert_eq!(to_bytes(&$input_ref).unwrap(), $arr)
        }
    }

    use super::*;
    use serde_derive::Serialize;

    #[test]
    fn test_ser_struct() {
        #[derive(Serialize)]
        struct Test<'a> {
            key: &'a str
        }

        #[derive(Serialize)]
        enum Value {
            Arr([f64; 2]),
            U8(u8)
        }

        #[derive(Serialize)]
        struct Test1 {
            en: Value,
        }

        eq_bytes!(Test { key: "value" }, [161, 131, 107, 101, 121, 133, 118, 97, 108, 117, 101]);
        eq_bytes!(Test { key: "" }, [161, 131, 107, 101, 121, 128]);
        eq_bytes!(Test1 { en: Value::Arr([1.0, 2.0]) }, [161, 130, 101, 110, 146, 193, 63, 240, 0, 0, 0, 0, 0, 0, 193, 64, 0, 0, 0, 0, 0, 0, 0]);
        eq_bytes!(Test1 { en: Value::U8(16) }, [161, 130, 101, 110, 200, 16]);
    }

    // fn test_enum () {}

    // fn test_tuple () {}

    // fn test_map () {}

    // fn test_list () {}

    // fn test_num () {}

    // fn test_str () {}

    // fn test_bytes () {}
}
