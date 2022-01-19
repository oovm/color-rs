use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    marker::PhantomData,
    ops::Range,
};

use crate::ColorSpanError;
use indexmap::IndexSet;

mod convert;
mod der;
mod iter;
mod ser;

pub trait Palette {
    type K;
    type V;

    fn get_index(&self, key: Self::K) -> Result<u8, ColorSpanError>;
}

pub struct ClassPalette {
    classes: IndexSet<String>,
}

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
#[derive(Clone, Eq, PartialEq)]
pub struct ColoredText {
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

impl ColoredText {
    /// # Arguments
    ///
    /// * `text`:
    ///
    /// returns: TextColorView
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ColoredText;
    /// ```
    pub fn new(text: &str) -> ColoredText {
        let mut intern = IndexSet::default();
        intern.insert(String::new());
        let colored = text.chars().map(|c| CharacterColor::from(c).into()).collect();
        Self { colors: intern, characters: colored }
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
    /// use color_span::ColoredText;
    /// ```
    pub fn dye(&mut self, start: usize, end: usize, color: &str) -> Result<u8, ColorSpanError> {
        let index = match self.colors.get_index_of(color) {
            Some(s) => s,
            None => self.colors.insert_full(color.to_string()).0,
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
    /// use color_span::ColoredText;
    /// ```
    pub fn color_map(&self) -> BTreeMap<u8, &str> {
        self.colors.iter().enumerate().map(|(k, v)| (k as u8, v.as_str())).collect()
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
    /// use color_span::ColoredText;
    /// ```
    pub fn text(&self) -> String {
        self.characters.iter().map(|s| CharacterColor::from(s).char).collect()
    }
}
