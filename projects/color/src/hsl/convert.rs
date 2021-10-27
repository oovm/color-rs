use super::*;

impl From<HSLA32> for RGBA32 {
    /// <https://www.w3.org/TR/css-color-4/#hsl-to-rgb>
    fn from(hsla: HSLA32) -> Self {
        let t2 = match hsla.l <= 0.5 {
            true => { hsla.l * (hsla.s + 1.0) }
            false => { hsla.l + hsla.s - hsla.l * hsla.s }
        };
        let t1 = hsla.l * 2. - t2;
        let hue_to_rgb = |h6: f32| -> f32 {
            if h6 < 1. {
                (t2 - t1) * h6 + t1
            } else if h6 < 3. {
                t2
            } else if h6 < 4. {
                (t2 - t1) * (4. - h6) + t1
            } else {
                t1
            }
        };
        let h6 = hsla.h * 6.;
        let h6_red = if h6 + 2. < 6. { h6 + 2. } else { h6 - 4. };
        let h6_blue = if h6 - 2. >= 0. { h6 - 2. } else { h6 + 4. };
        Self {
            r: hue_to_rgb(h6_red),
            g: hue_to_rgb(h6),
            b: hue_to_rgb(h6_blue),
            a: hsla.a,
        }
    }
}