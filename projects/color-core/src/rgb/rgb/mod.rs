use super::*;

mod convert;
mod traits;
mod named_colors;


impl RGB {
    /// Creates a new RGB color_parser from the given values.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
