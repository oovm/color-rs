use color_char::Character;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

use crate::ColorSpanError;
use serde::{Deserialize, Serialize};

mod convert;
mod der;
pub mod iter;
mod ser;

/// Write color palette into html
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
/// use color_span::ColorClass;
/// ```
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TextView {
    characters: Vec<Character>,
}

/// Write color palette into html
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
/// use color_span::ColorClass;
/// ```
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Colored<T> {
    /// Raw value
    pub value: T,
    /// Color id
    pub color: u32,
}

impl TextView {
    /// # Arguments
    ///
    /// * `text`:
    ///
    /// returns: TextColorView
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::TextView;
    /// ```
    pub fn new(text: &str) -> TextView {
        let white = text.chars().map(Character::from).collect();
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
    /// use color_span::TextView;
    /// ```
    pub fn dye(&mut self, start: usize, end: usize, color: u32) -> Result<(), ColorSpanError> {
        match self.characters.get_mut(Range { start, end }) {
            None => Err(ColorSpanError::OutOfRange { current: self.characters.len(), input: end })?,
            Some(s) => s.iter_mut().for_each(|c| c.set_color(color)),
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
    /// use color_span::TextView;
    /// ```
    pub fn text(&self) -> String {
        self.characters.iter().map(|s| s.get_char()).collect()
    }
}
