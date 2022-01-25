use crate::{view::iter::TextColorIter, ClassPalette, Colored};
use indexmap::IndexSet;

///
#[derive(Debug)]
pub struct ClassPaletteIter<'i> {
    map: &'i IndexSet<String>,
    text: TextColorIter<'i>,
}

impl<'i> IntoIterator for &'i ClassPalette {
    type Item = (String, String);
    type IntoIter = ClassPaletteIter<'i>;

    fn into_iter(self) -> Self::IntoIter {
        ClassPaletteIter { map: &self.classes, text: self.text.into_iter() }
    }
}

impl Iterator for ClassPaletteIter<'_> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let Colored { value, color } = self.text.next()?;
        let class = match self.map.get_index(color as usize) {
            Some(s) => s.to_string(),
            None => String::new(),
        };
        Some((class, value))
    }
}
