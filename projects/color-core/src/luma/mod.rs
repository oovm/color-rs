use std::fmt::{Debug, Formatter};

mod luma32;

pub type LUMA32 = LUMAColor<f32, f32>;

/// A color_parser in the luma color_parser space.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct LUMAColor<T, A = T> {
    ///
    pub l: T,
    ///
    pub u: T,
    ///
    pub m: T,
    ///
    pub a: A,
}
