// #![feature(const_for)]
// #![feature(const_mut_refs)]
// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]

extern crate proc_macro;

use proc_macro::TokenStream;

use color_parser::{RGB, RGBA, RGBA32};
use quote::quote;

use self::rgb::Color;

mod rgb;

/// a proc macro takes tokens as argument, and generates tokens
#[proc_macro]
pub fn rgb(input: TokenStream) -> TokenStream {
    let rgba: Color = syn::parse(input).unwrap();
    let RGB { r, g, b } = rgba.rgba.into();
    let gen = quote! {
        color::RGB {
            r: #r,
            g: #g,
            b: #b,
        }
    };
    gen.into()
}

#[proc_macro]
pub fn rgba(input: TokenStream) -> TokenStream {
    let rgba: Color = syn::parse(input).unwrap();
    let RGBA { r, g, b, a } = rgba.rgba.into();
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

#[proc_macro]
pub fn rgba32(input: TokenStream) -> TokenStream {
    let rgba: Color = syn::parse(input).unwrap();
    let RGBA32 { r, g, b, a } = rgba.rgba.into();
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
