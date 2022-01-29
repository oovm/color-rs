use std::ops::Range;

use arc_interner::ArcIntern;
use serde::{Deserialize, Serialize};

mod iter;

///
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TextView<T> {
    characters: Vec<CharacterInfo<T>>,
}

///
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CharacterInfo<T> {
    /// Raw character
    pub char: char,
    /// Information
    pub info: Option<ArcIntern<T>>,
}

///
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CodeSpan<T> {
    /// Raw character
    pub text: String,
    /// Information
    pub info: Option<ArcIntern<T>>,
}

impl<T> TextView<T> {
    /// # Arguments
    ///
    /// * `text`:
    /// * `default`:
    ///
    /// returns: TextView<T>
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::TextView;
    /// ```
    pub fn new(text: &str, default: Option<T>) -> TextView<T>
        where
            T: Clone,
    {
        Self { characters: text.chars().map(|c| CharacterInfo { char: c, info: default.clone() }).collect() }
    }
    /// # Arguments
    ///
    /// * `text`:
    /// * `default`:
    ///
    /// returns: TextView<T>
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::TextView;
    /// ```
    pub fn text(&self) -> String {
        self.characters.iter().map(|c| c.char).collect()
    }
    /// # Arguments
    ///
    /// * `start`:
    /// * `end`:
    /// * `info`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::TextView;
    /// ```
    pub fn mark(&mut self, start: usize, end: usize, info: Option<T>)
        where
            T: Clone,
    {
        debug_assert!(start <= end);
        let end = self.characters.len().min(end);
        let items = unsafe { self.characters.get_unchecked_mut(Range { start, end }) };
        for item in items {
            item.info = info.clone()
        }
    }
}
