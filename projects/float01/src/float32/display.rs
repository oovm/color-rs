use std::fmt::{Debug, Display, Formatter};
use super::*;


impl Display for f01 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
         Display::fmt(&self.wrapped, f)
    }
}

impl Debug for f01 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.wrapped, f)
    }
}