use super::*;

mod display;
mod convert;

impl RGBA32 {
    pub fn normalize(&mut self) {
        self.r = self.r.min(1.0).max(0.0);
        self.g = self.g.min(1.0).max(0.0);
        self.b = self.b.min(1.0).max(0.0);
        self.a = self.a.min(1.0).max(0.0);
    }
}