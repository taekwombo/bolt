use serde::{de, Deserialize};
use super::marker::{Marker};
use super::error::{Error, ErrorCode, Result};
use super::parse::ByteReader;

pub struct Deserializer<'de> {
    read: ByteReader<'de>,
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
            Marker::F64(num) => Ok(num),
            mark => Err(Error::from_code(ErrorCode::ExpectedFloatMarker)),
        }
    }

    fn parse_char(&mut self) -> Result<char> {
        match self.read.peek_marker()? {
            Marker::String(len) if len == 1 => {
                // consume bytes
                self.read.index += 1;
                let b = self.read.peek_byte() as char;
                self.read.index += 1;
                Ok(b)
            }
            mark => Err(Error::from_code(ErrorCode::ExpectedString1Marker))
        }
    }

    fn parse_str (&mut self) -> Result<&'de str> {
        match self.read.peek_marker()? {
            Marker::String(len) => {
                self.read.scratch_peeked();
                // Consume marker + length info bytes
                // return bytes as str
                Ok("")
            },
            mark => Err(Error::from_code(ErrorCode::ExpectedStringMarker))
        }
    }

    fn parse_string(&mut self) -> Result<String> {
        match self.read.peek_marker()? {
            Marker::String(len) => {
                self.read.scratch_peeked();
                // consume marker + length info bytes
                // return bytes as String
                Ok(String::new())
            }
            mark => Err(Error::from_code(ErrorCode::ExpectedStringMarker)),
        }
    }

    fn parse_bytes(&mut self) -> Result<&'de [u8]> {
        match self.read.peek_marker()? {
            Marker::Bytes(len) => {
                self.read.scratch_peeked();
                // consume bytes
                Ok(&[])
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

    fn parse_list(&mut self) -> Result<usize> {
        match self.read.peek_marker()? {
            Marker::List(size) => {
                self.read.scratch_peeked();
                Ok(size)
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

    fn parse_struct(&mut self) -> Result<usize> {
        match self.read.peek_marker()? {
            Marker::Struct(size) => {
                self.read.scratch_peeked();
                Ok(size)
            }
            _ => Err(Error::from_code(ErrorCode::ExpectedListMarker))
        }
    }
}

pub fn from_bytes<'a, T> (source: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>
{
    let mut deserializer = Deserializer {
        read: ByteReader::new(source)
    };

    Err(Error::make("Unimplemented!"))
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V> (self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        match self.read.peek_marker()? {
            Marker::True | Marker::False => self.deserialize_bool(visitor),
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
        eprintln!("Whoah! This is not really implemented. Use f64! Currently numeric value is casted using as operator.");
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
        visitor.visit_str(self.parse_str()?)
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
        visitor.visit_bytes(self.parse_bytes()?)
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
        let list_size = self.parse_list()?;
        visitor.visit_seq(SeqAccess {
            de: self,
            size: list_size,
        })
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




    /*
    = note: `deserialize_tuple` from trait: `fn(Self, usize, V) -> std::result::Result<<V as serde::de::Visitor<'de>>::Value, <Self as serde::de::Deserializer<'de>>::Error>`
    = note: `deserialize_tuple_struct` from trait: `fn(Self, &'static str, usize, V) -> std::result::Result<<V as serde::de::Visitor<'de>>::Value, <Self as serde::de::Deserializer<'de>>::Error>`
    = note: `deserialize_map` from trait: `fn(Self, V) -> std::result::Result<<V as serde::de::Visitor<'de>>::Value, <Self as serde::de::Deserializer<'de>>::Error>`
    = note: `deserialize_struct` from trait: `fn(Self, &'static str, &'static [&'static str], V) -> std::result::Result<<V as serde::de::Visitor<'de>>::Value, <Self as serde::de::Deserializer<'de>>::Error>`
    = note: `deserialize_enum` from trait: `fn(Self, &'static str, &'static [&'static str], V) -> std::result::Result<<V as serde::de::Visitor<'de>>::Value, <Self as serde::de::Deserializer<'de>>::Error>`
    */

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // TODO: Inspect
        // try!(self.ignore_value());
        visitor.visit_unit()
    }
}

struct SeqAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    size: usize,
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>
    {
        // let marker = self.de.parse_int()?;
        // Ok(Some(seed.deserialize(&mut *self.de)?))
    }

}