use serde_derive::Deserialize;
use std::fmt;
use super::Value;
use super::super::marker_bytes::STRUCTURE_NAME;
use serde::ser::{self, SerializeTupleStruct};

#[derive(Debug, Deserialize)]
pub struct Structure(Vec<Value>);

impl Structure {
    fn new () -> Self {
        Self (Vec::new())
    }

    fn push (&mut self, value: Value) -> &mut Self {
        self.0.push(value);
        self
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return f.write_str("Structure()");
        }
        let mut tuple = f.debug_tuple("Structure");
        self.0.iter().for_each(|v| { tuple.field(v); });
        tuple.finish()
    }
}

impl ser::Serialize for Structure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer
    {
        let len = if self.0.is_empty() {
            0
        } else {
            self.0.len() - 1
        };

        let mut tuple_struct = serializer.serialize_tuple_struct(STRUCTURE_NAME, len)?;
        for elem in self.0.iter() {
            tuple_struct.serialize_field(elem)?;
        }
        tuple_struct.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de::from_bytes;
    use crate::ser::to_bytes;
    use crate::marker_bytes::*;

    #[test]
    fn test_structure() {
        let b: &[u8] = &[TINY_STRUCT + 1, 10, 10];
        let s: Structure = from_bytes::<Structure>(&b).unwrap();
        assert_eq!(to_bytes(&s).unwrap(), b);
    }
}
