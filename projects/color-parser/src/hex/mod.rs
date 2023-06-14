use color_core::RGBA8;
use pex::{
    helpers::{ascii_whitespace, make_from_str},
    ParseResult, ParseState, StopBecause,
};

pub fn hex(input: &str) -> Result<RGBA8, StopBecause> {
    let state = ParseState::new(input.trim_end()).skip(ascii_whitespace);
    make_from_str(state, parse_hex)
}

/// `<hex-color_parser> = #<hex-value>{3,4,6,8}`
/// <https://www.w3.org/TR/css-color-4/#hex-notation>
pub fn parse_hex(input: ParseState) -> ParseResult<RGBA8> {
    let (state, (r, g, b, a)) = pex::helpers::hex_color(input, "#")?;
    state.finish(RGBA8::new(r, g, b, a))
}
