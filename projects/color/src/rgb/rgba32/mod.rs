use super::*;

mod convert;
mod display;

impl RGBA32 {
    pub fn normalized(&self) -> Self {
        Self {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            a: self.a.min(1.0).max(0.0),
        }
    }
}
