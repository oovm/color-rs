use std::fmt::{Display, Formatter};

mod lcha32;

///
#[derive(Debug, Clone, Copy)]
pub struct LCHA32 {
    /// **Lightness** is a percentage value in `[0, 1]`.
    ///
    /// 0% is black, 100% is white.
    pub l: f32,
    /// **Chroma** roughly representing the "amount of color".
    ///
    /// Its minimum useful value is 0, while its maximum is theoretically unbounded, but in practice does not exceed 230.
    pub c: f32,
    /// **Hue** is a percent on the color wheel from `[0, 1)`.
    ///
    /// 0 is red, 1/3 is green, 2/3 is blue.
    pub h: f32,
    /// **Alpha** is a percentage value in `[0, 1]`.
    ///
    /// 0% is transparent, 100% is opaque.
    pub a: f32,
}

///
#[derive(Debug, Clone, Copy)]
pub struct LABA32 {
    pub l: f32,
    pub c: f32,
    pub h: f32,
    pub a: f32,
}
