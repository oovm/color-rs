use std::fmt::{Debug, Formatter};

use super::*;

impl Debug for HSLA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hsla = self;
        hsla.normalize();
        write!(f, "hsla({:?}, {:?}%, {:?}%, {:?})", hsla.h, hsla.s, hsla.l, hsla.a)
    }
}