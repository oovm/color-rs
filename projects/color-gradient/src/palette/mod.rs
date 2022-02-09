use std::collections::BTreeMap;
use std::mem::transmute;
use color_core::{RGB, RGBA};

pub struct Palette {
    // key is a float from 0-1
    colors: BTreeMap<[u8; 4], RGBA>,
}


impl Default for Palette {
    fn default() -> Self {
        Palette::new(RGBA::gray(0), RGBA::gray(255))
    }
}

pub struct ColorSpan {
    pub min: (RGBA, f32),
    pub max: (RGBA, f32),
}

impl Palette {
    pub fn new(zero: RGBA, one: RGBA) -> Self {
        let mut out = Palette {
            colors: Default::default(),
        };
        out.add(0.0, zero);
        out.add(1.0, one);
        out
    }
    pub fn add(&mut self, position: f32, color: RGBA) {
        let normed = position.max(0.0).min(1.0);
        unsafe {
            self.colors.insert(transmute::<f32, [u8; 4]>(normed), color);
        }
    }
    pub fn add_u8(&mut self, position: u8, color: RGBA) {
        self.colors.insert(position, color);
    }
    pub fn get_colors(&self, value: f32) -> ColorSpan {
        let normed = value.max(0.0).min(1.0);
        let mut min = (RGBA::gray(0), 0.0);
        let mut max = (RGBA::gray(255), 1.0);
        for (position, color) in self.colors.iter() {
            let position = unsafe { transmute::<[u8; 4], f32>(*position) };
            if position < normed {
                min = (color.clone(), position);
            } else {
                max = (color.clone(), position);
                break;
            }
        }
        ColorSpan { min, max }
    }
}