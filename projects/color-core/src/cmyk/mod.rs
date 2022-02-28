use std::fmt::{Debug, Formatter};

mod cmyk32;

/// 128-bit 4 channel color in the [CMYK Color Space](https://en.wikipedia.org/wiki/CMYK_color_model).
pub type CMYK32 = CMYKColor<f32>;

/// A color in the [CMYK Color Space](https://en.wikipedia.org/wiki/CMYK_color_model).
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct CMYKColor<T> {
    /// Cyan is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% means no cyan and 100% is the maximum amount of cyan.
    pub c: T,
    /// Magenta is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% means no magenta and 100% is the maximum amount of magenta.
    pub m: T,
    /// Yellow is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% means no yellow and 100% is the maximum amount of yellow.
    pub y: T,
    /// Black is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% means no key and 100% is the maximum amount of key.
    pub k: T,
}
