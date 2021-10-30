// #![feature(const_for)]
// #![feature(const_mut_refs)]
// #![forbid(missing_docs)]
// #![forbid(missing_crate_level_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(missing_doc_code_examples)]
// #![doc = include_str!("../readme.md")]

extern crate proc_macro;
mod rgb;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    Expr, LitStr, Token,
};

/// a proc macro takes tokens as argument, and generates tokens
#[proc_macro]
pub fn rgb(input: TokenStream) -> TokenStream {
    let ParsedQuery { query, argument } = syn::parse(input).unwrap();

    // the parser extracted the format string argument and its type,
    // so we can generate code using the right type, and the compiler
    // will check it
    let quote_argument = match argument {
        ParsedArgument::Number(e) => quote! {
            parser::Argument::Number(#e)
        },
        ParsedArgument::String(e) => quote! {
            parser::Argument::String(#e)
        },
    };

    // we can then generate code using what we parsed. That
    // code will replace the macro call
    let gen = quote! {
        color::RGB {
            fragment: #query,
            argument: #quote_argument,
        }
    };
    gen.into()
}

pub fn rgba(input: TokenStream) -> TokenStream {
    todo!()
}

pub fn rgba32(input: TokenStream) -> TokenStream {
    todo!()
}
