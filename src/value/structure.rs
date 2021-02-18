use std::fmt;
use super::Value;

#[derive(Debug)]
pub struct Structure(u8, Vec<Value>);

// impl ser::Serialize for Structure {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: ser::Serializer
//     {
//         let mut tuple_struct = serializer.serialize_tuple_struct(STRUCTURE_NAME, self.1.len() + 1)?;
//         tuple_struct.serialize_field(&self.0)?;
//         for elem in self.1.iter() {
//             tuple_struct.serialize_field(elem)?;
//         }
//         tuple_struct.end()
//     }
// }

impl Structure {
    fn new (signature: u8) -> Self {
        Self (signature, Vec::new())
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tuple = f.debug_tuple(&format!("Structure {}", self.0));
        self.1.iter().for_each(|v| { tuple.field(v); });
        tuple.finish()
    }
}
