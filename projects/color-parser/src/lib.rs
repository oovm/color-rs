// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]

pub use color_core::*;

pub use self::{
    color::rgba,
    hex::{hex, parse_hex},
    number::{alpha_value, angle_turn, float_value},
};
mod color;
mod hex;
mod named;
mod number;
mod utils;
