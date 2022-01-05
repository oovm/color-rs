use super::*;

mod convert;
mod traits;

impl LCHA32 {
    /// Creates a new `LCHA32` from the given `LCH` color.
    pub fn normalized(&self) -> Self {
        let mut h = self.h % 1.0;
        if h < 0.0 {
            h += 1.0;
        }
        Self { l: self.l.max(0.0).min(1.0), c: self.c, h, a: self.a }
    }
}
