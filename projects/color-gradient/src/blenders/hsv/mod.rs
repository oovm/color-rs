use crate::interpolation::Interpolator;
use color_core::{HSVA32, RGBA32, RGBA8};
use image::{ImageBuffer, Rgba};
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
}

impl HsvGradient {
    fn get_ratio(&self, value: f32) -> u8 {
        if value <= self.range.start {
            return 0;
        }
        else if value >= self.range.end {
            return 255;
        }
        else {
            let ratio = (value - self.range.start) / (self.range.end - self.range.start);
            (ratio * 255.0) as u8
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
        let mut grad = HsvGradient::new(min, max);
        grad.insert_hue(0.0, 0.0);
        grad.insert_hue(60.0, 60.0);
        grad.insert_hue(120.0, 120.0);
        grad.insert_hue(180.0, 180.0);
        grad.insert_hue(240.0, 240.0);
        grad.insert_hue(300.0, 300.0);
        grad.insert_hue(360.0, 360.0);
        grad
    }
}

#[test]
fn test2() {
    let hsv = HsvGradient::standard(0.0, 360.0);
    for i in 0..360 {
        let hsva = hsv.get_step(i as f32);
        println!("{} {} {} {}", hsva.h, hsva.s, hsva.v, hsva.a);
        let RGBA32 { r, g, b, a } = hsva.into();
        println!("{} {} {} {}", r, g, b, a);
    }
}

#[test]
fn test() {
    let hsv = HsvGradient::standard(0.0, 1000.0);
    let mut img = ImageBuffer::new(1000, 100);
    for (x, _, pixel) in img.enumerate_pixels_mut() {
        let hsva = hsv.get_step(x as f32);
        let RGBA8 { r, g, b, a } = hsva.into();
        *pixel = Rgba([r, g, b, a]);
    }
    img.save("test.png").unwrap();
}
