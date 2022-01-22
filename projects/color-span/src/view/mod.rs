use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    marker::PhantomData,
    ops::Range,
};

use crate::{
    ColorClass, ColorSpanError,
    ColorSpanError::{OutOfRange, TooMuchColors},
};
use indexmap::IndexSet;

mod convert;
mod der;
mod iter;
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
#[derive(Clone, Eq, PartialEq)]
pub struct TextView {
    characters: Vec<[u8; 4]>,
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
pub struct Colored<T> {
    pub value: T,
    pub color: u8,
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
        let white = text.chars().map(char2slice).collect();
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
    /// use color_span::TextView;
    /// ```
    pub fn text(&self) -> String {
        self.characters.iter().map(|s| slice2color(*s).value).collect()
    }
}

#[inline]
fn slice2color(c: [u8; 4]) -> Colored<char> {
    let [l1, l2, l3, color] = c;
    let char_part = u32::from_le_bytes([l1, l2, l3, 0]).min(0x10FFFF);
    Colored { value: unsafe { char::from_u32_unchecked(char_part) }, color }
}

#[inline]
fn color2slice(c: Colored<char>) -> [u8; 4] {
    let [l1, l2, l3, _] = u32::from(c.value).to_le_bytes();
    [l1, l2, l3, c.color]
}

#[inline]
fn char2slice(c: char) -> [u8; 4] {
    let [l1, l2, l3, _] = u32::from(c).to_le_bytes();
    [l1, l2, l3, 0]
}
