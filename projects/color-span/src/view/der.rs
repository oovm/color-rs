use crate::TextColorView;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt::Formatter;

struct TextColorMap {}

impl<'de> Deserialize<'de> for TextColorView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(TextColorMap {})
    }
}

impl<'de> Visitor<'de> for TextColorMap {
    type Value = TextColorView;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expect `{color_map: [String], characters: [u32]}`")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut out = TextColorView { colors: Default::default(), characters: vec![] };
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "colors" => out.colors = map.next_value()?,
                "characters" => {
                    let value: Vec<u32> = map.next_value()?;
                    out.characters = value.into_iter().map(|v| v.to_le_bytes()).collect()
                },
                // drop
                _ => {},
            }
        }
        Ok(out)
    }
}
