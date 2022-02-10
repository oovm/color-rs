// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]
//
// pub use color_core::*;
// pub use color_macro::*;

mod palette;
mod blenders;

use std::collections::BTreeMap;
use color_core::{RGBA, RGBA32};
use crate::palette::{ColorSpan, Palette};

mod traits;

pub use crate::traits::ColorGradient;
pub use crate::blenders::linear::LinearGradient;
pub use crate::blenders::scatter::ScatterGradient;