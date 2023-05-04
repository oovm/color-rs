use super::*;
use crate::RGB8;

impl From<HSVA32> for RGBA8 {
    fn from(hsva: HSVA32) -> Self {
        RGBA8::from(RGBA32::from(hsva))
    }
}

#[doc = include_str!("hsva32_to_rgba32.tex")]
impl From<HSVA32> for RGBA32 {
    fn from(hsva: HSVA32) -> Self {
        hsv32_to_rgb32(hsva.h, hsva.s, hsva.v, hsva.a)
    }
}

/// # Arguments
/// * `h`: `[0°, 360°]`
/// * `s`: `[0%, 100%]`
/// * `v`: `[0%, 100%]`
/// * `a`: `[0%, 100%]`
/// # Returns
/// * `r`: `[0, 1]`
/// * `g`: `[0, 1]`
/// * `b`: `[0, 1]`
/// * `a`: `[0, 1]`
#[inline(always)]
fn hsv32_to_rgb32(h: f32, s: f32, v: f32, a: f32) -> RGBA32 {
    let c = v * s / 10000.0;
    let x = c * (1.0 - (h / 60.0 % 2.0 - 1.0).abs());
    let m = v / 100.0 - c;
    let (r, g, b) = match h {
        _ if h < 60.0 => (c, x, 0.0),
        _ if h < 120.0 => (x, c, 0.0),
        _ if h < 180.0 => (0.0, c, x),
        _ if h < 240.0 => (0.0, x, c),
        _ if h < 300.0 => (x, 0.0, c),
        _ if h < 360.0 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    RGBA32::new(r + m, g + m, b + m, a)
}

impl From<RGB8> for HSVA32 {
    fn from(rgba: RGB8) -> Self {
        HSVA32::from(RGBA32::from(rgba))
    }
}

impl From<RGBA8> for HSVA32 {
    fn from(rgba: RGBA8) -> Self {
        HSVA32::from(RGBA32::from(rgba))
    }
}

#[doc = include_str!("rgba32_to_hsva32.tex")]
impl From<RGBA32> for HSVA32 {
    fn from(rgba: RGBA32) -> Self {
        rgb32_to_hsv32(rgba.r, rgba.g, rgba.b, rgba.a)
    }
}

/// # Arguments
/// * `r`: `[0, 1]`
/// * `g`: `[0, 1]`
/// * `b`: `[0, 1]`
/// * `a`: `[0, 1]`
/// ## Returns
/// * `h`: `[0°, 360°]`
/// * `s`: `[0%, 100%]`
/// * `v`: `[0%, 100%]`
/// * `a`: `[0%, 100%]`
#[inline(always)]
fn rgb32_to_hsv32(r: f32, g: f32, b: f32, a: f32) -> HSVA32 {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let h = match delta {
        _ if delta == 0.0 => 0.0,
        _ if max == r => (g - b) / delta % 6.0,
        _ if max == g => (b - r) / delta + 2.0,
        _ if max == b => (r - g) / delta + 4.0,
        _ => 0.0,
    };
    let s = match max {
        _ if max == 0.0 => 0.0,
        _ => delta / max,
    };
    let v = max;
    HSVA32::new(h * 60.0, s * 100.0, v * 100.0, a * 100.0)
}
