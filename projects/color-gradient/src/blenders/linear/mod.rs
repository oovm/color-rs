use color_core::{RGBA, RGBA32};
use crate::ColorGradient;
use crate::palette::{ColorSpan, Palette};
use float01::float32::f01;

pub struct LinearGradient {
    min: f32,
    max: f32,
}

impl ColorGradient for LinearGradient {
    fn blend(&self, lhs: RGBA32, rhs: RGBA32, ratio: f32) -> RGBA32 {
        RGBA32 {
            r: lhs.r + (rhs.r - lhs.r) * ratio,
            g: lhs.g + (rhs.g - lhs.g) * ratio,
            b: lhs.b + (rhs.b - lhs.b) * ratio,
            a: lhs.a + (rhs.a - lhs.a) * ratio,
        }
    }
}