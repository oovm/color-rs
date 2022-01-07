use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum ColorSpanError {
    OutOfRange {
        current: usize,
        input: usize,
    },
    TooMuchColors,
}


impl Debug for ColorSpanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ColorSpanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ColorSpanError {

}