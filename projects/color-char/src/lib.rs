#![forbid(missing_docs)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_doc_code_examples)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/color-rs/dev/.github/assets/rainbow.png")]

pub use self::character::Character;

mod character;
