use super::*;

impl From<&RGBA32> for RGBA32 {
    fn from(rgba: &RGBA32) -> Self {
        *rgba
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
