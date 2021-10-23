use std::fmt::{Debug, Formatter};

use super::*;

impl Debug for HSLA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsla({:?}, {:?}, {:?}, {:?})", self.h, self.s, self.l, self.a)
    }
}