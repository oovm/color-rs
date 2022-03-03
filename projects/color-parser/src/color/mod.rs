use pex::{ParseResult, ParseState};

use crate::{alpha_value, float_value, RGBA32};

/// `<rgb> = (rgb|rgba) ((<percentage>|<number>)#{3},<alpha-value>?)`
///
/// ```bnf
/// <rgb> = rgb( [<percentage> | none]{3} [ / [<alpha-value> | none] ]? )
///       | rgb( [<number> | none]{3} [ / [<alpha-value> | none] ]? )
/// ```
/// https://www.w3.org/TR/css-color-4/#rgb-functions
pub fn rgba(input: ParseState) -> ParseResult<RGBA32> {
    todo!()
}

fn parse_rgb_inner(input: ParseState) -> ParseResult<RGBA32> {
    todo!()
}

fn maybe_alpha(input: ParseState) -> ParseResult<RGBA32> {
    todo!()
}

fn split_any(input: ParseState) -> ParseResult<RGBA32> {
    todo!()
}

fn split_comma(input: ParseState) -> ParseResult<RGBA32> {
    todo!()
}

fn split_slash(input: ParseState) -> ParseResult<RGBA32> {
    todo!()
}

fn split_space(input: ParseState) -> ParseResult<RGBA32> {
    todo!()
}
