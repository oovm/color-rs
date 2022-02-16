mod display;

pub type HSV8 = HSVAColor<u8, ()>;
pub type HSVA8 = HSVAColor<u8, u8>;
pub type HSV32 = HSVAColor<f32, ()>;
pub type HSVA32 = HSVAColor<f32, f32>;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct HSVAColor<T, A = T> {
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
