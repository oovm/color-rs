// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]
//
// pub use color_core::*;
// pub use color_macro::*;

mod blenders;
mod palettes;

pub mod builtin;
mod traits;
pub mod utils;

pub use crate::{
    blenders::{gamma::GammaGradient, linear::LinearGradient, scatter::ScatterGradient},
    palettes::discrete::Palette,
    traits::{ColorGradient, ColorMap},
};
