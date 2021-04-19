pub(crate) const STRUCTURE_NAME: &str = "__BOLT_STRUCTURE_SERDE_NAME__";
pub(crate) const STRUCTURE_SIG_KEY: &str = "__BOLT_STRUCTURE_SIGNATURE_KEY__";
pub(crate) const STRUCTURE_SIG_KEY_B: &[u8] = b"__BOLT_STRUCTURE_SIGNATURE_KEY__";
pub(crate) const STRUCTURE_FIELDS_KEY: &str = "__BOLT_STRUCTURE_FIELDS_KEY__";
pub(crate) const STRUCTURE_FIELDS_KEY_B: &[u8] = b"__BOLT_STRUCTURE_FIELDS_KEY__";
pub(crate) const SIG_KEY: &str = "signature";

/// [Marker] constants module.
///
/// [Marker]: https://boltprotocol.org/v1/#markers
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

/// [Signature] constants module.
///
/// [Signature]: https://boltprotocol.org/v1/#signature
pub mod signature {
    pub const MSG_INIT: u8 = 0x01;
    pub const MSG_RUN: u8 = 0x10;
    pub const MSG_DISCARD_ALL: u8 = 0x2F;
    pub const MSG_PULL_ALL: u8 = 0x3F;
    pub const MSG_ACK_FAILURE: u8 = 0x0E;
    pub const MSG_RESET: u8 = 0x0F;
    pub const MSG_RECORD: u8 = 0x71;
    pub const MSG_SUCCESS: u8 = 0x70;
    pub const MSG_FAILURE: u8 = 0x7F;
    pub const MSG_IGNORED: u8 = 0x7E;
    pub const TYPE_NODE: u8 = 0x4E;
    pub const TYPE_RELATIONSHIP: u8 = 0x52;
    pub const TYPE_PATH: u8 = 0x50;
    pub const TYPE_UNBOUND_RELATIONSHIP: u8 = 0x72;
}
