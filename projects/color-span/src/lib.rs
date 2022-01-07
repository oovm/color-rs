#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![allow(clippy::derivable_impls)]
#![doc = include_str!("../readme.md")]

mod view;
mod errors;
mod span;

pub use self::view::{TextColorView, CharacterColor, ColorSpan};
pub use self::errors::ColorSpanError;