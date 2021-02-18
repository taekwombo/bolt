use super::error::{Error, ErrorCode, Result};
use super::marker::Marker;
use super::marker_bytes::*;

macro_rules! bytes_to_usize {
    ($b8:expr) => {
        $b8 as usize
    };
    ($b7:expr, $b8:expr) => {
        ($b7 as usize) << 8 | $b8 as usize
    };
    ($b5:expr, $b6:expr, $b7:expr, $b8:expr) => {
        (($b5 as usize) << 24)
            | (($b6 as usize) << 16)
            | (($b7 as usize) << 8)
            | $b8 as usize
    };
}

type MarkerHint = (Marker, usize);

#[derive(Debug)]
pub struct ByteReader<'a> {
    pub bytes: &'a [u8],
    pub index: usize,
    pub peeked: Option<Marker>,
}

// peek_marker -> Result<Marker>
// scratch_peeked_marker -> ()
// consume_bytes -> &'a [u8]

impl<'a> ByteReader<'a> {
    pub fn new (bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            index: 0,
            peeked: None,
        }
    }

    pub fn peek_byte(&self) -> u8 {
        self.bytes[self.index]
    }

    pub fn get_bytes(&mut self, len: usize) -> &'a [u8] {
        if self.index + self.bytes.len() > len {
            // Err
        }
        &self.bytes[self.index..self.index + len]
    }

    pub fn scratch_peeked(&mut self) {
        if let None = self.peeked {
            return;
        }

        match self.peeked.as_ref().unwrap() {
            Marker::True | Marker::False | Marker::Null | Marker::EOS => {
                self.index += 1;
            }
            _ => {}
        }
    }

    fn get_byte(&self, ahead: usize) -> Result<&u8> {
        self.bytes.get(self.index + ahead)
            .ok_or_else(|| Error::from_code(ErrorCode::UnexpectedEndOfBytes))
    }

    pub fn peek_marker(&mut self) -> Result<Marker> {
        let marker_byte = *self.get_byte(0)?;

        let marker = match marker_byte {
            // String
            TINY_STRING..=TINY_STRING_MAX => Marker::String((marker_byte - TINY_STRING) as usize),
            STRING_8 => Marker::String(*self.get_byte(1)? as usize),
            STRING_16 => {
                let b8 = *self.get_byte(2)?;
                let b7 = self.bytes[self.index + 1];

                Marker::String(bytes_to_usize!(b7, b8))
            }
            STRING_32 => {
                let b8 = *self.get_byte(4)?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];

                Marker::String(bytes_to_usize!(b5, b6, b7, b8))
            }

            // Map
            TINY_MAP..=TINY_MAP_MAX => Marker::Map((marker_byte - TINY_MAP) as usize),
            MAP_8 => Marker::Map(*self.get_byte(1)? as usize),
            MAP_16 => {
                let b8 = *self.get_byte(self.index + 2)?;
                let b7 = self.bytes[self.index + 1];

                Marker::Map(bytes_to_usize!(b7, b8))
            }
            MAP_32 => {
                let b8 = *self.get_byte(4)?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];

                Marker::Map(bytes_to_usize!(b5, b6, b7, b8))
            },
            MAP_STREAM => Marker::Map(std::usize::MAX),
            //
            // Struct
            TINY_STRUCT..=TINY_STRUCT_MAX => Marker::Struct((marker_byte - TINY_STRUCT) as usize),
            STRUCT_8 => Marker::Struct(*self.get_byte(1)? as usize),
            STRUCT_16 => {
                let b8 = *self.get_byte(2)?;
                let b7 = self.bytes[self.index + 1];

                Marker::Struct(bytes_to_usize!(b7, b8))
            }

            // List
            TINY_LIST..=TINY_LIST_MAX => Marker::List((marker_byte - TINY_LIST) as usize),
            LIST_8 => Marker::List(*self.get_byte(1)? as usize),
            LIST_16 => {
                let b8 = *self.get_byte(2)?;
                let b7 = self.bytes[self.index + 1];

                Marker::List(bytes_to_usize!(b7, b8))
            }
            LIST_32 => {
                let b8 = *self.get_byte(4)?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];

                Marker::List(bytes_to_usize!(b5, b6, b7, b8))
            }
            LIST_STREAM => Marker::List(std::usize::MAX),

            NULL => Marker::Null,
            TRUE => Marker::True,
            FALSE => Marker::False,

            INT_8 => Marker::I64(i64::from(i8::from_be_bytes([*self.get_byte(1)?]))),
            INT_16 => {
                let b2 = *self.get_byte(2)?;
                let b1 = self.bytes[self.index + 1];

                let n = i16::from_be_bytes([b1, b2]);
                Marker::I64(i64::from(n))
            }
            INT_32 => {
                let b4 = *self.get_byte(4)?;
                let b3 = self.bytes[self.index + 3];
                let b2 = self.bytes[self.index + 2];
                let b1 = self.bytes[self.index + 1];

                let n = i32::from_be_bytes([b1, b2, b3, b4]);
                Marker::I64(i64::from(n))
            }
            INT_64 => {
                let b8 = *self.get_byte(8)?;
                let b7 = self.bytes[self.index + 7];
                let b6 = self.bytes[self.index + 6];
                let b5 = self.bytes[self.index + 5];
                let b4 = self.bytes[self.index + 4];
                let b3 = self.bytes[self.index + 3];
                let b2 = self.bytes[self.index + 2];
                let b1 = self.bytes[self.index + 1];

                Marker::I64(i64::from_be_bytes([b1, b2, b3, b4, b5, b6, b7, b8]))
            }
            FLOAT_64 => {
                let b8 = *self.get_byte(8)?;
                let b7 = self.bytes[self.index + 7];
                let b6 = self.bytes[self.index + 6];
                let b5 = self.bytes[self.index + 5];
                let b4 = self.bytes[self.index + 4];
                let b3 = self.bytes[self.index + 3];
                let b2 = self.bytes[self.index + 2];
                let b1 = self.bytes[self.index + 1];

                let n = f64::from_bits(u64::from_be_bytes([b1, b2, b3, b4, b5, b6, b7, b8]));
                Marker::F64(n)
            }

            END_OF_STREAM => Marker::EOS,

            BYTES_8 => Marker::Bytes(*self.get_byte(1)? as usize),
            BYTES_16 => {
                let b8 = *self.get_byte(2)?;
                let b7 = self.bytes[self.index + 1];

                Marker::Bytes(bytes_to_usize!(b7, b8))
            }
            BYTES_32 => {
                let b8 = *self.get_byte(4)?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];

                Marker::Bytes(bytes_to_usize!(b5, b6, b7, b8))
            }

            0..=0x7F | 0xF0..=0xFF => {
                Marker::I64(i8::from_be_bytes([marker_byte]).into())
            }

            // TODO: JS's PackstreamV1 interprets unknown markers as Integers
            _ => return Err(Error::from_code(ErrorCode::ExpectedMarkerByte)),
        };

        self.peeked = Some(marker.clone());

        Ok(marker)
    }

    fn get_marker(&mut self) -> Result<Marker> {
        let marker_byte = self.bytes[self.index];
        match marker_byte {
            _ => Ok(Marker::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_try_peek {
        ($($bytes:expr => $marker:expr),* $(,)*) => {
            $(assert_eq!($marker, ByteReader { bytes: &$bytes, index: 0, peeked: None }.peek_marker().unwrap());)*
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
