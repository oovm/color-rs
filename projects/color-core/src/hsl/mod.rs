use crate::RGBA32;
use std::fmt::{Debug, Display, Formatter};

mod hsla32;

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
