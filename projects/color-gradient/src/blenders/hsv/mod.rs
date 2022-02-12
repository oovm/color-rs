use color_core::{HSVA32, RGBA32};
use ordered_float::OrderedFloat;
use std::{collections::BTreeMap, ops::Range};

pub struct HsvGradient {
    hue: BTreeMap<OrderedFloat<f32>, f32>,
    saturation: BTreeMap<OrderedFloat<f32>, f32>,
    brightness: BTreeMap<OrderedFloat<f32>, f32>,
    alpha: BTreeMap<OrderedFloat<f32>, f32>,
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
            hue: Default::default(),
            saturation: Default::default(),
            brightness: Default::default(),
            alpha: Default::default(),
            range: Range { start: min, end: max },
        }
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
    /// Get zero-order interpolation, that is, the first number greater than ratio
    pub fn get_step(&self, value: f32) -> HSVA32 {
        todo!()
    }
    /// Get first-order linear interpolation
    pub fn get_linear(&self, value: f32) -> HSVA32 {
        todo!()
    }
}
