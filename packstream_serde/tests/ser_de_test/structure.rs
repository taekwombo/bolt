use super::*;
use packstream_serde::constants::marker::*;
use packstream_serde::value::structure::*;
use std::collections::HashMap;

#[test]
fn node() {
    const BYTES: &[u8] = &[TINY_STRUCT + Node::LEN, Node::SIG, 0, TINY_LIST, TINY_MAP];

    ser_de::<Node>(BYTES);
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(Node {
        id: 0,
        labels: Vec::new(),
        properties: HashMap::new(),
    });
    de_ser(Structure::Node(Node {
        id: 0,
        labels: Vec::new(),
        properties: HashMap::new(),
    }));
    de_ser(Value::Structure(Structure::Node(Node {
        id: 0,
        labels: Vec::new(),
        properties: HashMap::new(),
    })));

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
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(Path {
        nodes: Vec::new(),
        relationships: Vec::new(),
        sequence: Vec::new(),
    });
    de_ser(Structure::Path(Path {
        nodes: Vec::new(),
        relationships: Vec::new(),
        sequence: Vec::new(),
    }));
    de_ser(Value::Structure(Structure::Path(Path {
        nodes: Vec::new(),
        relationships: Vec::new(),
        sequence: Vec::new(),
    })));

    de_err::<Path>(&BYTES[0..(BYTES.len() - 1)]);
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
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(Relationship {
        id: 0,
        start_node_id: 0,
        end_node_id: 0,
        r#type: String::new(),
        properties: HashMap::new(),
    });
    de_ser(Structure::Relationship(Relationship {
        id: 0,
        start_node_id: 0,
        end_node_id: 0,
        r#type: String::new(),
        properties: HashMap::new(),
    }));
    de_ser(Value::Structure(Structure::Relationship(Relationship {
        id: 0,
        start_node_id: 0,
        end_node_id: 0,
        r#type: String::new(),
        properties: HashMap::new(),
    })));

    de_err::<Relationship>(&BYTES[0..(BYTES.len() - 1)]);
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
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(UnboundRelationship {
        id: 0,
        r#type: String::new(),
        properties: HashMap::new(),
    });
    de_ser(Structure::UnboundRelationship(UnboundRelationship {
        id: 0,
        r#type: String::new(),
        properties: HashMap::new(),
    }));
    de_ser(Value::Structure(Structure::UnboundRelationship(UnboundRelationship {
        id: 0,
        r#type: String::new(),
        properties: HashMap::new(),
    })));

    de_err::<UnboundRelationship>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn date() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + Date::LEN,
        Date::SIG,
        0,
    ];

    ser_de::<Date>(BYTES);
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(Date {
        days: 0,
    });
    de_ser(Structure::Date(Date {
        days: 0,
    }));
    de_ser(Value::Structure(Structure::Date(Date {
        days: 0,
    })));

    de_err::<Date>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn time() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + Time::LEN,
        Time::SIG,
        0,
        0
    ];

    ser_de::<Time>(BYTES);
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(Time {
        nanoseconds: 0,
        tz_offset_seconds: 0,
    });
    de_ser(Structure::Time(Time {
        nanoseconds: 0,
        tz_offset_seconds: 0,
    }));
    de_ser(Value::Structure(Structure::Time(Time {
        nanoseconds: 0,
        tz_offset_seconds: 0,
    })));

    de_err::<Time>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn local_time() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + LocalTime::LEN,
        LocalTime::SIG,
        0
    ];

    ser_de::<LocalTime>(BYTES);
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(LocalTime {
        nanoseconds: 0,
    });
    de_ser(Structure::LocalTime(LocalTime {
        nanoseconds: 0,
    }));
    de_ser(Value::Structure(Structure::LocalTime(LocalTime {
        nanoseconds: 0,
    })));

    de_err::<LocalTime>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn date_time() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + DateTime::LEN,
        DateTime::SIG,
        0,
        0,
        0,
    ];

    ser_de::<DateTime>(BYTES);
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(DateTime {
        seconds: 0,
        nanoseconds: 0,
        tz_offset_seconds: 0,
    });
    de_ser(Structure::DateTime(DateTime {
        seconds: 0,
        nanoseconds: 0,
        tz_offset_seconds: 0,
    }));
    de_ser(Value::Structure(Structure::DateTime(DateTime {
        seconds: 0,
        nanoseconds: 0,
        tz_offset_seconds: 0,
    })));

    de_err::<DateTime>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn date_time_zone_id() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + DateTimeZoneId::LEN,
        DateTimeZoneId::SIG,
        0,
        0,
        TINY_STRING + 2,
        b'n',
        b'z',
    ];

    ser_de::<DateTimeZoneId>(BYTES);
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(DateTimeZoneId {
        seconds: 0,
        nanoseconds: 0,
        tz_id: String::from("nz"),
    });
    de_ser(Structure::DateTimeZoneId(DateTimeZoneId {
        seconds: 0,
        nanoseconds: 0,
        tz_id: String::from("nz"),
    }));
    de_ser(Value::Structure(Structure::DateTimeZoneId(DateTimeZoneId {
        seconds: 0,
        nanoseconds: 0,
        tz_id: String::from("nz"),
    })));

    de_err::<DateTimeZoneId>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn local_date_time() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + LocalDateTime::LEN,
        LocalDateTime::SIG,
        0,
        0,
    ];

    ser_de::<LocalDateTime>(BYTES);
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(LocalDateTime {
        seconds: 0,
        nanoseconds: 0,
    });
    de_ser(Structure::LocalDateTime(LocalDateTime {
        seconds: 0,
        nanoseconds: 0,
    }));
    de_ser(Value::Structure(Structure::LocalDateTime(LocalDateTime {
        seconds: 0,
        nanoseconds: 0,
    })));

    de_err::<LocalDateTime>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn duration() {
    const BYTES: &[u8] = &[
        TINY_STRUCT + Duration::LEN,
        Duration::SIG,
        0,
        0,
        0,
        0,
    ];

    ser_de::<Duration>(BYTES);
    ser_de::<Structure>(BYTES);
    ser_de::<Value>(BYTES);

    de_ser(Duration {
        months: 0,
        days: 0,
        seconds: 0,
        nanoseconds: 0,
    });
    de_ser(Structure::Duration(Duration {
        months: 0,
        days: 0,
        seconds: 0,
        nanoseconds: 0,
    }));
    de_ser(Value::Structure(Structure::Duration(Duration {
        months: 0,
        days: 0,
        seconds: 0,
        nanoseconds: 0,
    })));

    de_err::<Duration>(&BYTES[0..(BYTES.len() - 1)]);
}

