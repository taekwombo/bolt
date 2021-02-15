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
        self.output.append(&mut Marker::I64(i64::from(value)).to_vec()?);
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        self.output.append(&mut Marker::I64(i64::from(value)).to_vec()?);
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        self.output.append(&mut Marker::I64(i64::from(value)).to_vec()?);
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        self.output.append(&mut Marker::I64(value).to_vec()?);
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        self.output.append(&mut Marker::I64(i64::try_from(value).unwrap()).to_vec()?);
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        self.output.append(&mut Marker::I64(i64::try_from(value).unwrap()).to_vec()?);
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        self.output.append(&mut Marker::I64(i64::try_from(value).unwrap()).to_vec()?);
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
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
        self.serialize_str(&value.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        self.output.append(&mut Marker::String(value.len()).to_vec()?);
        self.output.extend_from_slice(&value.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        self.output.append(&mut Marker::Bytes(value.len()).to_vec()?);
        self.output.extend_from_slice(value);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
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
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T> (self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize
    {
        // TODO: Is it the right way to serialize newtype_variant?
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
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
        // TODO: How to serialize?
        self.output.append(&mut Marker::List(len).to_vec()?);
        Ok(Compound::new_static(self))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if let Some(len) = len {
            self.output.append(&mut Marker::Map(len).to_vec()?);
            Ok(Compound::new_static(self))
        } else {
            Ok(Compound::new_dyn(self, Marker::Map(0)))
        }
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.output.append(&mut Marker::Map(len).to_vec()?);
        Ok(Compound::new_static(self))
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
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
        &mut self.end_state();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    macro_rules! marked_vec {
        ([$($elems:expr),*], [$elem:literal; $len:literal]) => {
            {
                let mut arr = vec![$($elems),*];
                for i in 0..$len {
                    arr.push($elem);
                }
                arr
            }
        };

        ([$($elems:expr),*], $($vec:expr),*) => {
            {
                let mut arr = vec![$($elems),*];
                $(arr.append(&mut $vec.into_iter().flatten().collect());)*
                arr
            }
        };
    }

    macro_rules! assert_bytes {
        ($($to_ser:expr => $expected:expr),* $(,)*) => {
            $(
                assert_eq!(to_bytes(&$to_ser).unwrap(), $expected);
            )*
        }
    }

    use super::*;
    use serde_derive::Serialize;
    use serde_bytes::Bytes;
    use super::super::marker_bytes::*;

    #[derive(Serialize)]
    struct NewType<T: Serialize>(T);

    #[derive(Serialize)]
    struct TupleStruct<T: Serialize, Y: Serialize>(T, Y);

    #[derive(Serialize)]
    struct List<T: Serialize>(Vec<T>);

    #[test]
    fn serialize_primitive_newtype() {
        assert_bytes! {
            NewType(127) => [INT_8, 127],
            NewType(-128) => [INT_8, 128],
            NewType(200) => [INT_16, 0, 200],
            NewType(-200) => [INT_16, 255, 56],
            NewType(-129) => [INT_16, 255, 127],
            NewType(100000) => [INT_32, 0, 1, 134, 160],
            NewType(-100000) => [INT_32, 255, 254, 121, 96],
            NewType(3000000000u64) => [INT_64, 0, 0, 0, 0, 178, 208, 94, 0],
            NewType(-3000000000i64) => [INT_64, 255, 255, 255, 255, 77, 47, 162, 0],
            NewType(100f64) => [FLOAT_64, 64, 89, 0, 0, 0, 0, 0, 0],
            NewType(100f32) => [FLOAT_64, 64, 89, 0, 0, 0, 0, 0, 0],
            NewType("11111") => [TINY_STRING + 5, 49, 49, 49, 49, 49],
            NewType('1') => [TINY_STRING + 1, 49],
            NewType(String::from("11111")) => [TINY_STRING + 5, 49, 49, 49, 49, 49],
            NewType("1".repeat(16)) => marked_vec!([STRING_8, 16], [49; 16]),
            NewType::<Option<u8>>(None) => [NULL],
            NewType(true) => [TRUE],
            NewType(false) => [FALSE],
        };

    }

    #[test]
    fn serialize_tuple_struct() {
        assert_bytes! {
            TupleStruct(-128, 128) => [TINY_LIST + 2, INT_8, 128, INT_16, 0, 128],
            TupleStruct(true, String::from("1")) => [TINY_LIST + 2, TRUE, TINY_STRING + 1, 49],
            TupleStruct::<Option<u8>, Option<u8>>(None, None) => [TINY_LIST + 2, NULL, NULL],
        };
    }

    #[test]
    fn serialize_list () {
        assert_bytes! {
            List(vec![1; 4]) => marked_vec!([TINY_LIST + 4], vec![vec![INT_8, 1]; 4]),
            List(vec![NewType(String::from("1".repeat(12)))]) => marked_vec!([TINY_LIST + 1, TINY_STRING + 12], [49; 12]),
        };
    }

    // fn test_enum () {}

    // fn test_map () {}

    #[test]
    fn serialize_bytes() {
        assert_bytes! {
            NewType(Bytes::new(&[10, 20, 30, 40, 50])) => [BYTES_8, 5, 10, 20, 30, 40, 50],
            NewType(Bytes::new(&[0; 256])) => marked_vec!([BYTES_16, 1, 0], [0; 256]),
        }
    }
}
