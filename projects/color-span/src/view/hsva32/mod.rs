use super::*;

mod convert;
mod traits;


impl HSVA32 {
    /// Normalize [`HSVA32`] color.
    pub fn normalized(&self) -> HSVA32 {
        let mut h = self.h % 360.0;
        if h < 0.0 {
            h += 360.0;
        }
        Self { h, s: self.s.max(0.0).min(1.0), v: self.v.max(0.0).min(1.0), a: self.a.max(0.0).min(1.0) }
    }
}
