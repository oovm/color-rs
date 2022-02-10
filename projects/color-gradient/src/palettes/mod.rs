use std::collections::BTreeMap;
use color_core::{RGBA32};
use float01::float32::f01;
use crate::ColorMap;

pub struct Palette {
    // key is a float from 0-1
    colors: BTreeMap<f01, RGBA32>,
}

impl ColorMap for Palette {
    fn get_color(&self, ratio: f32) -> RGBA32 {
        let value = f01::new(ratio);
        let mut min = (RGBA32::gray(0.0), 0.0);
        for (key, color) in self.colors.iter() {
            if key < &value {
                min = (*color, f32::from(*key));
            } else {
                break;
            }
        }
        min.0
    }

    fn get_color_span(&self, ratio: f32) -> ColorSpan {
        let value = f01::new(ratio);
        let mut min = (RGBA32::gray(0.0), 0.0);
        let mut max = (RGBA32::gray(1.0), 1.0);
        for (key, color) in self.colors.iter() {
            if key < &value {
                min = (*color, f32::from(*key));
            } else {
                max = (*color, f32::from(*key));
                break;
            }
        }
        ColorSpan { value: f32::from(value), min, max }
    }
}


impl Default for Palette {
    fn default() -> Self {
        Palette::new(RGBA32::gray(0.0), RGBA32::gray(1.0))
    }
}

pub struct ColorSpan {
    pub value: f32,
    pub min: (RGBA32, f32),
    pub max: (RGBA32, f32),
}

impl Palette {
    pub fn new(zero: RGBA32, one: RGBA32) -> Self {
        let mut out = Palette {
            colors: Default::default(),
        };
        out.add(0.0, zero);
        out.add(1.0, one);
        out
    }
    pub fn add(&mut self, position: f32, color: RGBA32) {
        self.colors.insert(f01::new(position), color);
    }
    pub fn add_u8(&mut self, position: u8, color: RGBA32) {
        self.add(position as f32 / 255.0, color);
    }
}