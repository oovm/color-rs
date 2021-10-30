use std::fmt::{Debug, Formatter};

use crate::RGBA32;

mod convert;
mod display;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct HSLA32 {
    /// Hue is a degree on the color wheel from `[0f32, 360f32)`.
    ///
    /// 0 is red, 120 is green, 240 is blue.
    pub h: f32,
    /// Saturation is a percentage value in `[0f32, 100f32]`.
    ///
    /// 0% means a shade of gray and 100% is the full color.
    pub s: f32,
    /// Lightness is a percentage value in `[0f32, 100f32]`.
    ///
    /// 0% is black, 100% is white.
    pub l: f32,
    /// Alpha is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% is transparent, 100% is opaque.
    pub a: f32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct HSVA32 {
    /// Hue is a degree on the color wheel from `[0f32, 360f32)`.
    ///
    /// 0 is red, 120 is green, 240 is blue.
    pub h: f32,
    /// Saturation is a percentage value in `[0f32, 100f32]`.
    ///
    /// 0% means a shade of gray and 100% is the full color.
    pub s: f32,
    pub v: f32,
    /// Alpha is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% is transparent, 100% is opaque.
    pub a: f32,
}

impl HSLA32 {
    pub fn normalize(&mut self) {
        self.h = self.h % 360f32;
        if self.h < 0f32 {
            self.h += 360f32;
        }
        self.s = self.s.max(0f32).min(100f32);
        self.l = self.l.max(0f32).min(100f32);
        self.a = self.a.max(0f32).min(1f32);
    }
}

impl HSVA32 {
    pub fn normalize(&mut self) {
        self.h = self.h % 360f32;
        if self.h < 0f32 {
            self.h += 360f32;
        }
        self.s = self.s.max(0f32).min(100f32);
        self.v = self.v.max(0f32).min(100f32);
        self.a = self.a.max(0f32).min(1f32);
    }
}
