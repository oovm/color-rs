use super::*;

mod convert;
mod traits;


impl RGBA {
    /// Create [`RGBA`] from `(u8, u8, u8, u8)` tuple.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
