use syn::{
    parse::{Parse, ParseStream},
    Error, LitStr,
};

use color_parser::{hex_color, rgba};

use crate::RGBA32;

pub struct Color {
    pub rgba: RGBA32,
}

impl Parse for Color {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let query: LitStr = input.parse()?;
        let string = query.value();
        if string.trim().starts_with('#') {
            return match hex_color(string.trim()) {
                Ok(o) => Ok(Color { rgba: o.1 }),
                Err(e) => Err(Error::new(query.span(), e.map(|e| e.input))),
            };
        }
        if string.trim().starts_with("rgb") {
            return match rgba(string.trim()) {
                Ok(o) => Ok(Color { rgba: o.1 }),
                Err(e) => Err(Error::new(query.span(), format!("{:?}", e))),
            };
        }
        Err(Error::new(query.span(), "Unknown color pattern".to_string()))
    }
}
