use crate::Interpolator;
use color_core::HSVA32;
use std::ops::Range;
mod builtin;

/// A color interpolator that interpolates between colors in the [HSV Color Space](https://en.wikipedia.org/wiki/HSL_and_HSV).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HsvGradient {
    hue: Interpolator,
    saturation: Interpolator,
    brightness: Interpolator,
    alpha: Interpolator,
    range: Range<f32>,
}

impl HsvGradient {
    /// Creates a new gradient sampler with the given min and max values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_core::HSVA32;
    /// # use color_gradient::{HsvGradient};
    /// let mut gradient = HsvGradient::default();
    /// assert_eq!(gradient.get_linear(0.5), HSVA32::new(180.0, 100.0, 100.0, 100.0));
    /// ```
    pub fn new(min: f32, max: f32) -> Self {
        debug_assert!(max > min, "max must be greater than min");
        Self {
            hue: Interpolator::new(0.0, 360.0),
            saturation: Interpolator::new(100.0, 100.0),
            brightness: Interpolator::new(100.0, 100.0),
            alpha: Interpolator::new(100.0, 100.0),
            range: Range { start: min, end: max },
        }
    }
    /// Rescales the gradient to the given min and max values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_core::HSVA32;
    /// # use color_gradient::{HsvGradient};
    /// let mut gradient = HsvGradient::default();
    /// gradient.rescale(0.0, 360.0);
    /// assert_eq!(gradient.get_linear(180.0), HSVA32::new(180.0, 100.0, 100.0, 100.0));
    /// ```
    pub fn rescale(&mut self, min: f32, max: f32) {
        debug_assert!(max > min, "max must be greater than min");
        self.range = Range { start: min, end: max };
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn insert_color<HSV>(&mut self, key: f32, color: HSV)
    where
        HSV: Into<HSVA32>,
    {
        let hsva = color.into();
        let ratio = self.get_ratio(key);
        self.hue.insert(ratio, hsva.h);
        self.saturation.insert(ratio, hsva.s);
        self.brightness.insert(ratio, hsva.v);
        self.alpha.insert(ratio, hsva.a);
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn remove_color(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.hue.remove(ratio);
        self.saturation.remove(ratio);
        self.brightness.remove(ratio);
        self.alpha.remove(ratio);
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn insert_hue(&mut self, key: f32, value: f32) {
        let ratio = self.get_ratio(key);
        self.hue.insert(ratio, value);
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn remove_hue(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.hue.remove(ratio);
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn insert_saturation(&mut self, key: f32, value: f32) {
        let ratio = self.get_ratio(key);
        self.saturation.insert(ratio, value);
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn remove_saturation(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.saturation.remove(ratio);
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn insert_brightness(&mut self, key: f32, value: f32) {
        let ratio = self.get_ratio(key);
        self.brightness.insert(ratio, value);
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn remove_brightness(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.brightness.remove(ratio);
    }
    /// Insert a new alpha control point at the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_core::HSVA32;
    /// # use color_gradient::HsvGradient;
    /// let mut gradient = HsvGradient::default();
    /// assert_eq!(gradient.get_linear(0.5), HSVA32::new(180.0, 100.0, 100.0, 100.0));
    /// gradient.insert_alpha(0.1, 50.0);
    /// assert_eq!(gradient.get_linear(0.5), HSVA32::new(180.0, 100.0, 100.0, 72.22203));
    /// ```
    pub fn insert_alpha(&mut self, key: f32, value: f32) {
        let ratio = self.get_ratio(key);
        self.alpha.insert(ratio, value);
    }
    /// Remove the alpha control point at the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_core::HSVA32;
    /// # use color_gradient::HsvGradient;
    /// let mut gradient = HsvGradient::default();
    /// gradient.insert_alpha(0.1, 50.0);
    /// assert_eq!(gradient.get_linear(0.5), HSVA32::new(180.0, 100.0, 100.0, 72.22203));
    /// gradient.remove_alpha(0.1);
    /// assert_eq!(gradient.get_linear(0.5), HSVA32::new(180.0, 100.0, 100.0, 100.0));
    /// ```
    pub fn remove_alpha(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.alpha.remove(ratio);
    }
    /// Clears all alpha control points from the gradient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// let mut gradient = HsvGradient::default();
    /// gradient.insert_alpha(0.0, 0.0);
    /// gradient.insert_alpha(1.0, 1.0);
    /// gradient.clear_alpha();
    /// ```
    pub fn clear_alpha(&mut self) {
        self.alpha.clear();
    }
}

impl HsvGradient {
    fn get_ratio(&self, value: f32) -> u16 {
        Interpolator::get_ratio(&self.range, value)
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn get_step(&self, value: f32) -> HSVA32 {
        let ratio = self.get_ratio(value);
        HSVA32 {
            h: self.hue.get_step(ratio),
            s: self.saturation.get_step(ratio),
            v: self.brightness.get_step(ratio),
            a: self.alpha.get_step(ratio),
        }
    }
    /// Creates a new HSVGradient with the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: HSVGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient;
    /// ```
    pub fn get_linear(&self, value: f32) -> HSVA32 {
        let ratio = self.get_ratio(value);
        HSVA32 {
            h: self.hue.get_linear(ratio),
            s: self.saturation.get_linear(ratio),
            v: self.brightness.get_linear(ratio),
            a: self.alpha.get_linear(ratio),
        }
    }
}
