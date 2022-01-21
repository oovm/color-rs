use crate::ColoredText;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt::Formatter;

struct ColoredTextVisitor {}

impl<'de> Deserialize<'de> for ColoredText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ColoredTextVisitor {})
    }
}

impl<'de> Visitor<'de> for ColoredTextVisitor {
    type Value = ColoredText;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expect `[u32]`")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut out = ColoredText { characters: vec![] };
        while let Some(buffer) = seq.next_element::<u32>()? {
            out.characters.push(buffer.to_le_bytes())
        }
        Ok(out)
    }
}
