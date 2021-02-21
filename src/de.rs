use serde::{de, Deserialize};
use super::marker::{Marker};
use super::error::{Error, ErrorCode, Result};
use super::read::ByteReader;

pub fn from_bytes<'de, T> (bytes: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>
{
    let mut de = Deserializer::new(bytes);
    let value = de::Deserialize::deserialize(&mut de)?;
    de.has_finished()?;
    Ok(value)
}

#[derive(Debug)]
pub struct Deserializer<'de> {
    read: ByteReader<'de>,
}

impl<'de> Deserializer<'de> {
    pub fn new(bytes: &'de [u8]) -> Self {
        Self { read: ByteReader::new(bytes) }
    }

    fn has_finished (&self) -> Result<()> {
        if self.read.index == self.read.bytes.len() {
            Ok(())
        } else {
            Err(Error::from_code(ErrorCode::UnexpectedTrailingBytes))
        }
    }
}

impl<'de> Deserializer<'de> {
    fn parse_bool(&mut self) -> Result<bool> {
        let marker = self.read.peek_marker()?;
        match marker {
            Marker::True => {
                self.read.scratch_peeked();
                Ok(true)
            }
            Marker::False => {
                self.read.scratch_peeked();
                Ok(false)
            }
            mark => Err(Error::from_code(ErrorCode::ExpectedBoolMarker))
        }
    }

    fn parse_int<T>(&mut self) -> Result<T>
    where
        T: std::convert::TryFrom<i64>,
        // Revisit this magic
        <T as std::convert::TryFrom<i64>>::Error: std::error::Error + 'static
    {
        match self.read.peek_marker()? {
            // Revisit this magic
            Marker::I64(num) => {
                let res = T::try_from(num);
                if res.is_ok() {
                    self.read.scratch_peeked();
                }
                res.map_err(|e| Error::make(e.to_string()))
            },
            mark => Err(Error::from_code(ErrorCode::ExpectedIntMarker))
        }
    }

    fn parse_f64(&mut self) -> Result<f64> {
        match self.read.peek_marker()? {
            Marker::F64(num) => {
                self.read.scratch_peeked();
                Ok(num)
            },
            mark => Err(Error::from_code(ErrorCode::ExpectedFloatMarker)),
        }
    }

    fn parse_char(&mut self) -> Result<char> {
        match self.read.peek_marker()? {
            Marker::String(len) if len == 1 => {
                self.read.scratch_peeked();
                let bytes = self.read.consume_bytes(1)?;
                Ok(bytes[0] as char)
            }
            mark => Err(Error::from_code(ErrorCode::ExpectedString1Marker))
        }
    }

