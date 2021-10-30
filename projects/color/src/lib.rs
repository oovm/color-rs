#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![doc = include_str!("../readme.md")]

pub(crate) use self::utils::*;
pub use self::{
    hsl::HSLA32,
    luma::LUMA32,
    rgb::{RGB, RGBA, RGBA32},
};

mod hsl;
mod luma;
mod rgb;
mod utils;
