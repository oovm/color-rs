use crate::TextColorView;
use itertools::Itertools;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
impl Serialize for TextColorView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("TextColorView", 2)?;
        ser.serialize_field("color_map", &self.color_map)?;
        ser.serialize_field("characters", &self.characters.iter().map(|s| u32::from_le_bytes(*s)).collect_vec())?;
        ser.end()
    }
}
