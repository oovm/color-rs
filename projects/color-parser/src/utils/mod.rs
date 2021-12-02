use crate::Rgba;
use nom::{
    error::{Error, ErrorKind},
    Err,
};

pub type ErrorMessage = Err<Error<&'static str>>;

pub fn nom_error<T>(kind: ErrorKind, input: &'static str) -> Result<T, Err<Error<&'static str>>> {
    Err(Err::Error(Error::new(input, kind)))
}

pub fn clamp(value: f32) -> f32 {
    value.max(0.).min(1.)
}

impl Rgba {
    /// Create [`RGBA32`] color from `(u8, u8, u8)` tuple.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 }
    }
    /// Create [`RGBA32`] color from `(u8, u8, u8, u8)` tuple.
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a as f32 / 255.0 }
    }
}
