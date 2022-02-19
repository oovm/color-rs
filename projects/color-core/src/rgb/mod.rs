mod convert;
mod display;
mod rgb;
mod rgba;
mod rgba32;

use std::{
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
    hash::{Hash, Hasher},
};

/// 128-bit 4 channel color in the HSL color space.
pub type RGB8 = RGBColor<u8, ()>;
pub type RGBA8 = RGBColor<u8, u8>;
pub type RGBA32 = RGBColor<f32, f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RGBColor<T, A = T> {
    /// The red channel of a rgb space color
    pub r: T,
    /// The green channel of a rgb space color
    pub g: T,
    /// The blue channel of a rgb space color
    pub b: T,
    /// The alpha channel of a rgb space color
    pub a: A,
}
