use super::*;

impl From<RGBA32> for RGB8 {
    fn from(rgba: RGBA32) -> Self {
        let rgba = rgba.normalized();
        Self { r: (rgba.r * 255.0) as u8, g: (rgba.g * 255.0) as u8, b: (rgba.b * 255.0) as u8, a: () }
    }
}
