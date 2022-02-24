use super::*;

impl RGBA32 {
    /// Create [`RGBA32`] from `(u8, u8, u8)` tuple.
    pub fn as_rgb(&self) -> RGB8 {
        RGB8::new((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }
    /// Create [`RGBA32`] from `(u8, u8, u8, u8)` tuple.
    pub fn as_rgba(&self) -> RGBA8 {
        RGBA8::new((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8, (self.a * 255.0) as u8)
    }
}
