// #![feature(const_for)]
// #![feature(const_mut_refs)]
// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]

extern crate proc_macro;
mod rgb;
use self::rgb::Color;
use color_parser::{Rgba, RGB, RGBA};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    Expr, LitStr, Token,
};

/// a proc macro takes tokens as argument, and generates tokens
#[proc_macro]
pub fn rgb(input: TokenStream) -> TokenStream {
    let rgba: Color = syn::parse(input).unwrap();
    let rgb: RGB = rgba.rgba.into();
    let gen = quote! {
        color_parser::RGB {
            r: #(rgb.r),
            g: #(rgb.g),
            b: #(rgb.b),
        }
    };
    gen.into()
}

#[proc_macro]
pub fn rgba(input: TokenStream) -> TokenStream {
    let rgba: Color = syn::parse(input).unwrap();
    let rgba: RGBA = rgba.rgba.into();
    let gen = quote! {
        color_parser::RGBA {
            r: #(rgba.r),
            g: #(rgba.g),
            b: #(rgba.b),
            a: #(rgba.a),
        }
    };
    gen.into()
}

#[proc_macro]
pub fn rgba32(input: TokenStream) -> TokenStream {
    let rgba: Color = syn::parse(input).unwrap();
    let rgba: Rgba = rgba.rgba.into();
    let gen = quote! {
        color_parser::RGBA32 {
            r: #(rgba.r),
            g: #(rgba.g),
            b: #(rgba.b),
            a: #(rgba.a),
        }
    };
    gen.into()
}
