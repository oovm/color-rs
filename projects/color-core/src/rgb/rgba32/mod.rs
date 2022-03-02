use super::*;

mod convert;
mod traits;

impl Default for RGBA32 {
    fn default() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
}

impl RGBA32 {
    /// Create a new gray color in the [RGB Color Space](https://en.wikipedia.org/wiki/RGB_color_model).
    ///
    /// # Arguments
    ///
    /// Input values are automatically normalized.
    ///
    /// The normalized [`RGBA32`] range is:
    ///
    /// * `gray`: `[0, 1]`
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_core::{RGBA32};
    /// let rgba32 = RGBA32::gray(0.5).with_alpha(0.5);
    /// ```
    pub fn gray(value: f32) -> Self {
        let gray = value.max(0.0).min(1.0);
        Self { r: gray, g: gray, b: gray, a: 1.0 }
    }
    /// Create a new color in the [RGB Color Space](https://en.wikipedia.org/wiki/RGB_color_model).
    ///
    /// # Arguments
    ///
    /// Input values are automatically normalized.
    ///
    /// The normalized [`RGBA32`] range is:
    ///
    /// * `r`: `[0, 1]`
    /// * `g`: `[0, 1]`
    /// * `b`: `[0, 1]`
    /// * `a`: `[0, 1]`
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_core::{RGBA32};
    /// let rgba32 = RGBA32::new(1.0, 0.0, 0.0, 1.0);
    /// ```
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }.normalized()
    }
}

impl RGBA32 {
    /// Normalize [`RGBA32`] color to range `[0.0, 1.0]`.
    ///
    /// The normalized [`RGBA32`] range is:
    ///
    /// * `r`: `[0, 1]`
    /// * `g`: `[0, 1]`
    /// * `b`: `[0, 1]`
    /// * `a`: `[0, 1]`
    pub fn normalized(&self) -> Self {
        Self {
            r: self.r.max(0.0).min(1.0),
            g: self.g.max(0.0).min(1.0),
            b: self.b.max(0.0).min(1.0),
            a: self.a.max(0.0).min(1.0),
        }
    }
    /// If γ >= 1 , it is a gamma expansion, and if γ < 1, it is a gamma compression.
    pub fn gamma_corrected(&self, gamma: f32) -> Self {
        debug_assert!(gamma >= 0.0, "γ correction must be >= 0.0");
        Self { r: self.r.powf(gamma), g: self.g.powf(gamma), b: self.b.powf(gamma), a: self.a }
    }
    /// If γ >= 1 , it is a gamma expansion, and if γ < 1, it is a gamma compression.
    pub fn map_all<F>(&self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self { r: f(self.r), g: f(self.g), b: f(self.b), a: f(self.a) }
    }
}
