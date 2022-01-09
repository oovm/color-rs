use super::*;
use crate::ColorSpan;
use std::{iter::Peekable, mem::take, slice::Iter};

#[derive(Debug)]
pub struct TextColorIter<'i> {
    colors: &'i IndexSet<String>,
    text: Peekable<Iter<'i, [u8; 4]>>,
    current_color_id: u8,
    buffer: String,
}

impl<'i> IntoIterator for &'i TextColorView {
    type Item = ColorSpan;
    type IntoIter = TextColorIter<'i>;

    fn into_iter(self) -> Self::IntoIter {
        TextColorIter {
            colors: &self.color_map,
            text: self.characters.iter().peekable(),
            current_color_id: 0,
            buffer: "".to_string(),
        }
    }
}

impl Iterator for TextColorIter<'_> {
    type Item = ColorSpan;

    fn next(&mut self) -> Option<Self::Item> {
        // match self.text.peek() {
        //     Some([l1, l2, l3, color]) => {},
        //     None => return Some(ColorSpan { color: "".to_string(), text: "".to_string() }),
        // }
        todo!()
    }
}
