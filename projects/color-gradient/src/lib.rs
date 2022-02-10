// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]
//
// pub use color_core::*;
// pub use color_macro::*;

mod palettes;
mod blenders;


mod traits;
pub mod builtin;




pub use crate::traits::{ColorGradient, ColorMap};
pub use crate::blenders::linear::LinearGradient;
pub use crate::blenders::scatter::ScatterGradient;
pub use crate::blenders::gamma::GammaGradient;