use crate::{ColorSpanError, ColoredText};
use indexmap::IndexSet;
use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::{Display, Formatter, Write},
};

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
#[derive(Debug)]
pub struct ColorSpan {
    /// Color name of the span
    pub color: String,
    /// Text of the span
    pub text: String,
}

pub trait Palette {
    type K: ?Sized;
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
    fn get_text(&self) -> &ColoredText;
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
    fn mut_text(&mut self) -> &mut ColoredText;
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
    fn dye(&mut self, start: usize, end: usize, color: &Self::K) -> Result<(), ColorSpanError> {
        let id = self.get_index(color)?;
        self.mut_text().dye(start, end, id)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ClassPalette {
    classes: IndexSet<String>,
    text: ColoredText,
}

#[test]
fn test() {
    let mut class = ClassPalette { classes: Default::default(), text: ColoredText::new("public") };
    class.dye(0, 5, "keyword").unwrap();
    println!("{:#?}", class)
}

impl Palette for ClassPalette {
    type K = str;
    type V = ColorSpan;

    fn get_text(&self) -> &ColoredText {
        &self.text
    }

    fn mut_text(&mut self) -> &mut ColoredText {
        &mut self.text
    }

    fn get_index(&mut self, key: &Self::K) -> Result<u8, ColorSpanError> {
        let index = match self.classes.get_full(key) {
            Some(s) => s.0,
            None => self.classes.insert_full(key.to_string()).0,
        };
        if index <= 255 { Ok(index as u8) } else { Err(ColorSpanError::TooMuchColors) }
    }
}
