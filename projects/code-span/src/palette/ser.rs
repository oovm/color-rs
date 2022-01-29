use crate::{HighlightStore, TextView};
use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

impl<T> Serialize for HighlightStore<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("CodePalette", 2)?;
        ser.serialize_field("", &self.share)?;
        ser.serialize_field("", &self.files)?;
        ser.end()
    }
}
