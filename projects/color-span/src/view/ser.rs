use crate::TextView;
use serde::{ser::SerializeSeq, Serialize, Serializer};

impl Serialize for TextView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_seq(Some(self.characters.len()))?;
        for character in &self.characters {
            ser.serialize_element(character)?
        }
        ser.end()
    }
}
