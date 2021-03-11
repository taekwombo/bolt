use super::constants::marker::*;
use super::error::{Error, ErrorCode, SerdeResult};
use super::marker::Marker;

macro_rules! bytes_to_usize {
    ($b8:expr) => {
        $b8 as usize
    };
    ($b7:expr, $b8:expr) => {
        ($b7 as usize) << 8 | $b8 as usize
    };
    ($b5:expr, $b6:expr, $b7:expr, $b8:expr) => {
        (($b5 as usize) << 24) | (($b6 as usize) << 16) | (($b7 as usize) << 8) | $b8 as usize
    };
}

pub trait Unpacker<'a> {
    /// Creates new instance of Unpacker trait object
    fn new(bytes: &'a [u8]) -> Self;

    /// Checks if all bytes were consumed
    fn is_done(&self) -> bool;

    /// Sets virtual marker and/or value needed for Structure deserialization
    fn set_virtual(&mut self, marker: Marker, value: Option<&'static [u8]>) -> SerdeResult<()>;

    /// Scratches peeked bytes and consumes next N bytes.
    /// If virtual value was set then the virtual value is returned instead.
    fn consume_bytes(&mut self, len: usize) -> SerdeResult<&'a [u8]>;

    /// Returns Nth byte from current index if it exists
    fn peek_byte_nth_ahead(&self, pos_ahead: usize) -> SerdeResult<u8>;

    /// Peek and consume marker.
    fn consume_marker(&mut self) -> SerdeResult<Marker>;

    // Peek marker byte.
    fn peek_marker(&mut self) -> SerdeResult<Marker>;

    fn scratch_peeked(&mut self);
}

impl<'a> Unpacker<'a> for ByteReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            index: 0,
            peeked: 0,
            virtual_value: None,
            virtual_marker: None,
        }
    }

    fn is_done(&self) -> bool {
        self.bytes.len() == self.index
    }

    fn set_virtual(&mut self, marker: Marker, value: Option<&'static [u8]>) -> SerdeResult<()> {
        // Ensure that call to .set_virtual never overwrites existing virtual values
        if self.virtual_marker.is_some() || self.virtual_value.is_some() {
            return Err(Error::create(ErrorCode::VirtualIllegalAssignment));
        }

        self.virtual_marker = Some(marker);
        self.virtual_value = value;

        Ok(())
    }

    /// Called only when additional data needs to be consumed after consuming marker.
    fn consume_bytes(&mut self, len: usize) -> SerdeResult<&'a [u8]> {
        if self.virtual_value.is_some() {
            assert!(self.virtual_marker.is_none());
            return Ok(self.virtual_value.take().expect("Virtual Value to exist"));
        }

        if self.index + len > self.bytes.len() {
            return Err(Error::create(ErrorCode::UnexpectedEndOfBytes));
        }

        let bytes = &self.bytes[self.index..self.index + len];
        self.index += len;

        Ok(bytes)
    }

    fn peek_byte_nth_ahead(&self, ahead: usize) -> SerdeResult<u8> {
        self.bytes
            .get(self.index + ahead)
            .copied()
            .ok_or_else(|| Error::create(ErrorCode::UnexpectedEndOfBytes))
    }

    fn consume_marker(&mut self) -> SerdeResult<Marker> {
        let marker = self.peek_marker()?;

        self.scratch_peeked();

        Ok(marker)
    }

    fn peek_marker(&mut self) -> SerdeResult<Marker> {
        if self.virtual_marker.is_some() {
            assert!(
                self.peeked == 0,
                "ByteReader.peeked must be equal to 0 when setting virtual marker"
            );
            return Ok(self
                .virtual_marker
                .clone()
                .expect("Virtual marker to exist"));
        }

        let marker_byte = self.peek_byte_nth_ahead(0)?;
        let marker = match marker_byte {
            TINY_STRING..=TINY_STRING_MAX => {
                self.peeked = 1;
                Marker::String((marker_byte - TINY_STRING) as usize)
            }
            STRING_8 => {
                let len = self.peek_byte_nth_ahead(1)? as usize;
                self.peeked = 2;
                Marker::String(len)
            }
            STRING_16 => {
                let b8 = self.peek_byte_nth_ahead(2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::String(bytes_to_usize!(b7, b8))
            }
            STRING_32 => {
                let b8 = self.peek_byte_nth_ahead(4)?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];

                self.peeked = 5;
                Marker::String(bytes_to_usize!(b5, b6, b7, b8))
            }
            TINY_MAP..=TINY_MAP_MAX => {
                self.peeked = 1;
                Marker::Map((marker_byte - TINY_MAP) as usize)
            }
            MAP_8 => {
                let len = self.peek_byte_nth_ahead(1)? as usize;
                self.peeked = 2;
                Marker::Map(len)
            }
            MAP_16 => {
                let b8 = self.peek_byte_nth_ahead(self.index + 2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::Map(bytes_to_usize!(b7, b8))
            }
            MAP_32 => {
                let b8 = self.peek_byte_nth_ahead(4)?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];

                self.peeked = 5;
                Marker::Map(bytes_to_usize!(b5, b6, b7, b8))
            }
            MAP_STREAM => {
                self.peeked = 1;
                Marker::Map(std::usize::MAX)
            }
            TINY_STRUCT..=TINY_STRUCT_MAX => {
                self.peeked = 1;
                Marker::Struct((marker_byte - TINY_STRUCT) as usize)
            }
            STRUCT_8 => {
                let len = self.peek_byte_nth_ahead(1)? as usize;
                self.peeked = 2;
                Marker::Struct(len)
            }
            STRUCT_16 => {
                let b8 = self.peek_byte_nth_ahead(2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::Struct(bytes_to_usize!(b7, b8))
            }
            TINY_LIST..=TINY_LIST_MAX => {
                self.peeked = 1;
                Marker::List((marker_byte - TINY_LIST) as usize)
            }
            LIST_8 => {
                let len = self.peek_byte_nth_ahead(1)? as usize;
                self.peeked = 2;
                Marker::List(len)
            }
            LIST_16 => {
                let b8 = self.peek_byte_nth_ahead(2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::List(bytes_to_usize!(b7, b8))
            }
            LIST_32 => {
                let b8 = self.peek_byte_nth_ahead(4)?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];

                self.peeked = 5;
                Marker::List(bytes_to_usize!(b5, b6, b7, b8))
            }
            LIST_STREAM => {
                self.peeked = 1;
                Marker::List(std::usize::MAX)
            }
            NULL => {
                self.peeked = 1;
                Marker::Null
            }
            TRUE => {
                self.peeked = 1;
                Marker::True
            }
            FALSE => {
                self.peeked = 1;
                Marker::False
            }
            INT_8 => {
                let b1 = self.peek_byte_nth_ahead(1)?;
                self.peeked = 2;
                Marker::I64(i64::from(i8::from_be_bytes([b1])))
            }
            INT_16 => {
                let b2 = self.peek_byte_nth_ahead(2)?;
                let b1 = self.bytes[self.index + 1];

                self.peeked = 3;
                let n = i16::from_be_bytes([b1, b2]);
                Marker::I64(i64::from(n))
            }
            INT_32 => {
                let b4 = self.peek_byte_nth_ahead(4)?;
                let b3 = self.bytes[self.index + 3];
                let b2 = self.bytes[self.index + 2];
                let b1 = self.bytes[self.index + 1];

                self.peeked = 5;
                let n = i32::from_be_bytes([b1, b2, b3, b4]);
                Marker::I64(i64::from(n))
            }
            INT_64 => {
                let b8 = self.peek_byte_nth_ahead(8)?;
                let b7 = self.bytes[self.index + 7];
                let b6 = self.bytes[self.index + 6];
                let b5 = self.bytes[self.index + 5];
                let b4 = self.bytes[self.index + 4];
                let b3 = self.bytes[self.index + 3];
                let b2 = self.bytes[self.index + 2];
                let b1 = self.bytes[self.index + 1];

                self.peeked = 9;
                Marker::I64(i64::from_be_bytes([b1, b2, b3, b4, b5, b6, b7, b8]))
            }
            FLOAT_64 => {
                let b8 = self.peek_byte_nth_ahead(8)?;
                let b7 = self.bytes[self.index + 7];
                let b6 = self.bytes[self.index + 6];
                let b5 = self.bytes[self.index + 5];
                let b4 = self.bytes[self.index + 4];
                let b3 = self.bytes[self.index + 3];
                let b2 = self.bytes[self.index + 2];
                let b1 = self.bytes[self.index + 1];

                self.peeked = 9;
                let n = f64::from_bits(u64::from_be_bytes([b1, b2, b3, b4, b5, b6, b7, b8]));
                Marker::F64(n)
            }
            END_OF_STREAM => {
                self.peeked = 1;
                Marker::EOS
            }
            BYTES_8 => {
                let len = self.peek_byte_nth_ahead(1)? as usize;
                self.peeked = 2;
                Marker::Bytes(len)
            }
            BYTES_16 => {
                let b8 = self.peek_byte_nth_ahead(2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::Bytes(bytes_to_usize!(b7, b8))
            }
            BYTES_32 => {
                let b8 = self.peek_byte_nth_ahead(4)?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];

                self.peeked = 5;
                Marker::Bytes(bytes_to_usize!(b5, b6, b7, b8))
            }

            0..=0x7F | 0xF0..=0xFF => {
                self.peeked = 1;
                Marker::I64(i8::from_be_bytes([marker_byte]).into())
            }

            b => {
                return Err(Error::create(format!(
                    "Peek error: byte {:x} is not a marker.",
                    b
                )))
            }
        };

        Ok(marker)
    }

    fn scratch_peeked(&mut self) {
        if self.virtual_marker.is_some() {
            assert!(self.peeked == 0);
            self.virtual_marker = None;
        } else {
            assert!(self.peeked != 0);
            self.index += self.peeked;
            self.peeked = 0;
        }
    }
}

