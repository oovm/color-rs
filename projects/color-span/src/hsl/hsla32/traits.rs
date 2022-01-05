use super::*;

impl Default for HSLA32 {
    fn default() -> Self {
        Self { h: 0.0, s: 0.0, l: 0.0, a: 1.0 }
    }
}

impl Display for HSLA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let HSLA32 { h, s, l, a } = self.normalized();
        match f.alternate() {
            true => write!(f, "hsla({} {}% {}% / {}%)", h * 360.0, s * 100.0, l * 100.0, a * 100.0),
            false => write!(f, "hsla({:.0} {:.0}% {:.0}% / {:.0}%)", h * 360.0, s * 100.0, l * 100.0, a * 100.0),
        }
    }
}
