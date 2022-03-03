use syn::{
    parse::{Parse, ParseStream},
    Error, LitFloat, LitInt, LitStr, Result, Token,
    __private::Span,
};

use color_parser::hex;

mod rgba;
use crate::RGBA32;

pub struct RgbaColor {
    pub rgba32: RGBA32,
}

pub fn next_number(input: ParseStream, transform: f32) -> Result<f32> {
    if input.peek(LitInt) {
        let string: LitInt = input.parse()?;
        Ok(string.base10_parse::<f32>()? / transform)
    }
    else if input.peek(LitFloat) {
        let string: LitFloat = input.parse()?;
        Ok(string.base10_parse::<f32>()?)
    }
    else if input.peek(LitStr) {
        let string: LitStr = input.parse()?;
        Ok(0.0)
    }
    else {
        Err(Error::new(input.span(), "Require of {integer} or {float}".to_string()))
    }
}

pub fn parse_hex(input: &str, span: Span) -> Result<RgbaColor> {
    match hex(input) {
        Ok(rgba) => Ok(RgbaColor { rgba32: rgba.into() }),
        _ => Err(Error::new(span, "Invalid hex pattern, can take 3,4,6,8 hex number only")),
    }
}

pub fn parse_rgba(input: &str, span: Span) -> Result<RgbaColor> {
    match hex(input) {
        Ok(rgba) => Ok(RgbaColor { rgba32: rgba.into() }),
        _ => Err(Error::new(span, "Invalid hex pattern, can take 3,4,6,8 hex number only")),
    }
}
