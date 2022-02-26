use crate::ColorGradient;
use color_core::RGBA32;

pub struct LinearGradient {
    min: f32,
    max: f32,
    gamma: f32,
}

impl LinearGradient {
    pub fn new(min: f32, max: f32) -> Self {
        debug_assert!(max > min, "max must be greater than min");
        Self { min, max, ..Default::default() }
    }
}

impl Default for LinearGradient {
    fn default() -> Self {
        Self { min: 0.0, max: 1.0, gamma: 2.2 }
    }
}

impl ColorGradient for LinearGradient {}
