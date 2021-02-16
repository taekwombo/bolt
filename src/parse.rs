use super::error::{Error, ErrorCode, Result};
use super::marker::Marker;
use super::marker_bytes::*;

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

    pub fn peek_marker(&mut self) -> Result<Marker> {
        let marker_byte = self.bytes[self.index];

        // Optimize usize by using arighmetic
        let marker = match marker_byte {
            // String
            TINY_STRING..=TINY_STRING_MAX => Marker::String(usize::from(marker_byte - TINY_STRING)),
            STRING_8 => Marker::String(usize::from(self.bytes[self.index + 1])),
            STRING_16 => {
                let b8 = self.bytes
                    .get(self.index + 2)
                    .ok_or_else(|| Error::from_code(ErrorCode::UnexpectedEndOfBytes))?;
                let b7 = self.bytes.get(self.index + 1).unwrap();
                let len = *b7 as usize * 256 + *b8 as usize;

                Marker::String(len)
            }
            STRING_32 => {
                let b8 = self.bytes.get(self.index + 4)
                    .ok_or_else(|| Error::from_code(ErrorCode::UnexpectedEndOfBytes))?;
                let b7 = self.bytes[self.index + 3];
                let b6 = self.bytes[self.index + 2];
                let b5 = self.bytes[self.index + 1];
                Marker::String(usize::from_be_bytes([0, 0, 0, 0, b5, b6, b7, *b8]))
            }

            // Map
            TINY_MAP..=TINY_MAP_MAX => Marker::Map(usize::from(marker_byte - TINY_MAP)),
            MAP_8 => Marker::Map(usize::from(self.bytes[self.index + 1])),
            MAP_16 => {
                let b8 = self.bytes.get(self.index + 2)
                    .ok_or_else(|| Error::from_code(ErrorCode::UnexpectedEndOfBytes))?;
                let b7 = self.bytes[self.index + 1];
                Marker::Map(usize::from_be_bytes([0, 0, 0, 0, 0, 0, b7, *b8]))
            }
            MAP_32 => {
                let b5 = self.bytes[self.index + 1];
                let b6 = self.bytes[self.index + 2];
                let b7 = self.bytes[self.index + 3];
                let b8 = self.bytes[self.index + 4];
                Marker::Map(usize::from_be_bytes([0, 0, 0, 0, b5, b6, b7, b8]))
            },
            MAP_STREAM => Marker::Map(std::usize::MAX),
            //
            // Struct
            TINY_STRUCT..=TINY_STRUCT_MAX => Marker::Struct(usize::from(marker_byte - TINY_STRUCT)),
            STRUCT_8 => Marker::Struct(usize::from(self.bytes[self.index + 1])),
            STRUCT_16 => {
                let b7 = self.bytes[self.index + 1];
                let b8 = self.bytes[self.index + 2];
                Marker::Struct(usize::from_be_bytes([0, 0, 0, 0, 0, 0, b7, b8]))
            }

            // List
            TINY_LIST..=TINY_LIST_MAX => Marker::List(usize::from(marker_byte - TINY_LIST)),
            LIST_8 => Marker::List(usize::from(self.bytes[self.index + 1])),
            LIST_16 => {
                let b7 = self.bytes[self.index + 1];
                let b8 = self.bytes[self.index + 2];
                Marker::List(usize::from_be_bytes([0, 0, 0, 0, 0, 0, b7, b8]))
            }
            LIST_32 => {
                let b5 = self.bytes[self.index + 1];
                let b6 = self.bytes[self.index + 2];
                let b7 = self.bytes[self.index + 3];
                let b8 = self.bytes[self.index + 4];
                Marker::List(usize::from_be_bytes([0, 0, 0, 0, b5, b6, b7, b8]))
            }
            LIST_STREAM => Marker::List(std::usize::MAX),

            NULL => Marker::Null,
            TRUE => Marker::True,
            FALSE => Marker::False,

            INT_8 => Marker::I64(i64::from(self.bytes[self.index + 1])),
            INT_16 => {
                let b1 = self.bytes[self.index + 1];
                let b2 = self.bytes[self.index + 2];
                let n = i16::from_be_bytes([b1, b2]);
                Marker::I64(i64::from(n))
            }
            INT_32 => {
                let b1 = self.bytes[self.index + 1];
                let b2 = self.bytes[self.index + 2];
                let b3 = self.bytes[self.index + 3];
                let b4 = self.bytes[self.index + 4];
                let n = i32::from_be_bytes([b1, b2, b3, b4]);
                Marker::I64(i64::from(n))
            }
            INT_64 => {
                let b1 = self.bytes[self.index + 1];
                let b2 = self.bytes[self.index + 2];
                let b3 = self.bytes[self.index + 3];
                let b4 = self.bytes[self.index + 4];
                let b5 = self.bytes[self.index + 5];
                let b6 = self.bytes[self.index + 6];
                let b7 = self.bytes[self.index + 7];
                let b8 = self.bytes[self.index + 8];
                let n = i64::from_be_bytes([b1, b2, b3, b4, b5, b6, b7, b8]);
                Marker::I64(n)
            }
            FLOAT_64 => {
                let b1 = self.bytes[self.index + 1];
                let b2 = self.bytes[self.index + 2];
                let b3 = self.bytes[self.index + 3];
                let b4 = self.bytes[self.index + 4];
                let b5 = self.bytes[self.index + 5];
                let b6 = self.bytes[self.index + 6];
                let b7 = self.bytes[self.index + 7];
                let b8 = self.bytes[self.index + 8];
                let n = f64::from_bits(u64::from_be_bytes([b1, b2, b3, b4, b5, b6, b7, b8]));
                Marker::F64(n)
            }

            END_OF_STREAM => Marker::EOS,

            BYTES_8 => Marker::Bytes(usize::from(self.bytes[self.index + 1])),
            BYTES_16 => {
                let b7 = self.bytes[self.index + 1];
                let b8 = self.bytes[self.index + 2];
                let n = usize::from_be_bytes([0, 0, 0, 0, 0, 0, b7, b8]);
                Marker::Bytes(n)
            }
            BYTES_32 => {
                let b5 = self.bytes[self.index + 1];
                let b6 = self.bytes[self.index + 2];
                let b7 = self.bytes[self.index + 3];
                let b8 = self.bytes[self.index + 4];
                let n = usize::from_be_bytes([0, 0, 0, 0, b5, b6, b7, b8]);
                Marker::Bytes(n)
            }

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
        ($bytes:expr, $marker:expr) => {
            assert_eq!($marker, ByteReader { bytes: $bytes, index: 0, peeked: None }.peek_marker().unwrap());
        }
    }

    #[test]
    fn test_peek_marker() {
        assert_try_peek!(&[TINY_MAP], Marker::Map(0));
        assert_try_peek!(&[TINY_MAP + 10], Marker::Map(10));
        assert_try_peek!(&[MAP_8, 20], Marker::Map(20));
        assert_try_peek!(&[MAP_16, 1, 0], Marker::Map(256));
        assert_try_peek!(&[MAP_32, 0, 1, 0, 0], Marker::Map(256 * 256));
        assert_try_peek!(&[MAP_STREAM], Marker::Map(std::usize::MAX));

        assert_try_peek!(&[TINY_STRING], Marker::String(0));
        assert_try_peek!(&[TINY_STRING + 10], Marker::String(10));
        assert_try_peek!(&[STRING_8, 20], Marker::String(20));
        assert_try_peek!(&[STRING_16, 1, 0], Marker::String(256));
        assert_try_peek!(&[STRING_32, 0, 1, 0, 0], Marker::String(256 * 256));

        assert_try_peek!(&[TINY_STRUCT], Marker::Struct(0));
        assert_try_peek!(&[STRUCT_8, 20], Marker::Struct(20));
        assert_try_peek!(&[STRUCT_16, 1, 0], Marker::Struct(256));

        assert_try_peek!(&[TINY_LIST + 5], Marker::List(5));
        assert_try_peek!(&[LIST_8, 100], Marker::List(100));
        assert_try_peek!(&[LIST_16, 1, 0], Marker::List(256));
        assert_try_peek!(&[LIST_32, 0, 1, 0, 0], Marker::List(256 * 256));
        assert_try_peek!(&[LIST_STREAM], Marker::List(std::usize::MAX));

        assert_try_peek!(&[BYTES_8, 1], Marker::Bytes(1));
        assert_try_peek!(&[BYTES_16, 1, 0], Marker::Bytes(256));
        assert_try_peek!(&[BYTES_32, 0, 1, 0, 0], Marker::Bytes(256 * 256));

        assert_try_peek!(&[NULL], Marker::Null);
        assert_try_peek!(&[TRUE], Marker::True);
        assert_try_peek!(&[FALSE], Marker::False);

        assert_try_peek!(&[END_OF_STREAM], Marker::EOS);

        assert_try_peek!(&[INT_8, 10], Marker::I64(10));
        assert_try_peek!(&[INT_16, 1, 0], Marker::I64(256));
        assert_try_peek!(&[INT_32, 0, 1, 0, 0], Marker::I64(256 * 256));
        assert_try_peek!(&[INT_64, 0, 0, 1, 0, 0, 0, 0, 0], Marker::I64(256 * 256 * 256 * 256 * 256));

        assert_try_peek!(&[FLOAT_64, 0, 0, 0, 0, 0, 0, 0, 0], Marker::F64(0.0));
    }
}
