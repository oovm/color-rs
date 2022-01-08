#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![allow(clippy::derivable_impls)]
#![doc = include_str!("../readme.md")]

mod errors;
mod span;
mod view;

pub use self::{
    errors::ColorSpanError,
    span::ColorSpan,
    view::{CharacterColor, TextColorView},
};
