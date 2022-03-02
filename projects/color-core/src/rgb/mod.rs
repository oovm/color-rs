mod convert;
mod display;
mod rgb;
mod rgba;
mod rgba32;

use std::{
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
    hash::{Hash, Hasher},
};

/// 24-bit 3 channel color in the [RGB Color Space](https://en.wikipedia.org/wiki/RGB_color_model).
pub type RGB8 = RGBColor<u8, ()>;
/// 32-bit 4 channel color in the [RGB Color Space](https://en.wikipedia.org/wiki/RGB_color_model).
pub type RGBA8 = RGBColor<u8, u8>;
/// 128-bit 4 channel color in the [RGB Color Space](https://en.wikipedia.org/wiki/RGB_color_model).
pub type RGBA32 = RGBColor<f32, f32>;

/// A color in the [RGB Color Space](https://en.wikipedia.org/wiki/RGB_color_model).
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl<T, A> RGBColor<T, A> {
    /// Build a new color with the given red channel.
    pub fn with_red(self, r: T) -> Self {
        Self { r, ..self }
    }
    /// Build a new color with the given green channel.
    pub fn with_green(self, g: T) -> Self {
        Self { g, ..self }
    }
    /// Build a new color with the given blue channel.
    pub fn with_blue(self, b: T) -> Self {
        Self { b, ..self }
    }
    /// Build a new color with the given alpha channel.
    pub fn with_alpha(self, a: A) -> Self {
        Self { a, ..self }
    }
    /// Map operator to r, g, b channels, without alpha channel.
    pub fn map<F, G>(&self, color: F, alpha: G) -> Self
    where
        F: Fn(T) -> T,
        G: Fn(A) -> A,
        T: Clone,
        A: Clone,
    {
        Self { r: color(self.r.clone()), g: color(self.g.clone()), b: color(self.b.clone()), a: alpha(self.a.clone()) }
    }
    /// Map operator to r, g, b channels, without alpha channel.
    pub fn map_color<F>(&self, f: F) -> Self
    where
        F: Fn(T) -> T,
        T: Clone,
        A: Clone,
    {
        Self { r: f(self.r.clone()), g: f(self.g.clone()), b: f(self.b.clone()), a: self.a.clone() }
    }
    /// Map operator to alpha channel only.
    pub fn map_alpha<F>(&self, f: F) -> Self
    where
        F: Fn(A) -> A,
        T: Clone,
        A: Clone,
    {
        Self { r: self.r.clone(), g: self.g.clone(), b: self.b.clone(), a: f(self.a.clone()) }
    }
}
