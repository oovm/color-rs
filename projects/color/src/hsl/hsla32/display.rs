use super::*;

impl Debug for HSLA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hsla = self.normalized();
        write!(f, "hsla({:?}, {:?}%, {:?}%, {:?})", hsla.h, hsla.s, hsla.l, hsla.a)
    }
}
