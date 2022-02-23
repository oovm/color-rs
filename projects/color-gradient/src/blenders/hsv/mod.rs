use crate::interpolation::Interpolator;
use color_core::{HSVA32, RGBA32, RGBA8};
use image::GenericImageView;
use ordered_float::OrderedFloat;
use std::{collections::BTreeMap, ops::Range};

#[derive(Clone, Debug)]
pub struct HsvGradient {
    hue: Interpolator,
    saturation: Interpolator,
    brightness: Interpolator,
    alpha: Interpolator,
    range: Range<f32>,
}

impl HsvGradient {
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
    pub fn insert_alpha(&mut self, key: f32, value: f32) {
        let ratio = self.get_ratio(key);
        self.alpha.insert(ratio, value);
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
    pub fn remove_alpha(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.alpha.remove(ratio);
    }
    pub fn clear_alpha(&mut self) {
        self.alpha.clear();
    }
}

impl HsvGradient {
    fn get_ratio(&self, value: f32) -> u16 {
        if value <= self.range.start {
            0
        }
        else if value >= self.range.end {
            65535
        }
        else {
            let ratio = (value - self.range.start) / (self.range.end - self.range.start);
            (ratio * 65535.0) as u16
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

impl HsvGradient {
    pub fn standard(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 360.0);
        grad.insert_hue(0.0, 0.0);
        grad.insert_hue(60.0, 60.0);
        grad.insert_hue(120.0, 120.0);
        grad.insert_hue(180.0, 180.0);
        grad.insert_hue(240.0, 240.0);
        grad.insert_hue(300.0, 300.0);
        grad.insert_hue(360.0, 360.0);
        grad.rescale(min, max);
        grad
    }
    pub fn parula(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 430.00);
        grad.insert_hue(0.00, -60.00);
        grad.insert_saturation(0.00, 77.38);
        grad.insert_brightness(0.00, 65.88);
        grad.insert_hue(43.00, -60.00);
        grad.insert_saturation(43.00, 71.30);
        grad.insert_brightness(43.00, 90.20);
        grad.insert_hue(86.00, -48.39);
        grad.insert_saturation(86.00, 73.52);
        grad.insert_brightness(86.00, 99.22);
        grad.insert_hue(129.00, -31.21);
        grad.insert_saturation(129.00, 81.48);
        grad.insert_brightness(129.00, 95.29);
        grad.insert_hue(172.00, -16.31);
        grad.insert_saturation(172.00, 87.44);
        grad.insert_brightness(172.00, 87.45);
        grad.insert_hue(215.00, 2.12);
        grad.insert_saturation(215.00, 89.47);
        grad.insert_brightness(215.00, 74.51);
        grad.insert_hue(258.00, 31.60);
        grad.insert_saturation(258.00, 64.53);
        grad.insert_brightness(258.00, 79.61);
        grad.insert_hue(301.00, 60.00);
        grad.insert_saturation(301.00, 68.00);
        grad.insert_brightness(301.00, 78.43);
        grad.insert_hue(344.00, 43.87);
        grad.insert_saturation(344.00, 78.81);
        grad.insert_brightness(344.00, 92.55);
        grad.insert_hue(387.00, 49.12);
        grad.insert_saturation(387.00, 81.60);
        grad.insert_brightness(387.00, 98.04);
        grad.insert_hue(430.00, 0.00);
        grad.insert_saturation(430.00, 0.00);
        grad.insert_brightness(430.00, 100.00);
        grad.rescale(min, max);
        grad
    }
}
