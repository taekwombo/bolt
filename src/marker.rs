use super::constants::marker::*;
use super::error::{PackstreamError, PackstreamResult};
use std::fmt;

/// Represents the marker byte of the core packstream data types.
///
/// The core data types can be one of:
///
/// * `Null`        missing or empty value
/// * `Boolean`     **true** or **false**
/// * `Integer`     signed 64-bit integer
/// * `Float`       64-bit floating point number
/// * `Bytes`       byte array
/// * `String`      unicode text, **UTF-8**
/// * `List`        ordered collection of values
/// * `Dictionary`  collection of key-value entries (no order guaranteed)
/// * `Structure`   composite value with a type signature
#[derive(Debug, PartialEq, Clone)]
pub enum Marker {
    I64(i64),
    F64(f64),
    String(usize),
    List(usize),
    Bytes(usize),
    Map(usize),
    Struct(usize),
    Null,
    True,
    False,
}

fn type_size_exceeded(marker_type: &str, len: usize, max_len: usize) -> PackstreamError {
    PackstreamError::create(format!("Cannot pack {} marker with size {}. Maximum size available for this data type is {}", marker_type, len, max_len))
}

impl Marker {
    pub(crate) fn inc_size(&mut self, size: usize) -> PackstreamResult<()> {
        match self {
            Self::String(len) => *len += size,
            Self::List(len) => *len += size,
            Self::Bytes(len) => *len += size,
            Self::Map(len) => *len += size,
            Self::Struct(len) => *len += size,
            marker => {
                return Err(PackstreamError::create(format!(
                    "Unexpected Marker {}, expected Marker with size",
                    marker
                )))
            }
        };
        Ok(())
    }

    pub(crate) fn append_to_vec(&self, vec: &mut Vec<u8>) -> PackstreamResult<()> {
        macro_rules! extend {
            ($arr:expr) => {
                vec.extend_from_slice(&$arr)
            };
            ($marker:ident, $num:ident, 2) => {
                vec.extend_from_slice(&[$marker, ($num >> 8) as u8, $num as u8])
            };
            ($marker:ident, $num:ident, 4) => {
                vec.extend_from_slice(&[$marker, ($num >> 24) as u8, ($num >> 16) as u8, ($num >> 8) as u8, $num as u8])
            };
            ($marker: ident, $num:ident, 8) => {{
                vec.push($marker);
                vec.extend_from_slice(&$num.to_be_bytes());
            }};
        }

        match *self {
            Self::I64(int) => match int {
                -0x10..=0x7F => extend!([int as u8]),
                -0x80..=-0x11 => extend!([INT_8, int as u8]),
                -0x8000..=-0x81 | 0x80..=0x7FFF => extend!(INT_16, int, 2),
                -0x8000_0000..=-0x8001 | 0x8000..=0x7FFF_FFFF => extend!(INT_32, int, 4),
                _ => extend!(INT_64, int, 8),
            },
            Self::F64(float) => {
                vec.push(FLOAT_64);
                vec.extend_from_slice(&float.to_bits().to_be_bytes());
            },
            Self::String(size) => match size {
                0x0..=0xF => extend!([TINY_STRING + size as u8]),
                0x10..=0xFF => extend!([STRING_8, size as u8]),
                0x100..=0xFFFF => extend!(STRING_16, size, 2),
                0x10000..=0xFFFF_FFFF => extend!(STRING_32, size, 4),
                _ => return Err(type_size_exceeded("String", size, 0xFFFF_FFFF)),

            },
            Self::List(size) => match size {
                0x0..=0xF => extend!([TINY_LIST + size as u8]),
                0x10..=0xFF => extend!([LIST_8, size as u8]),
                0x100..=0xFFFF => extend!(LIST_16, size, 2),
                0x10000..=0xFFFF_FFFF => extend!(LIST_32, size, 4),
                _ => vec.push(LIST_STREAM),
            },

            Self::Bytes(size) => match size {
                0x0..=0xFF => extend!([BYTES_8, size as u8]),
                0x100..=0xFFFF => extend!(BYTES_16, size, 2),
                0x10000..=0xFFFF_FFFF => extend!(BYTES_32, size, 4),
                _ => return Err(type_size_exceeded("Bytes", size, 0xFFFF_FFFF)),
            },
            Self::Map(size) => match size {
                0x0..=0xF => extend!([TINY_MAP + size as u8]),
                0x10..=0xFF => extend!([MAP_8, size as u8]),
                0x100..=0xFFFF => extend!(MAP_16, size, 2),
                0x10000..=0xFFFF_FFFF => extend!(MAP_32, size, 4),
                _ => vec.push(MAP_STREAM),
            },
            Self::Struct(size) => match size {
                0x0..=0xF => extend!([TINY_STRUCT + size as u8]),
                0x10..=0xFF => extend!([STRUCT_8, size as u8]),
                0x100..=0xFFFF => extend!(STRUCT_16, size, 2),
                _ => return Err(type_size_exceeded("Struct", size, 0xFFFF)),
            },
            Self::Null => vec.push(NULL),
            Self::True => vec.push(TRUE),
            Self::False => vec.push(FALSE),
        };

        Ok(())
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

    macro_rules! assert_marker_extend_vec {
        ($($marker:expr => $expected:expr),* $(,)*) => {
            $({
                let mut _tmp: Vec<u8> = vec![];
                let _res = $marker.append_to_vec(&mut _tmp);

                assert!(_res.is_ok());
                assert_eq!(_tmp, $expected);
            })*
        };
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_marker_append_to_vec() {
        assert_marker_extend_vec! {
            Marker::I64(127) => [127],
            Marker::I64(-16) => [240],
            Marker::I64(-128) => [INT_8, 128],
            Marker::I64(200) => [INT_16, 0, 200],
            Marker::I64(-200) => [INT_16, 255, 56],
            Marker::I64(-129) => [INT_16, 255, 127],
            Marker::I64(100_000) => [INT_32, 0, 1, 134, 160],
            Marker::I64(-100_000) => [INT_32, 255, 254, 121, 96],
            Marker::I64(3_000_000_000) => [INT_64, 0, 0, 0, 0, 178, 208, 94, 0],
            Marker::I64(-3_000_000_000) => [INT_64, 255, 255, 255, 255, 77, 47, 162, 0],
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
        };

        let mut test_vec = vec![];

        assert!(Marker::Struct(256 * 256 * 256).append_to_vec(&mut test_vec).is_err());
        assert!(Marker::String(256 * 256 * 256 * 256).append_to_vec(&mut test_vec).is_err());
        assert!(Marker::Bytes(256 * 256 * 256 * 256).append_to_vec(&mut test_vec).is_err());
    }
}
