use std::fmt::{Debug, Display, Formatter, UpperExp, UpperHex};
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

impl UpperHex for f01 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.wrapped.to_bits(), f)
    }
}

impl UpperExp for f01 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        UpperExp::fmt(&self.wrapped, f)
    }
}
