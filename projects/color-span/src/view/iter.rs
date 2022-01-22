use crate::{view::slice2color, Colored, TextView};
use indexmap::IndexSet;
use std::{iter::Peekable, mem::take, slice::Iter};

#[derive(Debug)]
pub struct TextColorIter<'i> {
    run_out: bool,
    current_color_id: u8,
    text: Peekable<Iter<'i, [u8; 4]>>,
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
            let char = slice2color(*this);
            if char.color == self.current_color_id {
                self.buffer.push(char.value);
                continue;
            }
            else {
                let out = Colored { color: self.current_color_id, value: take(&mut self.buffer) };
                self.buffer.push(char.value);
                self.current_color_id = char.color;
                if out.text.is_empty() {
                    continue;
                }
                else {
                    return Some(out);
                }
            }
        }
        self.run_out = true;
        Some(Colored { color: self.current_color_id, value: take(&mut self.buffer) })
    }
}
