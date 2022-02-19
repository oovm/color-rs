use super::*;
use crate::ColorMap;

pub struct QuadraticGradient {
    min: f32,
    max: f32,
}

impl Default for QuadraticGradient {
    fn default() -> Self {
        Self { min: 0.0, max: 1.0 }
    }
}

impl QuadraticGradient {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

impl ColorGradient for QuadraticGradient {
    fn blend(&self, lhs: RGBA32, rhs: RGBA32, ratio: f32) -> RGBA32 {
        let r = lhs.r + (rhs.r - lhs.r) * ratio * ratio;
        let g = lhs.g + (rhs.g - lhs.g) * ratio * ratio;
        let b = lhs.b + (rhs.b - lhs.b) * ratio * ratio;
        let a = lhs.a + (rhs.a - lhs.a) * ratio * ratio;
        RGBA32 { r: r, g, b, a: a }
    }
    fn get_color(&self, palette: &impl ColorMap, position: f32) -> RGBA32 {
        let ratio = (position - self.min) / (self.max - self.min);
        let span = palette.get_color_span(ratio);
        self.blend(span.min.0, span.max.0, span.value)
    }
}
