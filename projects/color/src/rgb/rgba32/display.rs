use super::*;

impl Default for RGBA32 {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl Debug for RGBA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl UpperHex for RGBA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rgba = self.clone();
        let rgba: RGBA = rgba.normalize().into();
        UpperHex::fmt(&rgba, f)
    }
}

impl LowerHex for RGBA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&RGBA::from(self), f)
    }
}