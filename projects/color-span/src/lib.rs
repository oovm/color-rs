#![feature(type_name_of_val)]
#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![allow(clippy::derivable_impls)]
#![doc = include_str!("../readme.md")]

mod errors;
mod palette;
mod view;
mod writer;

pub use self::{
    errors::ColorSpanError,
    palette::ClassPalette,
    view::{Colored, TextView},
    writer::html::HtmlWriter,
};
