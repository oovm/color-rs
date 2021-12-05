use syn::{
    parse::{Parse, ParseStream},
    LitStr,
};

use color_parser::rgba;

use crate::RGBA32;

pub struct Color {
    pub rgba: RGBA32,
}

impl Parse for Color {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let query: LitStr = input.parse()?;
        let input = query.value();

        match rgba(input.trim()) {
            Ok(o) => Ok(Color { rgba: o.1 }),
            Err(e) => Err(syn::Error::new(query.span(), format!("{:?}", e))),
        }
    }
}
