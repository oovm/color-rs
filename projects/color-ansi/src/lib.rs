// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]

mod ansi;
mod writer;

pub use crate::ansi::{AnsiColor, AnsiStyle};
