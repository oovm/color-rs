use crate::{Colored, TextView};
use color_char::Character;
use std::{iter::Peekable, mem::take, slice::Iter};

#[derive(Debug)]
pub struct TextColorIter<'i> {
    run_out: bool,
    current_color_id: u32,
    text: Peekable<Iter<'i, Character>>,
    buffer: String,
}

impl<'i> IntoIterator for &'i TextView {
    type Item = Colored<String>;
    type IntoIter = TextColorIter<'i>;

    fn into_iter(self) -> Self::IntoIter {
        TextColorIter { run_out: false, current_color_id: 0, text: self.characters.iter().peekable(), buffer: "".to_string() }
    }
}

impl Iterator for TextColorIter<'_> {
    type Item = Colored<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.run_out {
            return None;
        }
        while let Some(this) = self.text.next() {
            let char = this;
            if char.get_color() == self.current_color_id {
                self.buffer.push(char.get_char());
                continue;
            }
            else {
                let out = self.pop_span();
                self.buffer.push(char.get_char());
                self.current_color_id = char.get_color();
                if out.value.is_empty() {
                    continue;
                }
                else {
                    return Some(out);
                }
            }
        }
        self.run_out = true;
        Some(self.pop_span())
    }
}

impl TextColorIter<'_> {
    fn pop_span(&mut self) -> Colored<String> {
        Colored { color: self.current_color_id, value: take(&mut self.buffer) }
    }
}
