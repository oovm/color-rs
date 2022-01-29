use crate::{ColorSpanError, TextView};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
mod iter;

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
/// use color_span::TextView;
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClassPalette {
    classes: IndexSet<String>,
    text: TextView,
}

impl ClassPalette {
    /// # Arguments
    ///
    /// * `text`:
    ///
    /// returns: ClassPalette
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ClassPalette;
    /// let _ = ClassPalette::new("public static class Singleton {}");
    /// ```
    pub fn new(text: &str) -> ClassPalette {
        let mut classes = IndexSet::default();
        classes.insert("".to_string());
        Self { classes, text: TextView::new(text) }
    }

    /// # Arguments
    ///
    /// * `start`:
    /// * `end`:
    /// * `color`:
    ///
    /// returns: Result<(), ColorSpanError>
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ClassPalette;
    /// ```
    pub fn dye(&mut self, start: usize, end: usize, color: &str) -> Result<u8, ColorSpanError> {
        let index = match self.classes.get_full(color) {
            Some(s) => s.0,
            None => self.classes.insert_full(color.to_string()).0,
        };
        let index = if index <= 255 { index as u8 } else { Err(ColorSpanError::TooMuchColors)? };
        self.text.dye(start, end, index)?;
        Ok(index)
    }
}
