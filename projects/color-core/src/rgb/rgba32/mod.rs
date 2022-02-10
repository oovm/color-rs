use super::*;

mod convert;
mod traits;

impl RGBA32 {
    /// Normalize [`RGBA32`] color.
    pub fn normalized(&self) -> Self {
        Self {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            a: self.a.min(1.0).max(0.0),
        }
    }
    pub fn gray(value: f32) -> Self {
        Self { r: value, g: value, b: value, a: 1.0 }
    }
    /// Create [`RGBA32`] from `(f32, f32, f32, f32)` tuple.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }.normalized()
    }
    /// Create [`RGBA32`] from `(u8, u8, u8)` tuple.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }.into()
    }
    /// Create [`RGBA32`] from `(u8, u8, u8, u8)` tuple.
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA { r, g, b, a }.into()
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
    /// Map operator to all channels.
    pub fn map_all<F>(&self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self { r: f(self.r), g: f(self.g), b: f(self.b), a: f(self.a) }
    }
}
