pub(crate) const STRUCTURE_NAME: &str = "__BOLT_STRUCTURE_SERDE_NAME__";

pub(crate) mod marker {
    pub(crate) const TINY_STRING: u8 = 0x80;
    pub(crate) const TINY_STRING_MAX: u8 = 0x8F;
    pub(crate) const TINY_LIST: u8 = 0x90;
    pub(crate) const TINY_LIST_MAX: u8 = 0x9F;
    pub(crate) const TINY_MAP: u8 = 0xA0;
    pub(crate) const TINY_MAP_MAX: u8 = 0xAF;
    pub(crate) const TINY_STRUCT: u8 = 0xB0;
    pub(crate) const TINY_STRUCT_MAX: u8 = 0xBF;
    pub(crate) const NULL: u8 = 0xC0;
    pub(crate) const FLOAT_64: u8 = 0xC1;
    pub(crate) const TRUE: u8 = 0xC2;
    pub(crate) const FALSE: u8 = 0xC3;
    pub(crate) const INT_8: u8 = 0xC8;
    pub(crate) const INT_16: u8 = 0xC9;
    pub(crate) const INT_32: u8 = 0xCA;
    pub(crate) const INT_64: u8 = 0xCB;
    pub(crate) const BYTES_8: u8 = 0xCC;
    pub(crate) const BYTES_16: u8 = 0xCD;
    pub(crate) const BYTES_32: u8 = 0xCE;
    pub(crate) const STRING_8: u8 = 0xD0;
    pub(crate) const STRING_16: u8 = 0xD1;
    pub(crate) const STRING_32: u8 = 0xD2;
    pub(crate) const LIST_8: u8 = 0xD4;
    pub(crate) const LIST_16: u8 = 0xD5;
    pub(crate) const LIST_32: u8 = 0xD6;
    pub(crate) const LIST_STREAM: u8 = 0xD7;
    pub(crate) const MAP_8: u8 = 0xD8;
    pub(crate) const MAP_16: u8 = 0xD9;
    pub(crate) const MAP_32: u8 = 0xDA;
    pub(crate) const MAP_STREAM: u8 = 0xDB;
    pub(crate) const STRUCT_8: u8 = 0xDC;
    pub(crate) const STRUCT_16: u8 = 0xDD;
    pub(crate) const END_OF_STREAM: u8 = 0xDF;
}

pub(crate) mod signature {
    pub(crate) const MSG_INIT: u8 = 0x01;
    pub(crate) const MSG_RUN: u8 = 0x10;
    pub(crate) const MSG_DISCARD_ALL: u8 = 0x2F;
    pub(crate) const MSG_PULL_ALL: u8 = 0x3F;
    pub(crate) const MSG_ACK_FAILURE: u8 = 0x0E;
    pub(crate) const MSG_RESET: u8 = 0x0F;
    pub(crate) const MSG_RECORD: u8 = 0x71;
    pub(crate) const MSG_SUCCESS: u8 = 0x70;
    pub(crate) const MSG_FAILURE: u8 = 0x7F;
    pub(crate) const MSG_IGNORED: u8 = 0x7E;
    pub(crate) const TYPE_NODE: u8 = 0x4E;
    pub(crate) const TYPE_RELATIONSHIP: u8 = 0x52;
    pub(crate) const TYPE_PATH: u8 = 0x50;
    pub(crate) const TYPE_UNBOUND_RELATIONSHIP: u8 = 0x72;
}

