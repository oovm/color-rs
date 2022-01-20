use crate::ColoredText;
use itertools::Itertools;
use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

impl Serialize for ColoredText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_seq(Some(self.characters.len()))?;
        for character in &self.characters {
            ser.serialize_element(&u32::from_le_bytes(*character))?
        }
        ser.end()
    }
}
