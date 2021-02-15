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

        let marker = match marker_byte {
            // String
            TINY_STRING..=TINY_STRING_MAX => Marker::String(usize::from(marker_byte - TINY_STRING)),
            STRING_8 => Marker::String(usize::from(self.bytes[self.index + 1])),
            STRING_16 => {
                let b7 = self.bytes[self.index + 1];
                let b8 = self.bytes[self.index + 2];
                let len = usize::from_be_bytes([0, 0, 0, 0, 0, 0, b7, b8]);

                Marker::String(len)
            }
            STRING_32 => {
                let b5 = self.bytes[self.index + 1];
                let b6 = self.bytes[self.index + 2];
                let b7 = self.bytes[self.index + 3];
                let b8 = self.bytes[self.index + 4];
                Marker::String(usize::from_be_bytes([0, 0, 0, 0, b5, b6, b7, b8]))
            }

            // Map
            TINY_MAP..=TINY_MAP_MAX => Marker::Map(usize::from(marker_byte - TINY_MAP)),
            MAP_8 => Marker::Map(usize::from(self.bytes[self.index + 1])),
            MAP_16 => {
                let b7 = self.bytes[self.index + 1];
                let b8 = self.bytes[self.index + 2];
                Marker::Map(usize::from_be_bytes([0, 0, 0, 0, 0, 0, b7, b8]))
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

// peek_marker - returns marker, possibly with length information
// it will also move byte index accordingly. Should it instead remove bytes
// from index?

// Returns Marker and amount of bytes or size of collection
// fn get_marker_hint(byte: u8) -> MarkerHint {
// }

// fn read_header(bytes: &[u8]) -> MarkerHint {
//     if bytes.len() == 0 {
//         unimplemented!();
//     }

//     let (mut marker, size_bytes) = get_marker_hint(bytes[0]);
//     let size_chunk = &bytes[1..];

//     match marker {
//         m @ Marker::Int(_) | m @ Marker::Float64(_) => (m, size_bytes)
//         _ => unimplemented!()
//     };

//     println!("{:?} {:?}", marker, size_bytes);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_marker_hint_test () {
//         for i in 0u8..15u8 {
//             let len = usize::from(i);
//             assert_eq!(get_marker_hint(TINY_STRING + i), (Marker::String(len), 0));
//             assert_eq!(get_marker_hint(TINY_STRUCT + i), (Marker::Struct(len), 0));
//             assert_eq!(get_marker_hint(TINY_LIST + i), (Marker::List(len), 0));
//             assert_eq!(get_marker_hint(TINY_MAP + i), (Marker::Map(len), 0));
//         }
//     }

//     #[test]
//     fn read_header_test() {
//         read_header(&[INT_8, 121]);
//     }
// }

// Where deserializer will parse input, a method is needed that will produce Marker type
// basing on the value of u8 and also return information on how many bytes should be consumed
// in order to read the length of the element if any.
// Size: 0-4 bytes

// enum used to hint parser if any more bytes should be read in order to
// infer all necessary information
// enum ReadHint {
//     EndOfStream, // Read until end of stream marker
//     Bits(u8), // Read next N bits
//     None, // Do not read any additional bits
// }

// There should be a reader function that would accept &mut Deserializer and would try to return Marker
// from_bytes should return number of bytes that should be consumed
// fn from_bytes (bytes: &[u8]) -> () {
//     let (marker, consume_hint) = match bytes[0] {
//         TINY_STRING..=TINY_STRING_MAX => (Marker::String(usize::from(bytes[0])), None),
//         TINY_MAP..=TINY_MAP_MAX => (Marker::Map(usize::from(bytes[0])), None),
//         TINY_LIST..=TINY_LIST_MAX => (Marker::List(usize::from(bytes[0])), None),
//         TINY_STRUCT..=TINY_STRUCT_MAX => (Marker::Struct(usize::from(bytes[0])), None),
//         NULL => (Marker::Null, None),
//         FLOAT_64 => (Marker::Float64(0.0), Some(8)),
//         TRUE => (Marker::True, None),
//         FALSE => (Marker::False, None),
//         INT_8 => (Marker::Int(0), Some(1)),
//         INT_16 => (Marker::Int(0), Some(2)),
//         INT_32 => (Marker::Int(0), Some(4)),
//         INT_64 => (Marker::Int(0), Some(8)),
//         BYTES_8 => (Marker::Bytes(0), Some(1)),
//         BYTES_16 => (Marker::Bytes(0), Some(2)),
//         BYTES_32 => (Marker::Bytes(0), Some(4)),
//         STRING_8 => (Marker::String(0), Some(1)),
//         STRING_16 => (Marker::String(0), Some(2)),
//         STRING_32 => (Marker::String(0), Some(4)),
//         MAP_8 => (Marker::Map(0), Some(1)),
//         MAP_16 => (Marker::Map(0), Some(2)),
//         MAP_32 => (Marker::Map(0), Some(4)),
//         MAP_STREAM => (Marker::Map(0), None), // Is it the proper way to handle streams?
//         LIST_8 => (Marker::List(0), Some(1)),
//         LIST_16 => (Marker::List(0), Some(2)),
//         LIST_32 => (Marker::List(0), Some(4)),
//         LIST_STREAM => (Marker::List(0), None), // What to do? What to do?
//         STRUCT_8 => (Marker::Struct(0), Some(1)),
//         STRUCT_16 => (Marker::Struct(0), Some(2)),
//         _ => unimplemented!(),
//     };

//     // if let Some(no_of_bytes) = consume_hint {
//     //     let bytes = &bytes[1..1+no_of_bytes];
//     //     match &mut marker {
//     //         Marker::Int(value) => {
//     //             *value = i64::from_be_bytes(bytes);
//     //         }
//     //         Marker::Map(l) | Marker::List(l) | Marker::Struct(l) | Marker::Bytes(l) | Marker::String(l) => {
//     //             *l = usize::from_be_bytes(bytes);
//     //         }
//     //         _ => unimplemented!()
//     //     };
//     // }
// }
