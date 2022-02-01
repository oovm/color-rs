use std::fmt::Formatter;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};

use crate::Character;

struct CharacterVisitor;

impl<'de> Deserialize<'de> for Character {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CharacterVisitor)
    }
}

impl<'de> Visitor<'de> for CharacterVisitor {
    type Value = Character;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting a character")
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Character::from(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if value.len() != 1 {
            return Err(E::invalid_length(value.len(), &self));
        }
        unsafe { Ok(Character::from(value.chars().next().unwrap_unchecked())) }
    }
}
