use crate::HSLA32;

use super::*;

impl<T> From<&T> for RGBA32
where
    Self: From<<T as ToOwned>::Owned>,
    T: ToOwned,
{
    fn from(rgba: &T) -> Self {
        rgba.to_owned().into()
    }
}

impl From<RGB> for RGBA32 {
    fn from(rgb: RGB) -> Self {
        RGBA32 { r: rgb.r as f32 / 255.0, g: rgb.g as f32 / 255.0, b: rgb.b as f32 / 255.0, a: 1.0 }
    }
}

impl From<RGBA> for RGBA32 {
    fn from(rgba: RGBA) -> Self {
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
