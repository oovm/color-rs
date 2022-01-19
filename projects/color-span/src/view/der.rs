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
        formatter.write_str("Expect `{color_map: [String], characters: [u32]}`")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        todo!()
    }
}
