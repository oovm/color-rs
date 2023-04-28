#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![doc = include_str!("../readme.md")]

// pub use color_core::*;
// pub use color_macro::*;

mod blenders;
// pub mod builtin;
pub mod helpers;
mod interpolation;
mod traits;

pub use crate::{
    blenders::{hsv::HsvGradient, rgb::RgbGradient},
    interpolation::Interpolator,
    traits::{ColorGradient, ColorMap},
};
