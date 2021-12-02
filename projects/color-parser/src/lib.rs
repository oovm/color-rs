// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]

pub(crate) use self::utils::*;
pub use self::{
    color::rgba,
    hex::hex_color,
    number::{alpha_value, angle_turn, float_value},
};

mod color;
mod hex;
mod named;
mod number;
mod utils;

///
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub struct Rgba {
    /// The red channel of color in `[0.0f32, 1.0f32]`.
    pub r: f32,
    /// The green channel of color in `[0.0f32, 1.0f32]`.
    pub g: f32,
    /// The blue channel of color in `[0.0f32, 1.0f32]`.
    pub b: f32,
    /// The alpha channel of color in `[0.0f32, 1.0f32]`.
    pub a: f32,
}
