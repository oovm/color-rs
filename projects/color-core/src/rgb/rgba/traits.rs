use super::*;

impl Eq for RGBA8 {}

impl PartialEq for RGBA8 {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
