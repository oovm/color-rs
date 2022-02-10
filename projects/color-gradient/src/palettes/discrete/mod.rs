use itertools::Itertools;
use super::*;

mod display;

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
    pub fn count(&self) -> usize {
        self.colors.len()
    }
    pub fn color_spans(&self) -> impl Iterator<Item=ColorSpan> + '_ {
        self.colors.iter().tuple_windows().map(|((key1, color1), (key2, color2))| {
            ColorSpan {
                value: f32::from(*key1),
                min: (*color1, f32::from(*key1)),
                max: (*color2, f32::from(*key2)),
            }
        })
    }
}