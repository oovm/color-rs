use syn::{
    parse::{Parse, ParseStream},
    LitStr,
};

use color_parser::rgba;

use crate::Rgba;

pub struct Color {
    pub rgba: Rgba,
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
