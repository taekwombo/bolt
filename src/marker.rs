use super::constants::marker::*;
use super::error::{Error, ErrorCode, Result};
use std::convert::TryFrom;
use std::fmt;

macro_rules! to_bytes {
    ($marker_b:expr, $t:ty, $e:expr) => {{
        let mut header = vec![$marker_b];
        header.extend_from_slice(&<$t>::try_from($e).unwrap().to_be_bytes());
        header
    }};
    // i64
    ($marker_b:expr, $e:expr) => {{
        let mut header = vec![$marker_b];
        header.extend_from_slice(&$e.to_be_bytes());
        header
    }};
    // f64
    ($e:expr) => {{
        let mut header = vec![FLOAT_64];
        header.extend_from_slice(&$e.to_bits().to_be_bytes());
        header
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub enum Marker {
    I64(i64),      // TinyInt, Int8, Int16, Int32, Int64
    F64(f64),      // Float64
    String(usize), // TinyString, String8, String16, String32
    List(usize),   // TinyList, List8, List16, List32, ListStream
    Bytes(usize),  // Bytes8, Bytes16, Bytes32
    Map(usize),    // TinyMap, Map8, Map16, Map32, MapStream
    Struct(usize), // TinyStruct, Struct8, Struct16
    Null,
    True,
    False,
    EOS, // End of Stream
}

impl Marker {
    pub(crate) fn inc_size(&mut self, size: usize) -> Result<()> {
        match self {
            Self::String(len) => *len += size,
            Self::List(len) => *len += size,
            Self::Bytes(len) => *len += size,
            Self::Map(len) => *len += size,
            Self::Struct(len) => *len += size,
            _ => return Err(Error::from_code(ErrorCode::ExpectedSizeMarker)),
        };
        Ok(())
    }

    pub(crate) fn to_vec(&self) -> Result<Vec<u8>> {
        let bytes_vec = match self {
            Self::String(len) => match len {
                0x0..=0xF => vec![TINY_STRING + u8::try_from(*len).unwrap()],
                0x10..=0xFF => vec![STRING_8, u8::try_from(*len).unwrap()],
                0x100..=0xFFFF => to_bytes!(STRING_16, i16, *len),
                0x10000..=0xFFFF_FFFF => to_bytes!(STRING_32, i32, *len),
                _ => return Err(Error::make("String too long to pack.")),
            },
            Self::I64(int) => match int {
                -0x10..=0x7F => i8::try_from(*int).unwrap().to_be_bytes().to_vec(),
                -0x80..=-0x11 => to_bytes!(INT_8, i8, *int),
                -0x8000..=-0x81 | 0x80..=0x7FFF => to_bytes!(INT_16, i16, *int),
                -0x8000_0000..=-0x8001 | 0x8000..=0x7FFF_FFFF => to_bytes!(INT_32, i32, *int),
                _ => to_bytes!(INT_64, int),
            },
            Self::List(len) => match len {
                0x0..=0xF => vec![TINY_LIST + u8::try_from(*len).unwrap()],
                0x10..=0xFF => vec![LIST_8, u8::try_from(*len).unwrap()],
                0x100..=0xFFFF => to_bytes!(LIST_16, u16, *len),
                0x10000..=0xFFFF_FFFF => to_bytes!(LIST_32, u32, *len),
                _ => vec![LIST_STREAM],
            },
            Self::Bytes(len) => match len {
                0x0..=0xFF => vec![BYTES_8, u8::try_from(*len).unwrap()],
                0x100..=0xFFFF => to_bytes!(BYTES_16, u16, *len),
                0x10000..=0xFFFF_FFFF => to_bytes!(BYTES_32, u32, *len),
                _ => return Err(Error::make("Bytes too long to pack.")),
            },
            Self::Map(len) => match len {
                0x0..=0xF => vec![TINY_MAP + u8::try_from(*len).unwrap()],
                0x10..=0xFF => vec![MAP_8, u8::try_from(*len).unwrap()],
                0x100..=0xFFFF => to_bytes!(MAP_16, u16, *len),
                0x10000..=0xFFFF_FFFF => to_bytes!(MAP_32, u32, *len),
                _ => vec![MAP_STREAM],
            },
            Self::Struct(len) => match len {
                0x0..=0xF => vec![TINY_STRUCT + u8::try_from(*len).unwrap()],
                0x10..=0xFF => vec![STRUCT_8, u8::try_from(*len).unwrap()],
                0x100..=0xFFFF => to_bytes!(STRUCT_16, u16, *len),
                _ => return Err(Error::make("Struct too big to pack.")),
            },
            Self::Null => vec![NULL],
            Self::True => vec![TRUE],
            Self::False => vec![FALSE],
            Self::F64(value) => to_bytes!(value),
            Self::EOS => vec![END_OF_STREAM],
        };
        Ok(bytes_vec)
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Marker::I64(num) => write!(f, "Marker::I64({})", num),
            Marker::F64(num) => write!(f, "Marker::F64({})", num),
            Marker::Null => write!(f, "Marker::Null"),
            Marker::True => write!(f, "Marker::True"),
            Marker::False => write!(f, "Marker::False"),
            Marker::EOS => write!(f, "Marker::EOS"),
            Marker::Struct(len) => write!(f, "Marker::Struct({})", len),
            Marker::Map(len) => write!(f, "Marker::Map({})", len),
            Marker::List(len) => write!(f, "Marker::List({})", len),
            Marker::Bytes(len) => write!(f, "Marker::Bytes({})", len),
            Marker::String(len) => write!(f, "Marker::String({})", len),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_marker_to_vec {
        ($($marker:expr => $expected:expr),* $(,)*) => {
            $(assert_eq!($marker.to_vec().unwrap(), $expected);)*
        };
    }

    #[test]
    fn test_marker_to_vec() {
        assert_marker_to_vec! {
            Marker::I64(127) => [127],
            Marker::I64(-16) => [240],
            Marker::I64(-128) => [INT_8, 128],
            Marker::I64(200) => [INT_16, 0, 200],
            Marker::I64(-200) => [INT_16, 255, 56],
            Marker::I64(-129) => [INT_16, 255, 127],
            Marker::I64(100000) => [INT_32, 0, 1, 134, 160],
            Marker::I64(-100000) => [INT_32, 255, 254, 121, 96],
            Marker::I64(3000000000) => [INT_64, 0, 0, 0, 0, 178, 208, 94, 0],
            Marker::I64(-3000000000) => [INT_64, 255, 255, 255, 255, 77, 47, 162, 0],
            Marker::F64(100f64) => [FLOAT_64, 64, 89, 0, 0, 0, 0, 0, 0],
            Marker::String(10) => [TINY_STRING + 10],
            Marker::String(64) => [STRING_8, 64],
            Marker::String(256) => [STRING_16, 1, 0],
            Marker::String(256 * 256) => [STRING_32, 0, 1, 0, 0],
            Marker::String(256 * 256 * 256) => [STRING_32, 1, 0, 0, 0],
            Marker::List(10) => [TINY_LIST + 10],
            Marker::List(255) => [LIST_8, 255],
            Marker::List(256) => [LIST_16, 1, 0],
            Marker::List(256 * 256) => [LIST_32, 0, 1, 0, 0],
            Marker::List(256 * 256 * 256 * 256) => [LIST_STREAM],
            Marker::Bytes(10) => [BYTES_8, 10],
            Marker::Bytes(256) => [BYTES_16, 1, 0],
            Marker::Bytes(256 * 256) => [BYTES_32, 0, 1, 0, 0],
            Marker::Map(10) => [TINY_MAP + 10],
            Marker::Map(50) => [MAP_8, 50],
            Marker::Map(256) => [MAP_16, 1, 0],
            Marker::Map(256 * 256) => [MAP_32, 0, 1, 0, 0],
            Marker::Map(256 * 256 * 256 * 256) => [MAP_STREAM],
            Marker::Struct(10) => [TINY_STRUCT + 10],
            Marker::Struct(50) => [STRUCT_8, 50],
            Marker::Struct(256) => [STRUCT_16, 1, 0],
            Marker::Null => [NULL],
            Marker::True => [TRUE],
            Marker::False => [FALSE],
            Marker::EOS => [END_OF_STREAM],
        };

        assert!(Marker::Struct(256 * 256 * 256).to_vec().is_err());
        assert!(Marker::String(256 * 256 * 256 * 256).to_vec().is_err());
        assert!(Marker::Bytes(256 * 256 * 256 * 256).to_vec().is_err());
    }
}
