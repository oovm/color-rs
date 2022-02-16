#![warn(missing_docs)]
#![warn(missing_crate_level_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_doc_code_examples)]
#![doc = include_str!("../readme.md")]

pub use self::{
    hsl::HSLA32,
    hsv::{HSVAColor, HSV32, HSVA32},
    luma::LUMA32,
    rgb::{RGB8, RGBA32, RGBA8},
};

mod color_space;
mod hsl;
mod hsv;
mod luma;
mod rgb;
mod utils;
