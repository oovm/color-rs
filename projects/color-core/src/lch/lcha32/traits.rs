use std::fmt::{Display, Formatter};
use super::*;

impl Default for LCHA32 {
    fn default() -> Self {
        Self { l: 0.0, c: 0.0, h: 0.0, a: 1.0 }
    }
}

impl Display for LCHA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let LCHA32 { l, c, h, a } = self.normalized();
        match f.alternate() {
            true => write!(f, "lch({} {}% {}% / {}%)", l * 360.0, c * 100.0, h * 360.0, a * 100.0),
            false => write!(f, "lch({:.0} {:.0}% {:.0}% / {:.0}%)", l * 360.0, c * 360.0, h * 100.0, a * 100.0),
        }
    }
}
