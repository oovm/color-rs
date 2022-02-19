use super::*;

impl Display for RGB8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({} {} {})", self.r, self.g, self.b)
    }
}

impl Display for RGBA8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({} {} {} / {:.0}%)", self.r, self.g, self.b, self.a as f32 / 255.0)
    }
}

impl Display for RGBA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let RGBA32 { r: r, g, b, a: a } = self;
        match f.alternate() {
            true => write!(f, "rgba({} {} {} / {}%)", r * 255.0, g * 255.0, b * 255.0, a * 100.0),
            false => write!(f, "color(srgb {} {} {} / {})", r, g, b, a),
        }
    }
}

impl UpperHex for RGBA8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

impl UpperHex for RGB8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl UpperHex for RGBA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rgba: RGBA8 = self.normalized().into();
        UpperHex::fmt(&rgba, f)
    }
}

impl LowerHex for RGBA8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }
}

impl LowerHex for RGB8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl LowerHex for RGBA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rgba: RGBA8 = self.normalized().into();
        LowerHex::fmt(&rgba, f)
    }
}
