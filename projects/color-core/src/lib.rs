#![warn(missing_docs)]
#![warn(missing_crate_level_docs)]
#![deny(missing_copy_implementations, missing_debug_implementations)]
#![warn(missing_doc_code_examples)]
#![doc = include_str!("../readme.md")]

pub use self::{
    cmyk::{CMYKColor, CMYK32},
    hsl::{HSLColor, HSLA32},
    hsv::{HSVColor, HSVA32},
    rgb::{RGBColor, RGB8, RGBA32, RGBA8},
};

mod cmyk;
mod color_space;
mod hsl;
mod hsv;
mod rgb;
mod utils;
