use super::*;

impl Default for RGBA32 {
    fn default() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
}

impl Display for RGBA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let RGBA32 { r, g, b, a } = self;
        match f.alternate() {
            true => write!(f, "rgba({} {} {} / {}%)", r * 255.0, g * 255.0, b * 255.0, a * 100.0),
            false => write!(f, "color(srgb {} {} {} / {})", r, g, b, a),
        }
    }
}

impl UpperHex for RGBA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rgba: RGBA8 = self.normalized().into();
        UpperHex::fmt(&rgba, f)
    }
}

impl LowerHex for RGBA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rgba: RGBA8 = self.normalized().into();
        LowerHex::fmt(&rgba, f)
    }
}

impl Eq for RGBA32 {}

impl PartialEq<Self> for RGBA32 {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.normalized();
        let rhs = other.normalized();
        lhs.r == rhs.r && lhs.g == rhs.g && lhs.b == rhs.b && lhs.a == rhs.a
    }
}

impl Hash for RGBA32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let rgba = self.normalized();
        rgba.r.to_bits().hash(state);
        rgba.g.to_bits().hash(state);
        rgba.b.to_bits().hash(state);
        rgba.a.to_bits().hash(state);
    }
}
