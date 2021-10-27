

use super::*;

impl Debug for HSLA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hsla = self.clone();
        hsla.normalize();
        write!(f, "hsla({:?}, {:?}%, {:?}%, {:?})", hsla.h, hsla.s, hsla.l, hsla.a)
    }
}

impl Debug for HSVA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hsva = self.clone();
        hsva.normalize();
        write!(f, "hsva({:?}, {:?}%, {:?}%, {:?})", hsva.h, hsva.s, hsva.v, hsva.a)
    }
}