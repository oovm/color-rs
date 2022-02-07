use std::collections::BTreeMap;
use std::mem::transmute;
use color_core::{RGB, RGBA};

pub struct Palette {
    // key is a float from 0-1
    colors: BTreeMap<[u8; 4], RGBA>,
}

impl Default for Palette {
    fn default() -> Self {
        Palette::new(RGBA::BLACK, RGBA::WHITE)
    }
}

pub struct ColorRange {
    left: f32,
    right: f32,
}

impl Palette {
    pub fn new(zero: RGBA, one: RGBA) -> Self {
        let mut out = Palette {
            colors: Default::default(),
        };
        out.add_float(0.0, zero);
        out.add_float(1.0, one);
        out
    }
    pub fn add_float(&mut self, position: f32, color: RGBA) {
        let normed = position.max(0.0).min(1.0);
        unsafe {
            self.colors.insert(transmute::<f32, [u8; 4]>(normed), color);
        }
    }
    pub fn add(&mut self, position: u8, color: RGBA) {
        self.colors.insert(position, color);
    }
    pub fn get_colors(&self) -> &BTreeMap<u8, RGBA> {
        &self.colors
    }
}