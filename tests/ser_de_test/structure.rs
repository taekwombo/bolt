use super::*;
use serde_bolt::constants::marker::*;
use serde_bolt::value::structure::*;
use std::collections::HashMap;

#[test]
fn ack_failure() {
    const BYTES: &[u8] = &[TINY_STRUCT + AckFailure::LEN, AckFailure::SIG];
    ser_de::<AckFailure>(BYTES);
    de_ser(AckFailure);
    de_err::<AckFailure>(&[TINY_STRUCT, AckFailure::SIG + 1]);
}

#[test]
fn discard_all() {
    const BYTES: &[u8] = &[TINY_STRUCT + DiscardAll::LEN, DiscardAll::SIG];
    ser_de::<DiscardAll>(BYTES);
    de_ser(DiscardAll);
    de_err::<DiscardAll>(&[TINY_STRUCT, DiscardAll::SIG + 1]);
}

#[test]
fn failure() {
    const BYTES: &[u8] = &[TINY_STRUCT + Failure::LEN, Failure::SIG, TINY_MAP];
    ser_de::<Failure>(BYTES);
    de_ser(Failure {
        metadata: HashMap::new(),
    });
    de_err::<Failure>(&[TINY_STRUCT + 1, Failure::SIG + 1, TINY_MAP]);
}

#[test]
fn fields() {
    ser_de::<Empty>(&[TINY_LIST]);
    de_ser(Empty);
    de_err::<Empty>(&[TINY_LIST + 1, 0]);

    ser_de::<Single<u8>>(&[TINY_LIST + 1, 0]);
    de_ser(Single(100));
    de_err::<Single<u8>>(&[TINY_LIST]);
    de_err::<Single<u8>>(&[TINY_LIST + 2, 0, 0]);
}

#[test]
fn ignored() {
    const BYTES: &[u8] = &[TINY_STRUCT + Ignored::LEN, Ignored::SIG];
    ser_de::<Ignored>(BYTES);
    de_ser(Ignored);
    de_err::<Ignored>(&[TINY_STRUCT, Ignored::SIG + 1]);
}

#[test]
fn init() {
    // https://boltprotocol.org/v1/#message-init
    const BYTES: &[u8] = &[
        0xB2, 0x01, 0x8C, 0x4D, 0x79, 0x43, 0x6C, 0x69, 0x65, 0x6E, 0x74, 0x2F, 0x31, 0x2E, 0x30,
        0xA3, 0x86, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x65, 0x85, 0x62, 0x61, 0x73, 0x69, 0x63, 0x89,
        0x70, 0x72, 0x69, 0x6E, 0x63, 0x69, 0x70, 0x61, 0x6C, 0x85, 0x6E, 0x65, 0x6F, 0x34, 0x6A,
        0x8B, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6E, 0x74, 0x69, 0x61, 0x6C, 0x73, 0x86, 0x73, 0x65,
        0x63, 0x72, 0x65, 0x74,
    ];

    let s = String::from("test");

    ser_de::<Init>(BYTES);
    de_ser(Init::new(s.clone(), s.clone(), s.clone()));
    de_err::<Init>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn node() {
    const BYTES: &[u8] = &[TINY_STRUCT + Node::LEN, Node::SIG, 0, TINY_LIST, TINY_MAP];
    ser_de::<Node>(BYTES);
    de_ser(Node {
        identity: 0,
        labels: Vec::new(),
        properties: HashMap::new(),
    });
    de_err::<Node>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn path() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + Path::LEN,
        Path::SIG,
        TINY_LIST,
        TINY_LIST,
        TINY_LIST,
    ];

    ser_de::<Path>(BYTES);
    de_ser(Path {
        nodes: Vec::new(),
        relationships: Vec::new(),
        sequence: Vec::new(),
    });
    de_err::<Path>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn pull_all() {
    const BYTES: &[u8] = &[TINY_STRUCT + PullAll::LEN, PullAll::SIG];

    ser_de::<PullAll>(BYTES);
    de_ser(PullAll);
    de_err::<PullAll>(&[TINY_STRUCT, PullAll::SIG + 1]);
}

#[test]
fn record() {
    const BYTES: &[u8] = &[TINY_STRUCT + Record::LEN, Record::SIG, TINY_LIST + 1, 1];

    ser_de::<Record>(BYTES);
    de_ser(Record { fields: Vec::new() });
    de_err::<Record>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn relationship() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + Relationship::LEN,
        Relationship::SIG,
        0,
        0,
        0,
        TINY_STRING,
        TINY_MAP,
    ];

    ser_de::<Relationship>(BYTES);
    de_ser(Relationship {
        identity: 0,
        start_node_identity: 0,
        end_node_identity: 0,
        r#type: String::new(),
        properties: HashMap::new(),
    });
    de_err::<Relationship>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn reset() {
    const BYTES: &[u8] = &[TINY_STRUCT + Reset::LEN, Reset::SIG];

    ser_de::<Reset>(BYTES);
    de_ser(Reset);
    de_err::<Reset>(&[TINY_STRUCT, Reset::SIG + 1]);
}

#[test]
fn run() {
    const BYTES: &[u8] = &[TINY_STRUCT + Run::LEN, Run::SIG, TINY_STRING, TINY_MAP];

    ser_de::<Run>(BYTES);
    de_ser(Run {
        statement: String::new(),
        parameters: HashMap::new(),
    });
    de_err::<Run>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn success() {
    const BYTES: &[u8] = &[TINY_STRUCT + Success::LEN, Success::SIG, TINY_MAP];

    ser_de::<Success>(BYTES);
    de_ser(Success {
        metadata: HashMap::new(),
    });
    de_err::<Success>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn unbound_relationship() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + UnboundRelationship::LEN,
        UnboundRelationship::SIG,
        0,
        TINY_STRING,
        TINY_MAP,
    ];

    ser_de::<UnboundRelationship>(BYTES);
    de_ser(UnboundRelationship {
        identity: 0,
        r#type: String::new(),
        properties: HashMap::new(),
    });
    de_err::<UnboundRelationship>(&BYTES[0..(BYTES.len() - 1)]);
}
