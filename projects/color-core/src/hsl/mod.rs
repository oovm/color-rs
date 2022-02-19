use crate::RGBA32;
use std::fmt::{Debug, Display, Formatter};

mod hsla32;

pub type HSLA32 = HSLColor<f32, f32>;

/// A color in HSLA color space with 32-bit floating point components.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HSLColor<T, A = T> {
    /// **Hue** is a percent on the color wheel from `[0, 1)`.
    ///
    /// 0 is red, 1/3 is green, 2/3 is blue.
    pub h: T,
    /// **Saturation** is a percentage value in `[0, 1]`.
    ///
    /// 0% means a shade of gray and 100% is the full color_parser.
    pub s: T,
    /// **Lightness** is a percentage value in `[0, 1]`.
    ///
    /// 0% is black, 100% is white.
    pub l: T,
    /// **Alpha** is a percentage value in `[0, 1]`.
    ///
    /// 0% is transparent, 100% is opaque.
    pub a: A,
}
