use serde::{Deserialize, Deserializer};
use crate::float32::f01;

impl<'de> Deserialize<'de> for f01 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = f32::deserialize(deserializer)?;
        Ok(f01::new(value))
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error> where D: Deserializer<'de> {
        let value = f32::deserialize(deserializer)?;
        *place = f01::new(value);
        Ok(())
    }
}