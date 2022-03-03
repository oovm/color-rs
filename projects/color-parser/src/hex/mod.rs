use crate::RGBA32;
use pex::{ParseResult, ParseState};

/// `<hex-color_parser> = #<hex-value>{3,4,6,8}`
/// <https://www.w3.org/TR/css-color-4/#hex-notation>
pub fn hex_color(input: ParseState) -> ParseResult<RGBA32> {
    todo!()
}
