use super::constants::STRUCTURE_NAME;
use super::error::{PackstreamError, PackstreamResult};
use super::marker::Marker;
use serde::{ser, Serialize};

#[derive(Clone, Debug)]
pub struct Serializer {
    output: Vec<u8>,
}

pub fn to_bytes<T: Serialize>(value: &T) -> PackstreamResult<Vec<u8>> {
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = PackstreamError;

    type SerializeSeq = Compound<'a>;
    type SerializeTuple = Compound<'a>;
    type SerializeTupleStruct = Compound<'a>;
    type SerializeTupleVariant = Compound<'a>;
    type SerializeMap = Compound<'a>;
    type SerializeStruct = Compound<'a>;
    type SerializeStructVariant = Compound<'a>;

    fn serialize_bool(self, value: bool) -> PackstreamResult<Self::Ok> {
        let marker = if value { Marker::True } else { Marker::False };
        marker.append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> PackstreamResult<Self::Ok> {
        Marker::I64(i64::from(value)).append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> PackstreamResult<Self::Ok> {
        Marker::I64(i64::from(value)).append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> PackstreamResult<Self::Ok> {
        Marker::I64(i64::from(value)).append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> PackstreamResult<Self::Ok> {
        Marker::I64(i64::from(value)).append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> PackstreamResult<Self::Ok> {
        Marker::I64(value as i64).append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> PackstreamResult<Self::Ok> {
        Marker::I64(value as i64).append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> PackstreamResult<Self::Ok> {
        Marker::I64(value as i64).append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> PackstreamResult<Self::Ok> {
        use std::convert::TryFrom;

        let val_int = i64::try_from(value).map_err(|_| {
            PackstreamError::create(format!("Attempt to convert {}u64 into i64 failed", value))
        })?;
        Marker::I64(val_int).append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> PackstreamResult<Self::Ok> {
        self.serialize_f64(f64::from(value))
    }

    fn serialize_f64(self, value: f64) -> PackstreamResult<Self::Ok> {
        Marker::F64(value).append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_char(self, value: char) -> PackstreamResult<Self::Ok> {
        self.serialize_str(&value.to_string())
    }

    fn serialize_str(self, value: &str) -> PackstreamResult<Self::Ok> {
        Marker::String(value.len()).append_to_vec(&mut self.output)?;
        self.output.extend_from_slice(&value.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> PackstreamResult<Self::Ok> {
        Marker::String(value.len()).append_to_vec(&mut self.output)?;
        self.output.extend_from_slice(value);
        Ok(())
    }

    fn serialize_none(self) -> PackstreamResult<Self::Ok> {
        Marker::Null.append_to_vec(&mut self.output)?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> PackstreamResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> PackstreamResult<Self::Ok> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> PackstreamResult<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> PackstreamResult<Self::Ok> {
        Marker::Map(1).append_to_vec(&mut self.output)?;
        Marker::String(variant.len()).append_to_vec(&mut self.output)?;

        self.output.extend_from_slice(&variant.as_bytes());
        self.serialize_unit()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> PackstreamResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> PackstreamResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Marker::Map(1).append_to_vec(&mut self.output)?;
        Marker::String(variant.len()).append_to_vec(&mut self.output)?;

        self.output.extend_from_slice(&variant.as_bytes());
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> PackstreamResult<Self::SerializeSeq> {
        if let Some(len) = len {
            Marker::List(len).append_to_vec(&mut self.output)?;
            Ok(Compound::new_static(self))
        } else {
            Ok(Compound::new_dyn(self, Marker::List(0)))
        }
    }

    fn serialize_tuple(self, len: usize) -> PackstreamResult<Self::SerializeTuple> {
        Marker::List(len).append_to_vec(&mut self.output)?;
        Ok(Compound::new_static(self))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> PackstreamResult<Self::SerializeTupleStruct> {
        if name == STRUCTURE_NAME {
            let signature = len >> 56;
            let structure_length = len << 8 >> 8;
            Marker::Struct(structure_length).append_to_vec(&mut self.output)?;
            self.output.extend_from_slice(&[signature as u8]);
        } else {
            Marker::List(len).append_to_vec(&mut self.output)?;
        }
        Ok(Compound::new_static(self))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> PackstreamResult<Self::SerializeTupleVariant> {
        Marker::Map(1).append_to_vec(&mut self.output)?;
        Marker::String(variant.len()).append_to_vec(&mut self.output)?;
        self.output.extend_from_slice(&variant.as_bytes());
        Marker::List(len).append_to_vec(&mut self.output)?;
        Ok(Compound::new_static(self))
    }

    fn serialize_map(self, len: Option<usize>) -> PackstreamResult<Self::SerializeMap> {
        if let Some(len) = len {
            Marker::Map(len).append_to_vec(&mut self.output)?;
            Ok(Compound::new_static(self))
        } else {
            Ok(Compound::new_dyn(self, Marker::Map(0)))
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> PackstreamResult<Self::SerializeStruct> {
        Marker::Map(len).append_to_vec(&mut self.output)?;
        Ok(Compound::new_static(self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> PackstreamResult<Self::SerializeStructVariant> {
        Marker::Map(1).append_to_vec(&mut self.output)?;
        Marker::String(variant.len()).append_to_vec(&mut self.output)?;
        self.output.extend_from_slice(&variant.as_bytes());
        Marker::Map(len).append_to_vec(&mut self.output)?;
        Ok(Compound::new_static(self))
    }
}

#[derive(Debug)]
pub enum Compound<'a> {
    DynSized {
        ser: &'a mut Serializer,
        buf: Vec<u8>, // old buffer
        marker: Marker,
    },
    StaticSized(&'a mut Serializer),
}

impl<'a> Compound<'a> {
    fn new_dyn(ser: &'a mut Serializer, marker: Marker) -> Self {
        let mut buf = Vec::new();
        std::mem::swap(&mut buf, &mut ser.output);
        Self::DynSized { ser, buf, marker }
    }

    fn new_static(ser: &'a mut Serializer) -> Self {
        Self::StaticSized(ser)
    }

    fn end_state(&mut self) {
        if let Compound::DynSized { ser, ref mut buf, marker } = self {
            marker.append_to_vec(buf).unwrap();
            buf.append(&mut ser.output);
            std::mem::swap(buf, &mut ser.output);
        }
    }
}

impl<'a> ser::SerializeSeq for Compound<'a> {
    type Ok = ();
    type Error = PackstreamError;

    fn serialize_element<T>(&mut self, value: &T) -> PackstreamResult<Self::Ok>
    where
        T: ?Sized + Serialize,
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

    fn end(mut self) -> PackstreamResult<Self::Ok> {
        self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for Compound<'a> {
    type Ok = ();
    type Error = PackstreamError;

    fn serialize_element<T>(&mut self, value: &T) -> PackstreamResult<Self::Ok>
    where
        T: ?Sized + Serialize,
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

    fn end(mut self) -> PackstreamResult<Self::Ok> {
        self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for Compound<'a> {
    type Ok = ();
    type Error = PackstreamError;

    fn serialize_field<T>(&mut self, value: &T) -> PackstreamResult<()>
    where
        T: ?Sized + Serialize,
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

    fn end(mut self) -> PackstreamResult<Self::Ok> {
        self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for Compound<'a> {
    type Ok = ();
    type Error = PackstreamError;

    fn serialize_field<T>(&mut self, value: &T) -> PackstreamResult<()>
    where
        T: ?Sized + Serialize,
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

    fn end(mut self) -> PackstreamResult<Self::Ok> {
        self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeMap for Compound<'a> {
    type Ok = ();
    type Error = PackstreamError;

    fn serialize_key<T>(&mut self, value: &T) -> PackstreamResult<()>
    where
        T: ?Sized + Serialize,
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

    fn serialize_value<T>(&mut self, value: &T) -> PackstreamResult<()>
    where
        T: ?Sized + Serialize,
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

    fn end(mut self) -> PackstreamResult<Self::Ok> {
        self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for Compound<'a> {
    type Ok = ();
    type Error = PackstreamError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> PackstreamResult<()>
    where
        T: ?Sized + Serialize,
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

    fn end(mut self) -> PackstreamResult<Self::Ok> {
        self.end_state();
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for Compound<'a> {
    type Ok = ();
    type Error = PackstreamError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> PackstreamResult<()>
    where
        T: ?Sized + Serialize,
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

    fn end(mut self) -> PackstreamResult<()> {
        self.end_state();
        Ok(())
    }
}
