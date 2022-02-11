use serde::{Serialize, Serializer};
use crate::float32::f01;

impl Serialize for f01 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_f32(self.wrapped)
    }
}