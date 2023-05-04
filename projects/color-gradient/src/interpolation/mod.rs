use std::{collections::BTreeMap, ops::Range};

type Resolution = u16;

/// A interpolator that interpolates values in range.
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
    /// Insert a new key value pair into the interpolator, overwriting any existing value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::{Interpolator};
    /// let mut gradient = Interpolator::default();
    /// gradient.insert(0, 0.5);
    /// gradient.insert(65535, 0.5);
    /// assert_eq!(gradient.head(), 0.5);
    /// assert_eq!(gradient.tail(), 0.5);
    /// ```
    pub fn insert(&mut self, key: Resolution, value: f32) {
        self.keys.insert(key, value);
    }
    /// Remove a key value pair from the interpolator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::{Interpolator};
    /// let mut gradient = Interpolator::default();
    /// gradient.insert(0, 0.5);
    /// assert_eq!(gradient.head(), 0.5);
    /// gradient.remove(0);
    /// assert_eq!(gradient.head(), 0.0);
    /// ```
    pub fn remove(&mut self, key: Resolution) {
        self.keys.remove(&key);
    }
    /// Clear all key value pairs from the interpolator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use color_gradient::{Interpolator};
    /// ```
    pub fn clear(&mut self) {
        self.keys.clear();
    }
}

impl Interpolator {
    pub(crate) fn get_ratio(range: &Range<f32>, value: f32) -> u16 {
        if value <= range.start {
            0
        }
        else if value >= range.end {
            65535
        }
        else {
            let ratio = (value - range.start) / (range.end - range.start);
            (ratio * 65535.0) as u16
        }
    }

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
    /// Get bezier interpolation
    pub fn get_bezier(&self, key: Resolution) -> f32 {
        if key == Resolution::MIN {
            self.head()
        }
        else if key == Resolution::MAX {
            self.tail()
        }
        else {
            // bezier 1D
            let s1 = (&Resolution::MIN, &self.head());
            let s2 = (&Resolution::MAX, &self.tail());
            let (k1, v1) = self.keys.range(..=key).next_back().unwrap_or(s1);
            let (k2, v2) = self.keys.range(key..).next().unwrap_or(s2);
            let t = (key - k1) as f32 / (k2 - k1) as f32;
            let v = (1.0 - t) * v1 + t * v2;
            v
        }
    }
}
