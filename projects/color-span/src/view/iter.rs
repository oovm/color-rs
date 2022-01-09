use super::*;
use crate::ColorSpan;
use std::{iter::Peekable, mem::take, slice::Iter};

#[derive(Debug)]
pub struct TextColorIter<'i> {
    run_out: bool,
    current_color_id: u8,
    colors: &'i IndexSet<String>,
    text: Peekable<Iter<'i, [u8; 4]>>,
    buffer: String,
}

impl<'i> IntoIterator for &'i TextColorView {
    type Item = ColorSpan;
    type IntoIter = TextColorIter<'i>;

    fn into_iter(self) -> Self::IntoIter {
        TextColorIter {
            run_out: false,
            current_color_id: 0,
            colors: &self.color_map,
            text: self.characters.iter().peekable(),
            buffer: "".to_string(),
        }
    }
}

impl Iterator for TextColorIter<'_> {
    type Item = ColorSpan;

    fn next(&mut self) -> Option<Self::Item> {
        if self.run_out {
            return None;
        }
        while let Some(this) = self.text.next() {
            let char = CharacterColor::from(*this);
            if char.color == self.current_color_id {
                self.buffer.push(char.char);
                continue;
            }
            else {
                let out = self.pop_span();
                self.buffer.push(char.char);
                self.current_color_id = char.color;
                return out;
            }
        }
        self.run_out = true;
        self.pop_span()
    }
}

impl TextColorIter<'_> {
    fn pop_span(&mut self) -> Option<ColorSpan> {
        let color = match self.colors.get_index(self.current_color_id as usize) {
            Some(color) => color.to_string(),
            None => String::new(),
        };
        Some(ColorSpan { color, text: take(&mut self.buffer) })
    }
}
