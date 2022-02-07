// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]
//
// pub use color_core::*;
// pub use color_macro::*;

mod linear;
mod palette;

use std::collections::BTreeMap;
use color_core::{RGBA};

pub trait ColorGradient {
    /// Get the color at the given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the color in the gradient.
    ///
    /// # Returns
    ///
    /// The color at the given position.
    fn get_color(&self, position: f32) -> RGBA;
    fn get_scatter(&self, position: f32) -> f32;
}
