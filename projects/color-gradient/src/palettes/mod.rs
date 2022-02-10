use std::collections::BTreeMap;
use color_core::{RGBA32};
use float01::float32::f01;
use crate::ColorMap;

pub mod discrete;


pub struct ColorSpan {
    pub value: f32,
    pub min: (RGBA32, f32),
    pub max: (RGBA32, f32),
}
