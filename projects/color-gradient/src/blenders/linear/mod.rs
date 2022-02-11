use super::*;
use crate::ColorMap;

pub struct LinearGradient {
    min: f32,
    max: f32,
}

impl Default for LinearGradient {
    fn default() -> Self {
        Self { min: 0.0, max: 1.0 }
    }
}

impl LinearGradient {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
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
    fn get_color(&self, palette: &impl ColorMap, position: f32) -> RGBA32 {
        let ratio = (position - self.min) / (self.max - self.min);
        let span = palette.get_color_span(ratio);
        self.blend(span.min.0, span.max.0, span.value)
    }
}
