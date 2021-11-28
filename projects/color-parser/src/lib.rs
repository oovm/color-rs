// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]

pub use const_css_color::*;

pub(crate) use self::utils::*;
pub use self::{
    color::rgba,
    hex::hex_color,
    number::{alpha_value, float_value},
};

mod color;
mod hex;
mod named;
mod number;
mod utils;
