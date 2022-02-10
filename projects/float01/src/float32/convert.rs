use super::*;


impl From<f32> for f01 {
    fn from(value: f32) -> Self {
        Self { wrapped: value }
    }
}

impl From<f64> for f01 {
    fn from(value: f64) -> Self {
        Self { wrapped: value as f32 }
    }
}

impl From<f01> for f32 {
    fn from(value: f01) -> Self {
        value.wrapped
    }
}

impl From<f01> for f64 {
    fn from(value: f01) -> Self {
        value.wrapped as f64
    }
}