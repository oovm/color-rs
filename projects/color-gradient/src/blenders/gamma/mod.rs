use color_core::RGBA32;
use crate::ColorGradient;

pub struct GammaGradient {
    gamma: f32,
}


impl GammaGradient {
    pub fn new(gamma: f32) -> Self {
        debug_assert!(gamma > 1.0, "Gamma must be greater than 1.0 (got {})", gamma);
        Self { gamma }
    }
}

impl Default for GammaGradient {
    fn default() -> Self {
        Self { gamma: 2.2 }
    }
}


impl ColorGradient for GammaGradient {
    fn blend(&self, lhs: RGBA32, rhs: RGBA32, position: f32) -> RGBA32 {
        let ga = self.gamma;
        let gi = 1.0 / ga;
        let r = lhs.r.powf(gi) + (rhs.r.powf(gi) - lhs.r.powf(gi)) * position;
        let g = lhs.g.powf(gi) + (rhs.g.powf(gi) - lhs.g.powf(gi)) * position;
        let b = lhs.b.powf(gi) + (rhs.b.powf(gi) - lhs.b.powf(gi)) * position;
        let a = lhs.a.powf(gi) + (rhs.a.powf(gi) - lhs.a.powf(gi)) * position;
        RGBA32 {
            r: r.powf(ga),
            g: g.powf(ga),
            b: b.powf(ga),
            a: a.powf(ga),
        }
    }
}