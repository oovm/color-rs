use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use std::rc::Rc;
use indexmap::IndexSet;

pub struct ColorSpan {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

/// Supports 255 color
#[derive(Debug)]
pub struct TextColorView {
    // intern string
    colors: IndexSet<String>,
    // same as Vec<char> with color bits
    characters: Vec<u32>,
}

pub struct CharacterColor {
    char: char,
    color: u8,
}

impl Debug for CharacterColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Colored").field(&self.char).field(&self.color).finish()
    }
}


impl TextColorView {
    ///
    ///
    /// # Arguments
    ///
    /// * `text`:
    ///
    /// returns: TextColorView
    ///
    /// # Examples
    ///
    /// ```
    /// // 256
    /// ```
    pub fn new(text: &str) -> TextColorView {
        let mut intern = IndexSet::default();
        intern.insert(String::new());
        let colored = text.chars().map(|c| CharacterColor::from(c).into()).collect();
        Self {
            colors: intern,
            characters: colored,
        }
    }
    pub fn insert(&mut self, start: usize, end: usize, color: String) {
        match  self.characters.get_mut(Range { start, end }) {
            None => {}
            Some(s) => {
                println!("{}", s)
            }
        }

    }
}

#[test]
pub fn text() {
    let text = TextColorView::new("public static class G {}");
    println!("{:#?}", text)
    // CharacterColor::from(0x10FFFF);
}

impl From<char> for CharacterColor {
    fn from(c: char) -> Self {
        Self {
            char: c,
            color: 0,
        }
    }
}

impl From<u32> for CharacterColor {
    fn from(c: u32) -> Self {
        let [l1, l2, l3, color] = c.to_le_bytes();
        let char_part = u32::from_le_bytes([l1, l2, l3, 0]).min(0x10FFFF);
        Self {
            char: unsafe {
                char::from_u32_unchecked(char_part)
            },
            color,
        }
    }
}

impl Into<u32> for CharacterColor {
    fn into(self) -> u32 {
        let [l1, l2, l3, _] = u32::from(self.char).to_le_bytes();
        u32::from_le_bytes([l1, l2, l3, self.color])
    }
}