use super::*;
use std::fmt::{Debug, Display, Formatter};

impl Display for HSVA32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsva({:?}, {:?}%, {:?}%, {:?})", self.h, self.s, self.v, self.a)
    }
}
