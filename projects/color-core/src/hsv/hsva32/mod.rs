use super::*;
use crate::{RGBA32, RGBA8};

impl HSVA32 {
    pub fn as_rgba(&self) -> RGBA32 {
        RGBA32::from(*self)
    }
}

impl From<HSVA32> for RGBA32 {
    fn from(hsva: HSVA32) -> Self {
        let HSVA32 { h, s, v, a } = hsva;
        let (r, g, b) = hsv32_to_rgb32(h, s, v);
        Self { r, g, b, a }
    }
}

impl From<HSVA32> for RGBA8 {
    fn from(hsva: HSVA32) -> Self {
        RGBA8::from(RGBA32::from(hsva))
    }
}

/// # Arguments
///
/// * `h`: `[0, 360]`
/// * `s`: `[0, 100]`
/// * `v`: `[0, 100]`
///
/// ## Returns
///
/// * `r`: `[0, 1]`
/// * `g`: `[0, 1]`
/// * `b`: `[0, 1]`
fn hsv32_to_rgb32(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let h = h.rem_euclid(360.0);
    let s = s.rem_euclid(100.0) / 100.0;
    let v = v.rem_euclid(100.0) / 100.0;
    let c = v * s;
    let x = c * (1.0 - (h / 60.0 % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (r + m, g + m, b + m)
}
