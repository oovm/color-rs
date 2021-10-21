use std::fmt::{Debug, Display, Formatter, UpperHex, write};
use super::*;

impl Debug for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGB({}, {}, {})", self.red, self.green, self.blue)
    }
}


impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGB({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl UpperHex for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl UpperHex for RGBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl From<RGBA> for RGBA32 {
    fn from(rgba: RGBA32) -> Self {
        Self { r: rgba.r, g: rgba.g, b: rgba.b, a: rgba.a }
    }
}