use pex::{
    helpers::{ascii_whitespace, make_from_str},
    ParseResult, ParseState, StopBecause,
};

use crate::RGBA32;

/// `<rgb> = (rgb|rgba) ((<percentage>|<number>)#{3},<alpha-value>?)`
///
/// ```bnf
/// <rgb> = rgb( [<percentage> | none]{3} [ / [<alpha-value> | none] ]? )
///       | rgb( [<number> | none]{3} [ / [<alpha-value> | none] ]? )
/// ```
/// https://www.w3.org/TR/css-color-4/#rgb-functions
pub fn rgba(input: &str) -> Result<RGBA32, StopBecause> {
    let state = ParseState::new(input.trim_end()).skip(ascii_whitespace);
    make_from_str(state, parse_rgba)
}

fn parse_rgba(input: ParseState) -> ParseResult<RGBA32> {
    todo!()
}

fn maybe_alpha(input: ParseState) -> ParseResult<RGBA32> {
    let (state, _) = input.match_char('/')?;
    let state = state.skip(ascii_whitespace);

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
