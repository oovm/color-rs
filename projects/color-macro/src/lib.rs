#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![doc = include_str!("../readme.md")]

extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::parse;

use color_parser::{RGB8, RGBA32, RGBA8};

use self::parsing::RgbaColor;

mod parsing;

/// Compile time [`RGB8`] struct builder.
///
/// # Examples
///
/// ```rust, ignore
/// # use color_macro::rgb;
/// rgb!("#346"); // RGB::new(51, 68, 102)
/// rgb!("#334C66"); // RGB::new(51, 76, 102)
///
/// rgb!("rgb(51, 76, 102)"); // RGB::new(51, 76, 102)
/// rgb!("rgb(20% 30% 40%)"); // RGB::new(51, 76, 102)
///
/// rgb!(51, 76, 102); // RGB::new(51, 76, 102)
/// rgb!(0.2, 0.3, 0.4); // RGB::new(51, 76, 102)
/// ```
#[proc_macro]
pub fn rgb(input: TokenStream) -> TokenStream {
    let rgba: RgbaColor = match parse(input) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let RGB8 { r, g, b, a: _ } = rgba.rgba32.into();
    let gen = quote! {
        color::RGB8::new(#r, #g, #b)
    };
    gen.into()
}

/// Compile time [`RGBA8`] struct builder.
///
/// # Examples
///
/// ```rust, ignore
/// rgba!("#3467"); // RGBA::new(51, 76, 102, 127)
/// rgba!("#334C667F"); // RGBA::new(51, 76, 102, 127)
///
/// rgba!("rgb(51, 76, 102, .5)"); // RGBA::new(51, 76, 102, 127)
/// rgba!("rgb(20% 30% 40% 50%)"); // RGBA::new(51, 76, 102, 127)
///
/// rgba!(51, 76, 102, 127); // RGBA::new(51, 76, 102, 127)
/// rgba!(0.2, 0.3, 0.4, 0.5); // RGBA::new(51, 76, 102, 127)
/// ```
#[proc_macro]
pub fn rgba(input: TokenStream) -> TokenStream {
    let rgba: RgbaColor = match parse(input) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let RGBA8 { r, g, b, a } = rgba.rgba32.into();
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

/// Compile time [`RGBA32`] struct builder.
///
/// # Examples
///
/// ```rust, ignore
/// # use color_macro::rgba32;
/// rgba32!("#3467"); // RGBA32::rgba(51, 76, 102, 127)
/// rgba32!("#334C667F"); // RGBA32::rgba(51, 76, 102, 127)
///
/// rgba32!("rgb(51, 76, 102, .5)"); // RGBA32::rgba(51, 76, 102, 127)
/// rgba32!("rgb(20% 30% 40% 50%)"); // RGBA32::new(0.2, 0.3, 0.4, 0.5)
///
/// rgba32!(51, 76, 102, 127); // RGBA32::rgba(51, 76, 102, 127)
/// rgba32!(0.2, 0.3, 0.4, 0.5); // RGBA32::new(0.2, 0.3, 0.4, 0.5)
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
