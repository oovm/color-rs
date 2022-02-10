use super::*;


impl From<f32> for f01 {
    #[inline(always)]
    fn from(value: f32) -> Self {
        f01::new(value)
    }
}

impl From<f64> for f01 {
    #[inline(always)]
    fn from(value: f64) -> Self {
        f01::new(value as f32)
    }
}

impl From<f01> for f32 {
    #[inline(always)]
    fn from(value: f01) -> Self {
        value.wrapped
    }
}

impl From<f01> for f64 {
    #[inline(always)]
    fn from(value: f01) -> Self {
        value.wrapped as f64
    }
}