use super::*;
use crate::{RGBA32, RGBA8};
mod convert;

impl HSVA32 {
    pub fn as_rgba(&self) -> RGBA32 {
        RGBA32::from(*self)
    }
    pub fn normalize(&self) -> Self {
        let h = self.h.rem_euclid(360.0);
        let s = self.s.min(100.0).max(0.0);
        let v = self.v.min(100.0).max(0.0);
        let a = self.a.min(100.0).max(0.0);
        Self { h, s, v, a }
    }
}