#[test]
fn point2d() {
    let mut bytes: Vec<u8> = vec![
        TINY_STRUCT + Point2D::LEN,
        Point2D::SIG,
        0

    ];

    bytes.push(FLOAT_64);
    bytes.extend_from_slice(&0.0f64.to_bits().to_be_bytes());
    bytes.push(FLOAT_64);
    bytes.extend_from_slice(&0.0f64.to_bits().to_be_bytes());

    let bytes = &bytes;

    ser_de::<Point2D>(bytes);
    ser_de::<Structure>(bytes);
    ser_de::<Value>(bytes);

    de_ser(Point2D {
        srid: 0,
        x: 0.0,
        y: 0.0,
    });
    de_ser(Structure::Point2D(Point2D {
        srid: 0,
        x: 0.0,
        y: 0.0,
    }));
    de_ser(Value::Structure(Structure::Point2D(Point2D {
        srid: 0,
        x: 0.0,
        y: 0.0,
    })));

    de_err::<Point2D>(&bytes[0..(bytes.len() - 1)]);
}

#[test]
fn point3d() {
    let mut bytes: Vec<u8> = vec![
        TINY_STRUCT + Point3D::LEN,
        Point3D::SIG,
        0
    ];

    bytes.push(FLOAT_64);
    bytes.extend_from_slice(&0.0f64.to_bits().to_be_bytes());
    bytes.push(FLOAT_64);
    bytes.extend_from_slice(&0.0f64.to_bits().to_be_bytes());
    bytes.push(FLOAT_64);
    bytes.extend_from_slice(&0.0f64.to_bits().to_be_bytes());

    let bytes = &bytes;

    ser_de::<Point3D>(bytes);
    ser_de::<Structure>(bytes);
    ser_de::<Value>(bytes);

    de_ser(Point3D {
        srid: 0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
    de_ser(Structure::Point3D(Point3D {
        srid: 0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }));
    de_ser(Value::Structure(Structure::Point3D(Point3D {
        srid: 0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    })));

    de_err::<Point3D>(&bytes[0..(bytes.len() - 1)]);
}
