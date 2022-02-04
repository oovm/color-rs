use serde::{Serialize, Serializer};

use crate::Character;

impl Serialize for Character {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.repr)
    }
}
