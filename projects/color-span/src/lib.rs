#![forbid(missing_docs)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_doc_code_examples)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/color-rs/dev/.github/assets/rainbow.png")]

mod errors;
mod palette;
mod view;
mod writer;

pub use self::{
    errors::ColorSpanError,
    palette::ClassPalette,
    view::{Colored, TextView},
    writer::html::{self, HtmlWriter},
};
