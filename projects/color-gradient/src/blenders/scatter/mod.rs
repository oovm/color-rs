use crate::ColorMap;
use super::*;


pub struct ScatterGradient {}

impl ColorGradient for ScatterGradient {
    fn blend(&self, lhs: RGBA32, _: RGBA32, _: f32) -> RGBA32 {
        lhs
    }
    fn get_color(&self, palette: &impl ColorMap, position: f32) -> RGBA32 {
        palette.get_color(position)
    }
}