use super::*;

mod convert;
mod named_colors;
mod traits;

impl RGB {
    /// Creates a new [`RGB`] color from the given values.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    /// Map operator to all channels.
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(u8) -> u8,
    {
        Self { r: f(self.r), g: f(self.g), b: f(self.b) }
    }
}
