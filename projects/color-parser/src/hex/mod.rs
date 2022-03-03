use crate::RGBA32;
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
    let (state, _) = input.match_char('#')?;
    let (state, hex) = state.match_str_if(|c| c.is_ascii_hexdigit(), "ASCII_HEX")?;
    let rgba = match hex.as_bytes() {
        [gray] => RGBA8::gray(byte2_to_u8(*gray, *gray)),
        [gray1, gray2] => RGBA8::gray(byte2_to_u8(*gray1, *gray2)),
        [r, g, b] => RGBA8::new(byte2_to_u8(*r, *r), byte2_to_u8(*g, *g), byte2_to_u8(*b, *b), 255),
        [r, g, b, a] => RGBA8::new(byte2_to_u8(*r, *r), byte2_to_u8(*g, *g), byte2_to_u8(*b, *b), byte2_to_u8(*a, *a)),
        [r1, r2, g1, g2, b1, b2] => RGBA8::new(byte2_to_u8(*r1, *r2), byte2_to_u8(*g1, *g2), byte2_to_u8(*b1, *b2), 255),
        [r1, r2, g1, g2, b1, b2, a1, a2] =>
            RGBA8::new(byte2_to_u8(*r1, *r2), byte2_to_u8(*g1, *g2), byte2_to_u8(*b1, *b2), byte2_to_u8(*a1, *a2)),
        _ => StopBecause::must_be("3, 4, 6 or 8 hex digits", state.start_offset)?,
    };
    state.finish(rgba)
}

#[inline(always)]
fn byte2_to_u8(high: u8, low: u8) -> u8 {
    byte_to_u8(high) << 4 | byte_to_u8(low)
}

#[inline(always)]
fn byte_to_u8(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        b'A'..=b'F' => byte - b'A' + 10,
        _ => unreachable!(),
    }
}
