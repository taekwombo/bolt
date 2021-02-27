use super::constants::marker::*;
use super::error::{Error, ErrorCode, Result};
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
    fn new(bytes: &'a [u8]) -> Self;

    fn set_virtual(&mut self, marker: Marker, bytes: Option<&'static [u8]>);

    fn get_virtual_marker(&mut self) -> Option<Marker>;

    fn get_virtual_value(&mut self) -> Option<&'static [u8]>;

    fn clear_virtual(&mut self);

    fn consume_bytes(&mut self, len: usize) -> Result<&'a [u8]>;

    fn peek_marker(&mut self) -> Result<Marker>;

    fn consume_peeked(&mut self);

    fn peek_byte_n_ahead(&self, pos_ahead: usize) -> Result<u8>;

    fn done(&self) -> bool;
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

    fn set_virtual(&mut self, marker: Marker, bytes: Option<&'static [u8]>) {
        self.virtual_marker.replace(marker);
        if bytes.is_some() {
            self.virtual_value.replace(bytes.unwrap());
        }
    }

    fn get_virtual_marker(&mut self) -> Option<Marker> {
        self.virtual_marker.take()
    }

    fn get_virtual_value(&mut self) -> Option<&'static [u8]> {
        self.virtual_value.take()
    }

    fn clear_virtual(&mut self) {
        self.virtual_marker = None;
        self.virtual_value = None;
    }

    fn consume_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        if self.index + len > self.bytes.len() {
            return Err(Error::from_code(ErrorCode::UnexpectedEndOfBytes));
        }

        let bytes = &self.bytes[self.index..self.index + len];
        self.index += len;

        Ok(bytes)
    }

    fn consume_peeked(&mut self) {
        self.index += self.peeked;
        self.peeked = 0;
    }

    fn peek_byte_n_ahead(&self, ahead: usize) -> Result<u8> {
        self.bytes
            .get(self.index + ahead)
            .map(|byte| *byte)
            .ok_or_else(|| Error::from_code(ErrorCode::UnexpectedEndOfBytes))
    }

    fn peek_marker(&mut self) -> Result<Marker> {
        let marker_byte = self.peek_byte_n_ahead(0)?;

        let marker = match marker_byte {
            TINY_STRING..=TINY_STRING_MAX => {
                self.peeked = 1;
                Marker::String((marker_byte - TINY_STRING) as usize)
            }
            STRING_8 => {
                let len = self.peek_byte_n_ahead(1)? as usize;
                self.peeked = 2;
                Marker::String(len)
            }
            STRING_16 => {
                let b8 = self.peek_byte_n_ahead(2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::String(bytes_to_usize!(b7, b8))
            }
            STRING_32 => {
                let b8 = self.peek_byte_n_ahead(4)?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];

                self.peeked = 5;
                Marker::String(bytes_to_usize!(b5, b6, b7, b8))
            }

            // Map
            TINY_MAP..=TINY_MAP_MAX => {
                self.peeked = 1;
                Marker::Map((marker_byte - TINY_MAP) as usize)
            }
            MAP_8 => {
                let len = self.peek_byte_n_ahead(1)? as usize;
                self.peeked = 2;
                Marker::Map(len)
            }
            MAP_16 => {
                let b8 = self.peek_byte_n_ahead(self.index + 2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::Map(bytes_to_usize!(b7, b8))
            }
            MAP_32 => {
                let b8 = self.peek_byte_n_ahead(4)?;
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

            // Struct
            TINY_STRUCT..=TINY_STRUCT_MAX => {
                self.peeked = 1;
                Marker::Struct((marker_byte - TINY_STRUCT) as usize)
            }
            STRUCT_8 => {
                let len = self.peek_byte_n_ahead(1)? as usize;
                self.peeked = 2;
                Marker::Struct(len)
            }
            STRUCT_16 => {
                let b8 = self.peek_byte_n_ahead(2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::Struct(bytes_to_usize!(b7, b8))
            }

            // List
            TINY_LIST..=TINY_LIST_MAX => {
                self.peeked = 1;
                Marker::List((marker_byte - TINY_LIST) as usize)
            }
            LIST_8 => {
                let len = self.peek_byte_n_ahead(1)? as usize;
                self.peeked = 2;
                Marker::List(len)
            }
            LIST_16 => {
                let b8 = self.peek_byte_n_ahead(2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::List(bytes_to_usize!(b7, b8))
            }
            LIST_32 => {
                let b8 = self.peek_byte_n_ahead(4)?;
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
                let b1 = self.peek_byte_n_ahead(1)?;
                self.peeked = 2;
                Marker::I64(i64::from(i8::from_be_bytes([b1])))
            }
            INT_16 => {
                let b2 = self.peek_byte_n_ahead(2)?;
                let b1 = self.bytes[self.index + 1];

                self.peeked = 3;
                let n = i16::from_be_bytes([b1, b2]);
                Marker::I64(i64::from(n))
            }
            INT_32 => {
                let b4 = self.peek_byte_n_ahead(4)?;
                let b3 = self.bytes[self.index + 3];
                let b2 = self.bytes[self.index + 2];
                let b1 = self.bytes[self.index + 1];

                self.peeked = 5;
                let n = i32::from_be_bytes([b1, b2, b3, b4]);
                Marker::I64(i64::from(n))
            }
            INT_64 => {
                let b8 = self.peek_byte_n_ahead(8)?;
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
                let b8 = self.peek_byte_n_ahead(8)?;
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
                let len = self.peek_byte_n_ahead(1)? as usize;
                self.peeked = 2;
                Marker::Bytes(len)
            }
            BYTES_16 => {
                let b8 = self.peek_byte_n_ahead(2)?;
                let b7 = self.bytes[self.index + 1];

                self.peeked = 3;
                Marker::Bytes(bytes_to_usize!(b7, b8))
            }
            BYTES_32 => {
                let b8 = self.peek_byte_n_ahead(4)?;
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

            _ => return Err(Error::from_code(ErrorCode::ExpectedMarkerByte)),
        };

        Ok(marker)
    }

    fn done(&self) -> bool {
        self.bytes.len() == self.index
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

// TODO: Unit tests should test only methods of the Structures
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_try_peek {
        ($($bytes:expr => $marker:expr),* $(,)*) => {
            $(assert_eq!($marker, ByteReader::new(&$bytes).peek_marker().unwrap());)*
        };
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
