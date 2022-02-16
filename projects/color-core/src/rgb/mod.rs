mod rgb;
mod rgba;
mod rgba32;

use std::{
    fmt::{Debug, Display, Formatter, LowerHex, UpperHex, Write},
    hash::{Hash, Hasher},
};

pub type RGB8 = RGBAColor<u8, ()>;
pub type RGBA8 = RGBAColor<u8, u8>;
pub type RGB32 = RGBAColor<f32, ()>;
pub type RGBA32 = RGBAColor<f32, f32>;

pub struct RGBAColor<T, A = T> {
    /// The red channel of color in `[0.0, 1.0]`.
    pub r: T,
    /// The green channel of color in `[0.0, 1.0]`.
    pub g: T,
    /// The blue channel of color in `[0.0, 1.0]`.
    pub b: T,
    /// The alpha channel of color in `[0.0, 1.0]`.
    pub a: A,
}
