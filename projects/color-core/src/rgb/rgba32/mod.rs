use super::*;

mod traits;

impl Default for RGBA32 {
    fn default() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
}

impl RGBA32 {
    /// Create [`RGBA32`] from `(f32, f32, f32)` tuple.
    pub fn gray(value: f32) -> Self {
        Self { r: value, g: value, b: value, a: 1.0 }
    }
    /// Create [`RGBA32`] from `(f32, f32, f32, f32)` tuple.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }.normalized()
    }
}

impl RGBA32 {
    /// Create [`RGBA32`] from `(u8, u8, u8)` tuple.
    pub fn as_rgb(&self) -> RGB8 {
        RGB8::new((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }
    /// Create [`RGBA32`] from `(u8, u8, u8, u8)` tuple.
    pub fn as_rgba(&self) -> RGBA8 {
        RGBA8::new((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8, (self.a * 255.0) as u8)
    }
    /// Normalize [`RGBA32`] color to range `[0.0, 1.0]`.
    pub fn normalized(&self) -> Self {
        Self {
            r: self.r.max(0.0).min(1.0),
            g: self.g.max(0.0).min(1.0),
            b: self.b.max(0.0).min(1.0),
            a: self.a.max(0.0).min(1.0),
        }
    }
    /// Map operator to r, g, b channels, without alpha channel.
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self { r: f(self.r), g: f(self.g), b: f(self.b), a: self.a }
    }
    /// Map operator to alpha channel only.
    pub fn map_alpha<F>(&self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self { r: self.r, g: self.g, b: self.b, a: f(self.a) }
    }
    pub fn map_all<F>(&self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self { r: f(self.r), g: f(self.g), b: f(self.b), a: f(self.a) }
    }
}
