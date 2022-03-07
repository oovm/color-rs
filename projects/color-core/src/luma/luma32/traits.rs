use super::*;

impl Debug for LUMA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "luma({}, {}, {}, {})", self.l, self.u, self.m, self.a)
    }
}