    fn parse_str (&mut self) -> Result<&'de str> {
        match self.read.peek_marker()? {
            Marker::String(len) => {
                self.read.scratch_peeked();
                let bytes = self.read.consume_bytes(len)?;
                Ok(std::str::from_utf8(bytes)?)
            },
            mark => Err(Error::from_code(ErrorCode::ExpectedStringMarker))
        }
    }

    fn parse_string(&mut self) -> Result<String> {
        match self.read.peek_marker()? {
            Marker::String(len) => {
                self.read.scratch_peeked();
                let bytes = self.read.consume_bytes(len)?.to_vec();
                Ok(String::from_utf8(bytes)?)
            }
            mark => Err(Error::from_code(ErrorCode::ExpectedStringMarker)),
        }
    }

    fn parse_bytes(&mut self) -> Result<&'de [u8]> {
        match self.read.peek_marker()? {
            Marker::Bytes(len) => {
                self.read.scratch_peeked();
                let bytes = self.read.consume_bytes(len)?;
                Ok(bytes)
            }
            mark => Err(Error::from_code(ErrorCode::ExpectedStringMarker)),
        }
    }

    fn parse_null(&mut self) -> Result<bool> {
        match self.read.peek_marker()? {
            Marker::Null => {
                self.read.scratch_peeked();
                Ok(true)
            },
            _ => Ok(false),
        }
    }

    fn parse_list_or_structure(&mut self) -> Result<usize> {
        match self.read.peek_marker()? {
            Marker::List(size) => {
                self.read.scratch_peeked();
                Ok(size)
            }
            Marker::Struct(size) => {
                self.read.scratch_peeked();
                Ok(size + 1)
            }
            _ => Err(Error::from_code(ErrorCode::ExpectedListMarker)),
        }
    }

    fn parse_map(&mut self) -> Result<usize> {
        match self.read.peek_marker()? {
            Marker::Map(size) => {
                self.read.scratch_peeked();
                Ok(size)
            }
            _ => Err(Error::from_code(ErrorCode::ExpectedListMarker)),
        }
    }

    fn parse_enum(&mut self) -> Result<()> {
        match self.read.peek_marker()? {
            Marker::Map(len) if len == 1 => {
                self.read.scratch_peeked();
                Ok(())
            }
            _ => Err(Error::from_code(ErrorCode::ExpectedListMarker))
        }
    }

    fn try_end_stream(&mut self) -> bool {
        let marker = self.read.peek_marker();
        if marker.is_ok() {
            match marker.unwrap() {
                Marker::EOS => {
                    self.read.scratch_peeked();
                    true
                },
                _ => false,
            }
        } else {
            false
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V> (self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        match self.read.peek_marker()? {
            Marker::True | Marker::False => self.deserialize_bool(visitor),
            Marker::Null => self.deserialize_unit(visitor),
            Marker::List(len) => self.deserialize_seq(visitor),
            Marker::Map(len) => self.deserialize_map(visitor),
            Marker::Bytes(len) => self.deserialize_bytes(visitor),
            Marker::String(len) => self.deserialize_str(visitor),
            Marker::I64(_) => self.deserialize_i64(visitor),
            Marker::F64(_) => self.deserialize_f64(visitor),
            // TODO: Impl Structure ser/de
            _ => unimplemented!()
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_i8(self.parse_int()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_i16(self.parse_int()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_i32(self.parse_int()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_i64(self.parse_int()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_u8(self.parse_int()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_u16(self.parse_int()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_u32(self.parse_int()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_u64(self.parse_int()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_f32(self.parse_f64()? as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_f64(self.parse_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_char(self.parse_char()?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_string(self.parse_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_byte_buf(self.parse_bytes()?.to_owned())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        if self.parse_null()? {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        if self.parse_null()? {
            visitor.visit_unit()
        } else {
            // TODO: Refactor to function that gives hint on next marker
            Err(Error::from_code(ErrorCode::UnexpectedType))
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        let list_len = self.parse_list_or_structure()?;
        visitor.visit_seq(SeqAccess::new(self, list_len))
    }

    fn deserialize_tuple<V>(self, _size: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &str, _size: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        let map_len = self.parse_map()?;
        visitor.visit_map(MapAccess {
            de: self,
            len: map_len
        })
    }

    fn deserialize_struct<V>(self, _name: &str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        let map_len = self.parse_map()?;
        visitor.visit_map(MapAccess {
            de: self,
            len: map_len
        })
    }


    fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        self.parse_enum()?;
        visitor.visit_enum(VariantAccess { de: self })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // TODO: IMPLEMENT
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // TODO: Check if this function can be invoked in the middle of parsing map.
        // This would consume either key or value of a map.
        // visitor.consume_any()?;
        visitor.visit_none()
    }
}

struct SeqAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: usize,
}

impl<'a, 'de> SeqAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len: usize) -> Self {
        Self { de, len }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>
    {
        if self.len == 0 || self.de.try_end_stream() {
            return Ok(None);
        }

        let val = seed.deserialize(&mut *self.de)?;
        self.len -= 1;
        Ok(Some(val))
    }
}

struct MapAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: usize,
}

impl<'de, 'a> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }

        let val = seed.deserialize(&mut *self.de)?;
        self.len -= 1;
        Ok(Some(val))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>
    {
        seed.deserialize(&mut *self.de)
    }
}

struct VariantAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>
}

impl<'a, 'de: 'a> de::EnumAccess<'de> for VariantAccess<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>
    {
        let value = seed.deserialize(&mut *self.de)?;
        Ok((value, self))
    }
}

impl<'a, 'de: 'a> de::VariantAccess<'de> for VariantAccess<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        de::Deserialize::deserialize(self.de)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(self, fields: &'static[&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        de::Deserializer::deserialize_struct(self.de, "", fields, visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::marker_bytes::*;
    use serde_derive::Deserialize;
    use serde_bytes::{Bytes, ByteBuf};

    macro_rules! bytes {
        ($($slice:expr),* $(,)*) => {
            {
                let mut arr = Vec::new();
                $(arr.extend_from_slice(&$slice);)*
                arr
            }
        }
    }

    macro_rules! assert_deserialize {
        ($($t:ty => $arr:expr),* $(,)*) => {
            $(assert!(from_bytes::<$t>(&$arr).map_err(|e| eprintln!("{}", e)).is_ok());)*
        }
    }

    #[derive(Deserialize)]
    struct NewType<T>(T);

    #[derive(Deserialize)]
    struct TupleStruct<T, Y>(T, Y);

    #[derive(Deserialize)]
    struct List<T>(Vec<T>);

    #[derive(Deserialize)]
    enum TestEnum {
        UnitVariant,
        NewTypeVariant(u8),
        TupleVariant(u8, u8),
        StructVarint { one: u8 }
    }

    #[test]
    fn deserialize_primitive_newtype() {
        assert_deserialize! {
            NewType<i8> => [10],
            NewType<i8> => [INT_8, 10],
            NewType<i8> => [INT_16, 0, 0],
            NewType<i8> => [INT_32, 0, 0, 0, 0],
            NewType<i8> => [INT_64, 0, 0, 0, 0, 0, 0, 0, 0],
            NewType<i16> => [INT_16, 1, 0],
            NewType<i16> => [INT_32, 0, 0, 1, 0],
            NewType<i16> => [INT_64, 0, 0, 0, 0, 0, 0, 1, 0],
            NewType<i32> => [INT_32, 0, 1, 1, 0],
            NewType<i32> => [INT_64, 0, 0, 0, 0, 0, 0, 1, 0],
            NewType<i64> => [INT_64, 0, 0, 0, 0, 0, 0, 1, 0],
            NewType<f64> => [FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0],
            NewType<char> => [TINY_STRING + 1, 49],
            NewType<char> => [STRING_8, 1, 49],
            NewType<char> => [STRING_16, 0, 1, 49],
            NewType<char> => [STRING_32, 0, 0, 0, 1, 49],
            NewType<&str> => [TINY_STRING + 1, 50],
            NewType<&str> => [STRING_8, 1, 50],
            NewType<&str> => [STRING_16, 0, 1, 50],
            NewType<&str> => [STRING_32, 0, 0, 0, 1, 50],
            NewType<String> => [TINY_STRING + 1, 51],
            NewType<String> => [STRING_8, 1, 51],
            NewType<String> => [STRING_16, 0, 1, 51],
            NewType<String> => [STRING_32, 0, 0, 0, 1, 51],
            NewType<&[u8]> => [BYTES_8, 1, 0],
            NewType<&[u8]> => [BYTES_16, 0, 1, 0],
            NewType<&[u8]> => [BYTES_32, 0, 0, 0, 1, 0],
            NewType<&Bytes> => [BYTES_8, 1, 0],
            NewType<&Bytes> => [BYTES_16, 0, 1, 0],
            NewType<&Bytes> => [BYTES_32, 0, 0, 0, 1, 0],
            NewType<ByteBuf> => [BYTES_8, 1, 0],
            NewType<ByteBuf> => [BYTES_16, 0, 1, 0],
            NewType<ByteBuf> => [BYTES_32, 0, 0, 0, 1, 0],
            NewType<()> => [NULL],
            NewType<bool> => [TRUE],
            NewType<bool> => [FALSE],
        };
    }

    #[test]
    fn deserialize_tuple_struct() {
        assert_deserialize! {
            TupleStruct<i8, i8> => [TINY_LIST + 2, 1, 1],
            TupleStruct<i8, i8> => [LIST_8, 2, 1, 1],
            TupleStruct<i8, i8> => [LIST_16, 0, 2, 1, 1],
            TupleStruct<i8, i8> => [LIST_32, 0, 0, 0, 2, 1, 1],
        }
    }

    #[test]
    fn deserialize_list() {
        assert_deserialize! {
            List<u8> => [TINY_LIST + 2, 1, 1],
            List<u8> => [LIST_8, 2, 1, 1],
            List<u8> => [LIST_16, 0, 2, 1, 1],
            List<u8> => [LIST_32, 0, 0, 0, 2, 1, 1],
            List<u8> => [LIST_STREAM, 1, 1, END_OF_STREAM],
        }
    }

    #[test]
    fn deserialize_enum() {
        assert_deserialize! {
            TestEnum => bytes!([TINY_MAP + 1, TINY_STRING + 11], b"UnitVariant".to_vec(), [NULL]),
            TestEnum => bytes!([TINY_MAP + 1, TINY_STRING + 14], b"NewTypeVariant".to_vec(), [127]),
            TestEnum => bytes!([TINY_MAP + 1, TINY_STRING + 12], b"TupleVariant".to_vec(), [TINY_LIST + 2, 100, 100]),
            TestEnum => bytes!([TINY_MAP + 1, TINY_STRING + 12], b"StructVarint".to_vec(), [TINY_MAP + 1, TINY_STRING + 3], b"one".to_vec(), [100]),
        }
    }

    #[test]
    fn deserialize_map() {
        use std::collections::HashMap;

        #[derive(Deserialize)]
        struct TestStruct {
            one: u8,
        }

        assert_deserialize! {
            TestStruct => bytes!([TINY_MAP + 1, TINY_STRING + 3], b"one".to_vec(), [100]),
            HashMap<&str, u8> => bytes!([TINY_MAP + 2, TINY_STRING + 2], b"01".to_vec(), [100], [TINY_STRING + 3], b"123".to_vec(), [100]),
        }
    }

    #[test]
    fn deserialize_bytes() {
        assert_deserialize! {
            NewType<&Bytes> => bytes!([BYTES_8, 5, 10, 20, 30, 40, 50]),
            NewType<ByteBuf> => bytes!([BYTES_16, 1, 0], [0; 256]),
        }
    }
}
