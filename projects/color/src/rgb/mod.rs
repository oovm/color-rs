mod rgb;
mod rgba;
mod rgba32;
use std::fmt::{Debug, Formatter, LowerHex, UpperHex, Write};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct RGB {
    /// The red channel of color in `[0u8, 255u8]`.
    pub r: u8,
    /// The green channel of color in `[0u8, 255u8]`.
    pub g: u8,
    /// The blue channel of color in `[0u8, 255u8]`.
    pub b: u8,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct RGBA {
    /// The red channel of color in `[0u8, 255u8]`.
    pub r: u8,
    /// The green channel of color in `[0u8, 255u8]`.
    pub g: u8,
    /// The blue channel of color in `[0u8, 255u8]`.
    pub b: u8,
    /// The alpha channel of color in `[0u8, 255u8]`.
    pub a: u8,
}

/// lossless format of all colors
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct RGBA32 {
    /// The red channel of color in `[0.0f32, 1.0f32]`.
    pub r: f32,
    /// The green channel of color in `[0.0f32, 1.0f32]`.
    pub g: f32,
    /// The blue channel of color in `[0.0f32, 1.0f32]`.
    pub b: f32,
    /// The alpha channel of color in `[0.0f32, 1.0f32]`.
    pub a: f32,
}

