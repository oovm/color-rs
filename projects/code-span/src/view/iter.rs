use crate::view::{CharacterInfo, CodeSpan, TextView};
use std::{mem::take, slice::Iter};

#[derive(Debug)]
pub struct TextViewIter<'i, T> {
    text: Iter<'i, CharacterInfo<T>>,
    current: Option<T>,
    buffer: String,
    run_out: bool,
}

impl<'i, T> IntoIterator for &'i TextView<T>
where
    T: Clone + PartialEq,
{
    type Item = CodeSpan<T>;
    type IntoIter = TextViewIter<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        TextViewIter { run_out: false, current: None, text: self.characters.iter(), buffer: "".to_string() }
    }
}

impl<'i, T> Iterator for TextViewIter<'i, T>
where
    T: Clone + PartialEq,
{
    type Item = CodeSpan<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.run_out {
            return None;
        }
        while let Some(this) = self.text.next() {
            if this.info == self.current {
                self.buffer.push(this.char);
                continue;
            }
            else {
                let out = self.pop_span();
                self.buffer.push(this.char);
                self.current = this.info.clone();
                if out.text.is_empty() {
                    continue;
                }
                else {
                    return Some(out);
                }
            }
        }
        self.run_out = true;
        Some(self.pop_span())
    }
}

impl<'i, T> TextViewIter<'i, T>
where
    T: Clone,
{
    #[inline]
    fn pop_span(&mut self) -> CodeSpan<T> {
        CodeSpan { text: take(&mut self.buffer), info: self.current.clone() }
    }
}
