use super::*;

mod traits;

impl Default for RGBA8 {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl RGBA8 {
    /// Create [`RGBA8`] from `(u8, u8, u8)` tuple.
    pub fn gray(value: u8) -> Self {
        Self { r: value, g: value, b: value, a: 255 }
    }
    /// Create [`RGBA8`] from `(u8, u8, u8, u8)` tuple.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    /// Map operator to all channels.
    pub fn map_all<F>(&self, f: F) -> Self
    where
        F: Fn(u8) -> u8,
    {
        Self { r: f(self.r), g: f(self.g), b: f(self.b), a: f(self.a) }
    }
}
