use syn::{
    parse::{Parse, ParseStream},
    Error, LitStr, Result,
};

use color_parser::{hex_color, rgba, Finish};

use crate::RGBA32;

pub struct Color {
    pub rgba: RGBA32,
}

impl Parse for Color {
    fn parse(input: ParseStream) -> Result<Self> {
        let query: LitStr = input.parse()?;
        let string = query.value();
        let span = query.span();

        if string.trim().starts_with('#') {
            return match hex_color(string.trim()).finish() {
                Ok((_, rgba)) => Ok(Color { rgba }),
                _ => Err(Error::new(span, "Invalid hex pattern, can take 3,4,6,8 hex number only")),
            };
        }
        if string.trim().starts_with("rgb") {
            return match rgba(string.trim()).finish() {
                Ok((_, rgba)) => Ok(Color { rgba }),
                Err(e) => Err(Error::new(span, format!("{:?}", e))),
            };
        }
        Err(Error::new(span, "Unknown color pattern".to_string()))
    }
}
