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
