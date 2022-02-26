// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]
//
// pub use color_core::*;
// pub use color_macro::*;

mod blenders;
pub mod builtin;
mod interpolation;
mod traits;
pub mod utils;

#[cfg(feature = "image")]
pub use crate::utils::sampler::GradientSampler;
pub use crate::{
    blenders::{gamma::LinearGradient, hsv::HsvGradient, rgb::RgbGradient, scatter::ScatterGradient},
    interpolation::Interpolator,
    traits::{ColorGradient, ColorMap},
};
