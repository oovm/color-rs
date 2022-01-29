use std::rc::Rc;

use crate::TextView;

// mod iter;

mod der;
mod ser;

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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HighlightClass {
    share: TextView<String>,
}

impl HighlightClass {
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
    pub fn new(text: &str) -> HighlightClass {
        Self {
            share: TextView::new(text, None),
        }
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
    pub fn mark(&mut self, file: &str, start: usize, end: usize, info: Option<T>) -> Option<()>
        where
            T: Ord,
    {
        let file = self.files.get_mut(file)?;

        match info {
            Some(info) => {
                let index = match self.share.get(&info) {
                    Some(s) => s.clone(),
                    None => Rc::from(info),
                };
                self.files.mark(start, end, Some(index));
            }
            None => {
                self.files.mark(start, end, None);
            }
        }
        None
    }
}
