use super::*;

pub struct LinearGradient {

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