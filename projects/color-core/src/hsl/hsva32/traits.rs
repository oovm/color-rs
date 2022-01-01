use super::*;

impl Debug for HSVA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hsva = self.normalized();
        write!(f, "hsva({:?}, {:?}%, {:?}%, {:?})", hsva.h, hsva.s, hsva.v, hsva.a)
    }
}
