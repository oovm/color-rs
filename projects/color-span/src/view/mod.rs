use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    ops::Range,
};

use crate::ColorSpanError;
use indexmap::IndexSet;
mod der;
pub mod html;
mod iter;
mod ser;

/// Write color span into html
///
/// **Support 255 color at most**.
///
/// # Arguments
///
/// * `w`:
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// use color_span::ColorSpan;
/// ```
#[derive(Debug)]
pub struct TextColorView {
    // intern string
    color_map: IndexSet<String>,
    // same as Vec<char> with color bits
    characters: Vec<[u8; 4]>,
}

/// Write color span into html
///
/// # Arguments
///
/// * `w`:
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// use color_span::ColorSpan;
/// ```
pub struct CharacterColor {
    char: char,
    color: u8,
}

impl Debug for CharacterColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Colored").field(&self.char).field(&self.color).finish()
    }
}

impl From<&str> for TextColorView {
    fn from(s: &str) -> Self {
        TextColorView::new(s)
    }
}

impl From<String> for TextColorView {
    fn from(s: String) -> Self {
        TextColorView::new(&s)
    }
}

impl TextColorView {
    /// # Arguments
    ///
    /// * `text`:
    ///
    /// returns: TextColorView
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::TextColorView;
    /// ```
    pub fn new(text: &str) -> TextColorView {
        let mut intern = IndexSet::default();
        intern.insert(String::new());
        let colored = text.chars().map(|c| CharacterColor::from(c).into()).collect();
        Self { color_map: intern, characters: colored }
    }

    /// Color the text in the range of `start`..`end` to given color name
    ///
    /// # Arguments
    ///
    /// * `start`: start offset
    /// * `end`: end offset
    /// * `color`: color name
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::TextColorView;
    /// ```
    pub fn dye(&mut self, start: usize, end: usize, color: &str) -> Result<u8, ColorSpanError> {
        let index = match self.color_map.get_index_of(color) {
            Some(s) => s,
            None => self.color_map.insert_full(color.to_string()).0,
        };
        let index = match index <= 255 {
            true => index as u8,
            false => Err(ColorSpanError::TooMuchColors)?,
        };
        match self.characters.get_mut(Range { start, end }) {
            None => Err(ColorSpanError::OutOfRange { current: self.characters.len(), input: end })?,
            Some(s) => s.iter_mut().for_each(|s| s[3] = index),
        }
        Ok(index)
    }
    /// Color the text in the range of `start`..`end` to given color name
    ///
    /// # Arguments
    ///
    /// * `start`: start offset
    /// * `end`: end offset
    /// * `color`: color name
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::TextColorView;
    /// ```
    pub fn colors(&self) -> BTreeMap<u8, &str> {
        self.color_map.iter().enumerate().map(|(k, v)| (k as u8, v.as_str())).collect()
    }
}

impl From<char> for CharacterColor {
    fn from(c: char) -> Self {
        Self { char: c, color: 0 }
    }
}

impl From<[u8; 4]> for CharacterColor {
    fn from(c: [u8; 4]) -> Self {
        let [l1, l2, l3, color] = c;
        let char_part = u32::from_le_bytes([l1, l2, l3, 0]).min(0x10FFFF);
        Self { char: unsafe { char::from_u32_unchecked(char_part) }, color }
    }
}

impl Into<[u8; 4]> for CharacterColor {
    fn into(self) -> [u8; 4] {
        let [l1, l2, l3, _] = u32::from(self.char).to_le_bytes();
        [l1, l2, l3, self.color]
    }
}
