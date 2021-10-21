mod display;
mod convert;
mod named_colors;

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// lossless format
pub struct RGBA32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}