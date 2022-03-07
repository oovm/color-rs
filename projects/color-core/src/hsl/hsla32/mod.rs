use super::*;

mod convert;
mod traits;

impl HSLA32 {
    /// Normalize [`HSLA32`] color.
    pub fn normalized(&self) -> Self {
        let mut h = self.h % 1.0;
        if h < 0.0 {
            h += 1.0;
        }
        Self { h, s: self.s.max(0.0).min(1.0), l: self.l.max(0.0).min(1.0), a: self.a.max(0.0).min(1.0) }
    }
    /// Construct a new [`HSLA32`] color.
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self { h, s, l, a }.normalized()
    }
}
