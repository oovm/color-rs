use super::*;

mod convert;
mod display;

impl Default for RGBA {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl RGBA {
    /// Create [`RGBA`] from `(u8, u8, u8, u8)` tuple.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