#[derive(Debug)]
pub struct ByteReader<'a> {
    pub bytes: &'a [u8],
    pub index: usize,
    pub peeked: usize,
    pub virtual_value: Option<&'static [u8]>,
    pub virtual_marker: Option<Marker>,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_try_peek {
        ($($bytes:expr => $marker:expr),* $(,)*) => {
            $(assert_eq!($marker, ByteReader::new(&$bytes).peek_marker().unwrap());)*
        };
    }

    #[test]
    fn test_set_virtual() {
        let mut reader = ByteReader::new(&[TINY_STRING]);
        assert!(reader.set_virtual(Marker::I64(0), Some(&[10])).is_ok());
        assert!(reader.consume_marker().is_ok());
        assert!(reader.consume_bytes(0).is_ok());
        assert!(reader.set_virtual(Marker::Null, None).is_ok());
        assert!(reader.set_virtual(Marker::Null, None).is_err());
    }

    #[test]
    fn test_peek_marker() {
        assert_try_peek! {
            [TINY_MAP] => Marker::Map(0),
            [TINY_MAP + 10] => Marker::Map(10),
            [MAP_8, 20] => Marker::Map(20),
            [MAP_16, 1, 0] => Marker::Map(256),
            [MAP_32, 0, 1, 0, 0] => Marker::Map(256 * 256),
            [MAP_STREAM] => Marker::Map(std::usize::MAX),
            [TINY_STRING] => Marker::String(0),
            [TINY_STRING + 10] => Marker::String(10),
            [STRING_8, 20] => Marker::String(20),
            [STRING_16, 1, 0] => Marker::String(256),
            [STRING_32, 0, 1, 0, 0] => Marker::String(256 * 256),
            [TINY_STRUCT] => Marker::Struct(0),
            [STRUCT_8, 20] => Marker::Struct(20),
            [STRUCT_16, 1, 0] => Marker::Struct(256),
            [TINY_LIST + 5] => Marker::List(5),
            [LIST_8, 100] => Marker::List(100),
            [LIST_16, 1, 0] => Marker::List(256),
            [LIST_32, 0, 1, 0, 0] => Marker::List(256 * 256),
            [LIST_STREAM] => Marker::List(std::usize::MAX),
            [BYTES_8, 1] => Marker::Bytes(1),
            [BYTES_16, 1, 0] => Marker::Bytes(256),
            [BYTES_32, 0, 1, 0, 0] => Marker::Bytes(256 * 256),
            [NULL] => Marker::Null,
            [TRUE] => Marker::True,
            [FALSE] => Marker::False,
            [END_OF_STREAM] => Marker::EOS,
            [INT_8, 10] => Marker::I64(10),
            [INT_8, 255] => Marker::I64(-1),
            [INT_16, 1, 0] => Marker::I64(256),
            [INT_16, 255, 255] => Marker::I64(-1),
            [INT_32, 0, 1, 0, 0] => Marker::I64(256 * 256),
            [INT_32, 255, 255, 255, 255] => Marker::I64(-1),
            [INT_64, 0, 0, 1, 0, 0, 0, 0, 0] => Marker::I64(256 * 256 * 256 * 256 * 256),
            [INT_64, 255, 255, 255, 255, 255, 255, 255, 255] => Marker::I64(-1),
            [FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0] => Marker::F64(0.0),
        };
    }
}
