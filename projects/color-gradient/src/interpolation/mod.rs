use std::collections::BTreeMap;

type Resolution = u16;

/// A color interpolator that interpolates between colors in the [RGB Color Space](https://en.wikipedia.org/wiki/RGB_color_space).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Interpolator {
    keys: BTreeMap<Resolution, f32>,
    lhs: f32,
    rhs: f32,
}

impl Default for Interpolator {
    fn default() -> Self {
        Self { keys: Default::default(), lhs: 0.0, rhs: 1.0 }
    }
}

impl Interpolator {
    /// Create a new interpolator with the given lower and upper bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::Interpolator;
    /// let interpolator = Interpolator::new(0.0, 1.0);
    /// assert_eq!(interpolator.head(), 0.0);
    /// assert_eq!(interpolator.tail(), 1.0);
    /// ```
    pub fn new(lower: f32, upper: f32) -> Self {
        Self { keys: Default::default(), lhs: lower, rhs: upper }
    }
    /// Get the left most value of the interpolator, if not set, the lower bound is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::Interpolator;
    /// let mut interpolator = Interpolator::default();
    /// assert_eq!(interpolator.head(), 0.0);
    /// interpolator.insert(0, 0.5);
    /// assert_eq!(interpolator.head(), 0.5);
    /// ```
    pub fn head(&self) -> f32 {
        self.keys.get(&Resolution::MIN).copied().unwrap_or(self.lhs)
    }
    /// Get the right most value of the interpolator, if not set, the upper bound is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::Interpolator;
    /// let mut interpolator = Interpolator::default();
    /// assert_eq!(interpolator.tail(), 1.0);
    /// interpolator.insert(65535, 0.5);
    /// assert_eq!(interpolator.tail(), 0.5);
    /// ```
    pub fn tail(&self) -> f32 {
        self.keys.get(&Resolution::MAX).copied().unwrap_or(self.rhs)
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
    /// # use colormap::HsvGradient;
    /// ```
    pub fn insert(&mut self, key: Resolution, value: f32) {
        self.keys.insert(key, value);
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
    /// # use colormap::HsvGradient;
    /// ```
    pub fn remove(&mut self, key: Resolution) {
        self.keys.remove(&key);
    }
}

impl Interpolator {
    /// Get zero-order interpolation, that is, the first number greater than ratio
    pub fn get_step(&self, key: Resolution) -> f32 {
        if key == Resolution::MIN {
            self.head()
        }
        else if key == Resolution::MAX {
            self.tail()
        }
        else {
            self.keys.range(..=key).next_back().map(|(_, v)| *v).unwrap_or(self.lhs)
        }
    }
    /// Get first-order linear interpolation
    pub fn get_linear(&self, key: Resolution) -> f32 {
        if key == Resolution::MIN {
            self.head()
        }
        else if key == Resolution::MAX {
            self.tail()
        }
        else {
            let s1 = (&Resolution::MIN, &self.head());
            let s2 = (&Resolution::MAX, &self.tail());
            let (k1, v1) = self.keys.range(..=key).next_back().unwrap_or(s1);
            let (k2, v2) = self.keys.range(key..).next().unwrap_or(s2);
            if k1 == k2 { *v1 } else { v1 + (v2 - v1) * (key - k1) as f32 / (k2 - k1) as f32 }
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
    /// # use colormap::HsvGradient;
    /// ```
    pub fn clear(&mut self) {
        self.keys.clear();
    }
}
