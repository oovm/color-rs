mod display;
mod convert;
mod named_colors;

pub struct RGB {
    /// The red channel of color in `[0u8, 255u8]`.
    pub r: u8,
    /// The green channel of color in `[0u8, 255u8]`.
    pub g: u8,
    /// The blue channel of color in `[0u8, 255u8]`.
    pub b: u8,
}

pub struct RGBA {
    /// The red channel of color in `[0u8, 255u8]`.
    pub r: u8,
    /// The green channel of color in `[0u8, 255u8]`.
    pub g: u8,
    /// The blue channel of color in `[0u8, 255u8]`.
    pub b: u8,
    /// The alpha channel of color in `[0u8, 255u8]`.
    pub a: u8,
}

/// lossless format of all colors
pub struct RGBA32 {
    /// The red channel of color in `[0.0f32, 1.0f32]`.
    pub r: f32,
    /// The green channel of color in `[0.0f32, 1.0f32]`.
    pub g: f32,
    /// The blue channel of color in `[0.0f32, 1.0f32]`.
    pub b: f32,
    /// The alpha channel of color in `[0.0f32, 1.0f32]`.
    pub a: f32,
}


impl RGBA32 {
    pub fn normalize(&mut self) {
        self.r = self.r.min(1.0).max(0.0);
        self.g = self.g.min(1.0).max(0.0);
        self.b = self.b.min(1.0).max(0.0);
        self.a = self.a.min(1.0).max(0.0);
    }
}