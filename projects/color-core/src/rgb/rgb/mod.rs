use super::*;

mod convert;
mod display;
mod named_colors;

impl Default for RGB {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl RGB {
    /// Creates a new RGB color_parser from the given values.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
