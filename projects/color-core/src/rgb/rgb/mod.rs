use super::*;

mod named_colors;
mod traits;

impl Default for RGB8 {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: () }
    }
}

impl RGB8 {
    /// Creates a new gray color from the given value.
    pub fn gray(value: u8) -> Self {
        Self { r: value, g: value, b: value, a: () }
    }
    /// Creates a new rgb color from the given values.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: () }
    }
}
