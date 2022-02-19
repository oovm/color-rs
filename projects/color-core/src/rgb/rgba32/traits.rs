use super::*;

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
