use crate::TextView;
use color_char::Character;
use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt::Formatter;

struct TextViewSequence {}

impl<'de> Deserialize<'de> for TextView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(TextViewSequence {})
    }
}

impl<'de> Visitor<'de> for TextViewSequence {
    type Value = TextView;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expect `[u32]`")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut out = TextView { characters: vec![] };
        while let Some(repr) = seq.next_element::<u32>()? {
            out.characters.push(Character::from(repr));
        }
        Ok(out)
    }
}
