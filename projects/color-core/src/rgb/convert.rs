use super::*;
use crate::HSLA32;

impl Default for RGBA32 {
    fn default() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
}
impl Default for RGBA8 {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl Default for RGB8 {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: () }
    }
}

impl From<RGBA32> for RGBA8 {
    fn from(rgba: RGBA32) -> Self {
        let rgba = rgba.normalized();
        Self { r: (rgba.r * 255.0) as u8, g: (rgba.g * 255.0) as u8, b: (rgba.b * 255.0) as u8, a: (rgba.a * 255.0) as u8 }
    }
}

impl<T> From<[T; 3]> for RGBA8
where
    T: Into<u8> + Copy,
{
    fn from(rgba: [T; 3]) -> Self {
        Self { r: rgba[0].into(), g: rgba[1].into(), b: rgba[2].into(), a: 255 }
    }
}

// noinspection DuplicatedCode
impl<T> From<[T; 4]> for RGBA8
where
    T: Into<u8> + Copy,
{
    fn from(rgba: [T; 4]) -> Self {
        Self { r: rgba[0].into(), g: rgba[1].into(), b: rgba[2].into(), a: rgba[3].into() }
    }
}

impl From<u32> for RGBA8 {
    #[track_caller]
    fn from(rgba: u32) -> Self {
        // if rgba < 0xFFFFFF00 {
        //     panic!("Invalid color value: #{:02X}", rgba);
        // }
        let [r, g, b, a] = rgba.to_be_bytes();
        Self { r, g, b, a }
    }
}

impl<T> From<&T> for RGBA32
where
    Self: From<<T as ToOwned>::Owned>,
    T: ToOwned,
{
    fn from(rgba: &T) -> Self {
        rgba.to_owned().into()
    }
}

impl From<RGB8> for RGBA32 {
    fn from(rgb: RGB8) -> Self {
        RGBA32 { r: rgb.r as f32 / 255.0, g: rgb.g as f32 / 255.0, b: rgb.b as f32 / 255.0, a: 1.0 }
    }
}

impl From<RGBA8> for RGBA32 {
    fn from(rgba: RGBA8) -> Self {
        Self { r: rgba.r as f32 / 255.0, g: rgba.g as f32 / 255.0, b: rgba.b as f32 / 255.0, a: rgba.a as f32 / 255.0 }
    }
}

impl From<HSLA32> for RGBA32 {
    /// <https://www.w3.org/TR/css-color-4/#hsl-to-rgb>
    fn from(hsla: HSLA32) -> Self {
        let HSLA32 { h, s, l, a } = hsla.normalized();
        let ts = |n: f32| {
            let k = (n + h * 12.0) % 12.0;
            let c = s * l.min(1.0 - l);
            l - c * 1f32.min(k - 3.0).min(9.0 - k).max(-1.0)
        };
        Self { r: ts(0.0), g: ts(8.0), b: ts(4.0), a }
    }
}

impl<T> From<[T; 3]> for RGBA32
where
    T: Into<f32> + Copy,
{
    fn from(rgba: [T; 3]) -> Self {
        Self { r: rgba[0].into(), g: rgba[1].into(), b: rgba[2].into(), a: 1.0 }
    }
}

impl<T> From<[T; 4]> for RGBA32
where
    T: Into<f32> + Copy,
{
    fn from(rgba: [T; 4]) -> Self {
        Self { r: rgba[0].into(), g: rgba[1].into(), b: rgba[2].into(), a: rgba[3].into() }
    }
}

impl<R, G, B> From<(R, G, B)> for RGBA32
where
    R: Into<f32>,
    G: Into<f32>,
    B: Into<f32>,
{
    fn from(rgba: (R, G, B)) -> Self {
        Self { r: rgba.0.into(), g: rgba.1.into(), b: rgba.2.into(), a: 1.0 }
    }
}

impl<R, G, B, A> From<(R, G, B, A)> for RGBA32
where
    R: Into<f32>,
    G: Into<f32>,
    B: Into<f32>,
    A: Into<f32>,
{
    fn from(rgba: (R, G, B, A)) -> Self {
        Self { r: rgba.0.into(), g: rgba.1.into(), b: rgba.2.into(), a: rgba.3.into() }
    }
}

impl From<u32> for RGBA32 {
    fn from(rgba: u32) -> Self {
        RGBA8::from(rgba).into()
    }
}

impl From<RGBA32> for RGB8 {
    fn from(rgba: RGBA32) -> Self {
        let rgba = rgba.normalized();
        Self { r: (rgba.r * 255.0) as u8, g: (rgba.g * 255.0) as u8, b: (rgba.b * 255.0) as u8, a: () }
    }
}
