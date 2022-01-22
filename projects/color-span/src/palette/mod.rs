use crate::{ColorSpanError, TextView};
use indexmap::IndexSet;
use std::{
    any::{type_name, type_name_of_val},
    borrow::Borrow,
    collections::HashMap,
    fmt::{Display, Formatter, Write},
};

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
#[derive(Debug)]
pub struct ColorClass {
    /// Color name of the palette
    pub color: String,
    /// Text of the palette
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
    /// use color_span::TextView;
    /// ```
    fn get_text(&self) -> &TextView;
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
    fn mut_text(&mut self) -> &mut TextView;
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
    /// use color_span::TextView;
    /// ```
    fn dye(&mut self, start: usize, end: usize, color: &Self::K) -> Result<(), ColorSpanError> {
        let id = self.get_index(color)?;
        self.mut_text().dye(start, end, id)?;
        Ok(())
    }
}

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
#[derive(Debug)]
pub struct ClassPalette {
    classes: IndexSet<String>,
    text: TextView,
}

#[test]
fn test() {
    let mut class = ClassPalette { classes: Default::default(), text: TextView::new("public") };
    class.dye(0, 5, "keyword").unwrap();

    println!("{:#?}", class)
}

impl Palette for ClassPalette {
    type K = str;
    type V = ColorClass;

    fn get_text(&self) -> &TextView {
        &self.text
    }

    fn mut_text(&mut self) -> &mut TextView {
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
