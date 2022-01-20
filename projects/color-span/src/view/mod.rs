use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    marker::PhantomData,
    ops::Range,
};

use crate::{
    ColorSpan, ColorSpanError,
    ColorSpanError::{OutOfRange, TooMuchColors},
};
use indexmap::IndexSet;

mod convert;
mod der;
mod iter;
mod ser;

pub trait Palette {
    type K;
    type V;

    /// Get or insert
    ///
    /// # Arguments
    ///
    /// * `key`:
    ///
    /// returns: Result<u8, ColorSpanError>
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ColoredText;
    /// ```
    fn get_index(&mut self, key: &Self::K) -> Result<u8, ColorSpanError>;
}

pub struct ClassPalette {
    classes: IndexSet<String>,
}

impl Palette for ClassPalette {
    type K = String;
    type V = ColorSpan;

    fn get_index(&mut self, key: &Self::K) -> Result<u8, ColorSpanError> {
        let index = match self.classes.get_full(key) {
            Some(s) => s.0,
            None => self.classes.insert_full(key.to_string()).0,
        };
        if index <= 255 { Ok(index as u8) } else { Err(ColorSpanError::TooMuchColors) }
    }
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
        let white = text.chars().map(|c| CharacterColor::from(c).into()).collect();
        Self { characters: white }
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
    pub fn dye(&mut self, start: usize, end: usize, color: u8) -> Result<(), ColorSpanError> {
        match self.characters.get_mut(Range { start, end }) {
            None => Err(ColorSpanError::OutOfRange { current: self.characters.len(), input: end })?,
            Some(s) => s.iter_mut().for_each(|s| s[3] = color),
        }
        Ok(())
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
