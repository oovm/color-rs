use super::*;


impl From<HSLA32> for RGBA32 {
    fn from(hsla: HSLA32) -> Self {
        Self {
            r: hsla.h as f32 / 255.0,
            g: hsla.s as f32 / 255.0,
            b: hsla.l as f32 / 255.0,
            a: hsla.a as f32 / 255.0,
        }
    }
}