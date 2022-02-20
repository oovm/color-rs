use super::*;
use crate::{RGBA32, RGBA8};

impl HSVA32 {
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self { h, s, v, a }
    }
    pub fn as_rgba(&self) -> RGBA32 {
        RGBA32::from(*self)
    }
    pub fn normalize(&self) -> Self {
        let h = self.h.rem_euclid(360.0);
        let s = self.s.min(100.0).max(0.0);
        let v = self.v.min(100.0).max(0.0);
        let a = self.a.min(100.0).max(0.0);
        Self { h, s, v, a }
    }
}

impl From<HSVA32> for RGBA8 {
    fn from(hsva: HSVA32) -> Self {
        RGBA8::from(RGBA32::from(hsva))
    }
}

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
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        h if h < 360.0 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    RGBA32::new(r + m, g + m, b + m, a)
}

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
    let c = max - min;
    let h = match max {
        r if r == max => (g - b) / c,
        g if g == max => (b - r) / c + 2.0,
        b if b == max => (r - g) / c + 4.0,
        // c == 0
        _ => 0.0,
    };
    let h = h / 6.0;
    let v = max * 100.0;
    let s = match max == min {
        true => 0.0,
        false => c / (1.0 - (2.0 * v - 100.0).abs()),
    };
    HSVA32::new(h * 360.0, s * 100.0, v, a * 100.0)
}
