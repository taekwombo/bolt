pub(crate) const STRUCTURE_NAME: &str = "__BOLT_STRUCTURE_SERDE_NAME__";
pub(crate) const STRUCTURE_SIG_KEY: &str = "__BOLT_STRUCTURE_SIGNATURE_KEY__";
pub(crate) const STRUCTURE_SIG_KEY_B: &[u8] = b"__BOLT_STRUCTURE_SIGNATURE_KEY__";
pub(crate) const STRUCTURE_FIELDS_KEY: &str = "__BOLT_STRUCTURE_FIELDS_KEY__";
pub(crate) const STRUCTURE_FIELDS_KEY_B: &[u8] = b"__BOLT_STRUCTURE_FIELDS_KEY__";
pub(crate) const SIG_KEY: &str = "signature";

pub mod marker {
    pub const TINY_STRING: u8 = 0x80;
    pub const TINY_STRING_MAX: u8 = 0x8F;
    pub const TINY_LIST: u8 = 0x90;
    pub const TINY_LIST_MAX: u8 = 0x9F;
    pub const TINY_MAP: u8 = 0xA0;
    pub const TINY_MAP_MAX: u8 = 0xAF;
    pub const TINY_STRUCT: u8 = 0xB0;
    pub const TINY_STRUCT_MAX: u8 = 0xBF;
    pub const NULL: u8 = 0xC0;
    pub const FLOAT_64: u8 = 0xC1;
    pub const TRUE: u8 = 0xC2;
    pub const FALSE: u8 = 0xC3;
    pub const INT_8: u8 = 0xC8;
    pub const INT_16: u8 = 0xC9;
    pub const INT_32: u8 = 0xCA;
    pub const INT_64: u8 = 0xCB;
    pub const BYTES_8: u8 = 0xCC;
    pub const BYTES_16: u8 = 0xCD;
    pub const BYTES_32: u8 = 0xCE;
    pub const STRING_8: u8 = 0xD0;
    pub const STRING_16: u8 = 0xD1;
    pub const STRING_32: u8 = 0xD2;
    pub const LIST_8: u8 = 0xD4;
    pub const LIST_16: u8 = 0xD5;
    pub const LIST_32: u8 = 0xD6;
    pub const LIST_STREAM: u8 = 0xD7;
    pub const MAP_8: u8 = 0xD8;
    pub const MAP_16: u8 = 0xD9;
    pub const MAP_32: u8 = 0xDA;
    pub const MAP_STREAM: u8 = 0xDB;
    pub const STRUCT_8: u8 = 0xDC;
    pub const STRUCT_16: u8 = 0xDD;
    pub const END_OF_STREAM: u8 = 0xDF;
}

pub mod message {
    pub const INIT: u8 = 0x01;
    pub const ACK_FAILURE: u8 = 0x0E;
    pub const RESET: u8 = 0x0F;
    pub const RUN: u8 = 0x10;
    pub const DISCARD_ALL: u8 = 0x2F;
    pub const PULL_ALL: u8 = 0x3F;
    pub const SUCCESS: u8 = 0x70;
    pub const IGNORED: u8 = 0x7E;
    pub const FAILURE: u8 = 0x7F;
    pub const RECORD: u8 = 0x71;
}

pub mod structure {
    pub const NODE: u8 = 0x4E;
    pub const RELATIONSHIP: u8 = 0x52;
    pub const UNBOUND_RELATIONSHIP: u8 = 0x72;
    pub const PATH: u8 = 0x50;
    pub const DATE: u8 = 0x44;
    pub const TIME: u8 = 0x54;
    pub const LOCAL_TIME: u8 = 0x74;
    pub const DATE_TIME: u8 = 0x46;
    pub const DATE_TIME_ZONE_ID: u8 = 0x66;
    pub const LOCAL_DATE_TIME: u8 = 0x64;
    pub const DURATION: u8 = 0x45;
    pub const POINT_2D: u8 = 0x58;
    pub const POINT_3D: u8 = 0x59;
}

