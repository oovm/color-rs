#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![doc = include_str!("../readme.md")]

extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::parse;

use color_parser::{RGB, RGBA, RGBA32};

use self::parsing::RgbaColor;

mod parsing;

///
///
/// # Arguments
///
/// * `input`:
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
///
/// ```
#[proc_macro]
pub fn rgb(input: TokenStream) -> TokenStream {
    let rgba: RgbaColor = match parse(input) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let RGB { r, g, b } = rgba.rgba32.into();
    let gen = quote! {
        color::RGB {
            r: #r,
            g: #g,
            b: #b,
        }
    };
    gen.into()
}


/// a proc macro takes tokens as argument, and generates tokens
///
/// # Arguments
///
/// * `input`:
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
///
/// ```
#[proc_macro]
pub fn rgba(input: TokenStream) -> TokenStream {
    let rgba: RgbaColor = match parse(input) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let RGBA { r, g, b, a } = rgba.rgba32.into();
    let gen = quote! {
        color::RGBA {
            r: #r,
            g: #g,
            b: #b,
            a: #a,
        }
    };
    gen.into()
}

/// a proc macro takes tokens as argument, and generates tokens
///
/// # Arguments
///
/// * `input`: literal
///
/// returns: TokenStream
///
/// # Examples
///
/// ```rust
/// rgba32!(0, 0, 0, 0);
/// rgba32!("#FF00FF");
/// ```
#[proc_macro]
pub fn rgba32(input: TokenStream) -> TokenStream {
    let rgba: RgbaColor = match parse(input) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let RGBA32 { r, g, b, a } = rgba.rgba32.into();
    let gen = quote! {
        color::RGBA32 {
            r: #r,
            g: #g,
            b: #b,
            a: #a,
        }
    };
    gen.into()
}
