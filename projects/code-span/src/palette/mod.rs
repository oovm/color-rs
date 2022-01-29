use crate::TextView;
use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};
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
pub struct HighlightStore<T> {
    share: BTreeSet<Rc<T>>,
    files: BTreeMap<String, TextView<Rc<T>>>,
}

impl<T> HighlightStore<T> {
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
    pub fn insert(&mut self, file: impl Into<String>, text: &str) -> Option<TextView<Rc<T>>> {
        self.files.insert(file.into(), TextView::new(text, None))
    }
    pub fn delete(&mut self) {
        self.files.remove()
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
            },
            None => {
                self.files.mark(start, end, None);
            },
        }
        None
    }
}
