use std::fmt::{Debug, Formatter};

use crate::RGBA32;

mod convert;
mod display;
mod hsla32;

/// A color_parser in HSLA color_parser space with 32-bit floating point components.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct HSLA32 {
    /// Hue is a degree on the color_parser wheel from `[0f32, 360f32)`.
    ///
    /// 0 is red, 120 is green, 240 is blue.
    pub h: f32,
    /// Saturation is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% means a shade of gray and 100% is the full color_parser.
    pub s: f32,
    /// Lightness is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% is black, 100% is white.
    pub l: f32,
    /// Alpha is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% is transparent, 100% is opaque.
    pub a: f32,
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

impl HSVA32 {
    /// Normalize [`HSVA32`] color_parser.
    pub fn normalized(&self) -> HSVA32 {
        let mut h = self.h % 360.0;
        if h < 0.0 {
            h += 360.0;
        }
        Self { h, s: self.s.max(0.0).min(1.0), v: self.v.max(0.0).min(1.0), a: self.a.max(0.0).min(1.0) }
    }
}
