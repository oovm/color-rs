use super::*;
use crate::{RGBA32, RGBA8};
mod convert;

impl HSVA32 {
    /// Create a new color in the [HSV Color Space](https://en.wikipedia.org/wiki/HSL_and_HSV).
    ///
    /// # Warning
    ///
    /// The raw value is created, and the legality of the data will not be verified
    ///
    /// The legal [`HSVA32`] range is:
    ///
    /// * `h`: `[0°, 360°)`
    /// * `s`: `[0%, 100%]`
    /// * `v`: `[0%, 100%]`
    /// * `a`: `[0%, 100%]`
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_core::HSVA32;
    /// let hsva32 = HSVA32::new(720.0, 100.0, 100.0, 100.0);
    /// assert_eq!(hsva32.h, 0.0);
    /// ```
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self { h, s, v, a }.normalize()
    }
    /// Make a normalized hsva color
    pub fn normalize(&self) -> Self {
        let h = self.h.rem_euclid(360.0);
        let s = self.s.min(100.0).max(0.0);
        let v = self.v.min(100.0).max(0.0);
        let a = self.a.min(100.0).max(0.0);
        Self { h, s, v, a }
    }
}
