use super::*;


impl Default for RGB {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl UpperHex for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl LowerHex for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
