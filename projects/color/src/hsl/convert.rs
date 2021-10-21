use super::*;


impl From<HSLA> for RGBA32 {
    fn from(hsla: HSLA) -> Self {
        Self {
            r: hsla.r as f32 / 255.0,
            g: hsla.g as f32 / 255.0,
            b: hsla.b as f32 / 255.0,
            a: hsla.a as f32 / 255.0,
        }
    }
}