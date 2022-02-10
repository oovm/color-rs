use std::collections::BTreeMap;
use color_core::{RGB, RGBA, RGBA32};
use float01::float32::f01;


pub struct Palette {
    // key is a float from 0-1
    colors: BTreeMap<f01, RGBA32>,
}


impl Default for Palette {
    fn default() -> Self {
        Palette::new(RGBA::gray(0.0), RGBA::gray(1.0))
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
    pub fn add(&mut self, position: f32, color: RGBA) {
        self.colors.insert(f01::new(position), color);
    }
    pub fn add_u8(&mut self, position: u8, color: RGBA) {
        self.add(position as f32 / 255.0, color);
    }
    pub fn get_range<V: Into<f01>>(&self, value: V) -> ColorSpan {
        let value = value.into();
        let mut min = (RGBA::gray(0), 0.0);
        let mut max = (RGBA::gray(255), 1.0);
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