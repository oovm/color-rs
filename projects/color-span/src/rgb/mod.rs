mod rgb;
mod rgba;
mod rgba32;

use std::{
    fmt::{Debug, Display, Formatter, LowerHex, UpperHex, Write},
    hash::{Hash, Hasher},
};

/// A color in the RGB color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RGB {
    /// The red channel of color in `[0, 255]`.
    pub r: u8,
    /// The green channel of color in `[0, 255]`.
    pub g: u8,
    /// The blue channel of color in `[0, 255]`.
    pub b: u8,
}

/// A color in the RGBA color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RGBA {
    /// The red channel of color in `[0, 255]`.
    pub r: u8,
    /// The green channel of color in `[0, 255]`.
    pub g: u8,
    /// The blue channel of color in `[0, 255]`.
    pub b: u8,
    /// The alpha channel of color in `[0, 255]`.
    pub a: u8,
}

/// A color in the RGBA color_parser space with 32-bit precision.
///
/// lossless format of rgb colors
#[derive(Debug, Clone, Copy)]
pub struct RGBA32 {
    /// The red channel of color in `[0.0, 1.0]`.
    pub r: f32,
    /// The green channel of color in `[0.0, 1.0]`.
    pub g: f32,
    /// The blue channel of color in `[0.0, 1.0]`.
    pub b: f32,
    /// The alpha channel of color in `[0.0, 1.0]`.
    pub a: f32,
}
