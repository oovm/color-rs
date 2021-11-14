use super::*;

mod display;

impl Default for HSLA32 {
    fn default() -> Self {
        Self { h: 0.0, s: 0.0, l: 0.0, a: 1.0 }
    }
}

impl HSLA32 {
    /// Normalize [`HSLA32`] color.
    pub fn normalized(&self) -> Self {
        let mut h = self.h % 360.0;
        if h < 0.0 {
            h += 360.0;
        }
        Self { h, s: self.s.max(0.0).min(1.0), l: self.l.max(0.0).min(1.0), a: self.a.max(0.0).min(1.0) }
    }
}
