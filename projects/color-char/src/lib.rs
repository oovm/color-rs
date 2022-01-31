#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![allow(clippy::derivable_impls)]
#![doc = include_str ! ("../readme.md")]
// #![doc(html_logo_url = "https://optimisticpeach.github.io/Logo.svg")]

pub use self::character::Character;

mod character;

