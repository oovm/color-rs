use crate::ColoredText;
use itertools::Itertools;
use serde::{ser::SerializeStruct, Serialize, Serializer};
impl Serialize for ColoredText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("TextColorView", 2)?;
        ser.serialize_field("colors", &self.colors)?;
        ser.serialize_field("characters", &self.characters.iter().map(|s| u32::from_le_bytes(*s)).collect_vec())?;
        ser.end()
    }
}
