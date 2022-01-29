use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
/// ColorSpanError
#[derive(Debug)]
pub enum ColorSpanError {
    /// ColorSpanError
    OutOfRange {
        /// ColorSpanError
        current: usize,
        /// ColorSpanError
        input: usize,
    },
    /// ColorSpanError
    TooMuchColors,
    ///
    NoSuchColor {
        ///
        color: String,
    },
}

impl Display for ColorSpanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorSpanError::OutOfRange { current, input } => {
                write!(f, "Length of the text is {current}, but you want to get {input}")
            },
            ColorSpanError::TooMuchColors => f.write_str("Too much colors, support 255 colors at most"),
            ColorSpanError::NoSuchColor { .. } => todo!(),
        }
    }
}

impl Error for ColorSpanError {}
