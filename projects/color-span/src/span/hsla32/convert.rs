use super::*;

impl From<RGBA32> for HSLA32 {
    /// <https://www.w3.org/TR/css-color-4/#rgb-to-hsl>
    fn from(rgba: RGBA32) -> Self {
        let max = rgba.r.max(rgba.g).max(rgba.b);
        let min = rgba.r.min(rgba.g).min(rgba.b);
        let c = max - min;
        let h = match max {
            r if r == rgba.r => (rgba.g - rgba.b) / c,
            g if g == rgba.g => (rgba.b - rgba.r) / c + 2.0,
            b if b == rgba.b => (rgba.r - rgba.g) / c + 4.0,
            // c == 0
            _ => 0.0,
        };
        let h = h / 6.0;
        let l = (max + min) / 2.0;
        let s = match max == min {
            true => 0.0,
            false => c / (1.0 - (2.0 * l - 1.0).abs()),
        };
        Self { h, s, l, a: rgba.a }
    }
}
