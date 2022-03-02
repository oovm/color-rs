mod display;
mod hsva32;

/// 128-bit 4 channel color in the [HSV Color Space](https://en.wikipedia.org/wiki/HSL_and_HSV).
pub type HSVA32 = HSVColor<f32, f32>;

/// A color in the [HSV Color Space](https://en.wikipedia.org/wiki/HSL_and_HSV).
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HSVColor<T, A = T> {
    /// Hue is a degree on the color_parser wheel from `[0f32, 360f32)`.
    ///
    /// 0 is red, 120 is green, 240 is blue.
    pub h: T,
    /// Saturation is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% means a shade of gray and 100% is the full color_parser.
    pub s: T,
    /// Saturation is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% means a shade of gray and 100% is the full color_parser.
    pub v: T,
    /// Alpha is a percentage value in `[0f32, 1f32]`.
    ///
    /// 0% is transparent, 100% is opaque.
    pub a: A,
}

impl<T, A> HSVColor<T, A> {
    /// Build a new color with the given hue.
    pub fn with_hue(self, h: T) -> Self {
        Self { h, ..self }
    }
    /// Build a new color with the given saturation.
    pub fn with_saturation(self, s: T) -> Self {
        Self { s, ..self }
    }
    /// Build a new color with the given brightness.
    pub fn with_brightness(self, v: T) -> Self {
        Self { v, ..self }
    }
    /// Build a new color with the given α.
    pub fn with_alpha(self, a: A) -> Self {
        Self { a, ..self }
    }
}
