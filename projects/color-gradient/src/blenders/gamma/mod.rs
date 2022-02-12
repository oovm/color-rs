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

impl ColorGradient for LinearGradient {
    fn blend(&self, lhs: RGBA32, rhs: RGBA32, ratio: f32) -> RGBA32 {
        let ga = self.gamma;
        let gi = 1.0 / ga;
        let r = lhs.r.powf(gi) + (rhs.r.powf(gi) - lhs.r.powf(gi)) * ratio;
        let g = lhs.g.powf(gi) + (rhs.g.powf(gi) - lhs.g.powf(gi)) * ratio;
        let b = lhs.b.powf(gi) + (rhs.b.powf(gi) - lhs.b.powf(gi)) * ratio;
        let a = lhs.a.powf(gi) + (rhs.a.powf(gi) - lhs.a.powf(gi)) * ratio;
        RGBA32 { r: r.powf(ga), g: g.powf(ga), b: b.powf(ga), a: a.powf(ga) }
    }
}
