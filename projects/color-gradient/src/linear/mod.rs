use color_core::RGBA;
use crate::ColorGradient;
use crate::palette::Palette;

pub struct LinearGradient {
    palette: Palette,
    min: f64,
    max: f64,
}

impl ColorGradient for LinearGradient {
    fn get_color(&self, position: f32) -> RGBA {
        let position = (position * 255.0) as u8;
        self.palette.get_colors().get(&position).unwrap().clone()
    }
    fn get_scatter(&self, position: f32) -> f32 {
        let position = (position * 255.0) as u8;
        self.palette.get_colors().get(&position).unwrap().clone()
    }
}