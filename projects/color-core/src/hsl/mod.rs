use crate::RGBA32;
use std::fmt::{Debug, Display, Formatter};

mod hsla32;
mod hsva32;

/// A color in HSLA color space with 32-bit floating point components.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HSLA32 {
    /// **Hue** is a percent on the color wheel from `[0, 1)`.
    ///
    /// 0 is red, 1/3 is green, 2/3 is blue.
    pub h: f32,
    /// **Saturation** is a percentage value in `[0, 1]`.
    ///
    /// 0% means a shade of gray and 100% is the full color_parser.
    pub s: f32,
    /// **Lightness** is a percentage value in `[0, 1]`.
    ///
    /// 0% is black, 100% is white.
    pub l: f32,
    /// **Alpha** is a percentage value in `[0, 1]`.
    ///
    /// 0% is transparent, 100% is opaque.
    pub a: f32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct HSV32 {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

/// A color_parser in HSVA color_parser space with 32-bit floating point components.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct HSVA32 {
    /// Hue is a degree on the color_parser wheel from `[0f32, 360f32)`.
    ///
    /// 0 is red, 120 is green, 240 is blue.
    pub h: f32,
    /// Saturation is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% means a shade of gray and 100% is the full color_parser.
    pub s: f32,
    ///
    pub v: f32,
    /// Alpha is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% is transparent, 100% is opaque.
    pub a: f32,
}
