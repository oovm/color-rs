use super::*;

/// A color interpolator that interpolates between colors in the [RGB Color Space](https://en.wikipedia.org/wiki/RGB_color_space).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RgbGradient {
    red: Interpolator,
    green: Interpolator,
    blue: Interpolator,
    alpha: Interpolator,
    range: Range<f32>,
}

impl RgbGradient {
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn new(min: f32, max: f32) -> Self {
        debug_assert!(max > min, "max must be greater than min");
        Self {
            red: Interpolator::new(0.0, 1.0),
            green: Interpolator::new(0.0, 1.0),
            blue: Interpolator::new(0.0, 1.0),
            alpha: Interpolator::new(0.0, 1.0),
            range: Range { start: min, end: max },
        }
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn rescale(&mut self, min: f32, max: f32) {
        debug_assert!(max > min, "max must be greater than min");
        self.range = Range { start: min, end: max };
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn insert_color<RGB>(&mut self, key: f32, color: RGB)
    where
        RGB: Into<RGBA32>,
    {
        let rgba = color.into();
        let ratio = self.get_ratio(key);
        self.red.insert(ratio, rgba.r);
        self.green.insert(ratio, rgba.g);
        self.blue.insert(ratio, rgba.b);
        self.alpha.insert(ratio, rgba.a);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn remove_color(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.red.remove(ratio);
        self.green.remove(ratio);
        self.blue.remove(ratio);
        self.alpha.remove(ratio);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn insert_red(&mut self, key: f32, value: f32) {
        let ratio = self.get_ratio(key);
        self.red.insert(ratio, value);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn remove_red(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.red.remove(ratio);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn insert_green(&mut self, key: f32, value: f32) {
        let ratio = self.get_ratio(key);
        self.green.insert(ratio, value);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn remove_green(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.green.remove(ratio);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn insert_blue(&mut self, key: f32, value: f32) {
        let ratio = self.get_ratio(key);
        self.blue.insert(ratio, value);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn remove_blue(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.blue.remove(ratio);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn insert_alpha(&mut self, key: f32, value: f32) {
        let ratio = self.get_ratio(key);
        self.alpha.insert(ratio, value);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn remove_alpha(&mut self, key: f32) {
        let ratio = self.get_ratio(key);
        self.alpha.remove(ratio);
    }
    /// # Arguments
    ///
    /// * `min`:
    /// * `max`:
    ///
    /// returns: RgbGradient
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::HsvGradient;
    /// ```
    pub fn clear_alpha(&mut self) {
        self.alpha.clear();
    }
}

impl RgbGradient {
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
    pub fn get_step(&self, value: f32) -> RGBA32 {
        let ratio = self.get_ratio(value);
        RGBA32 {
            r: self.red.get_step(ratio),
            g: self.green.get_step(ratio),
            b: self.blue.get_step(ratio),
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
    pub fn get_linear(&self, value: f32) -> RGBA32 {
        let ratio = self.get_ratio(value);
        RGBA32 {
            r: self.red.get_linear(ratio),
            g: self.green.get_linear(ratio),
            b: self.blue.get_linear(ratio),
            a: self.alpha.get_linear(ratio),
        }
    }
}
