use nom::{
    bytes::complete::{tag, take_while_m_n},
    error::ErrorKind,
    IResult,
};

use crate::{utils::nom_error, ErrorMessage, Rgba};

/// `<hex-color> = #<hex-value>{3,4,6,8}`
/// <https://www.w3.org/TR/css-color-4/#hex-notation>
pub fn hex_color(input: &str) -> IResult<&str, Rgba> {
    let (rest, _) = tag("#")(input)?;
    let (rest, hex) = take_while_m_n(3, 8, |c: char| c.is_digit(16))(rest)?;
    let hex = hex.as_bytes();
    let out = match hex.len() {
        8 => Rgba::rgba(
            hex_digit(hex[0])? * 16 + hex_digit(hex[1])?,
            hex_digit(hex[2])? * 16 + hex_digit(hex[3])?,
            hex_digit(hex[4])? * 16 + hex_digit(hex[5])?,
            hex_digit(hex[6])? * 16 + hex_digit(hex[7])?,
        ),
        6 => Rgba::rgb(
            hex_digit(hex[0])? * 16 + hex_digit(hex[1])?,
            hex_digit(hex[2])? * 16 + hex_digit(hex[3])?,
            hex_digit(hex[4])? * 16 + hex_digit(hex[5])?,
        ),
        4 => Rgba::rgba(hex_digit(hex[0])? * 17, hex_digit(hex[1])? * 17, hex_digit(hex[2])? * 17, hex_digit(hex[3])? * 17),
        3 => Rgba::rgb(hex_digit(hex[0])? * 17, hex_digit(hex[1])? * 17, hex_digit(hex[2])? * 17),
        _ => nom_error(ErrorKind::ManyMN, "Invalid hex pattern, can take 3,4,6,8 hex number only")?,
    };
    Ok((rest, out))
}

fn hex_digit(c: u8) -> Result<u8, ErrorMessage> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => nom_error(ErrorKind::HexDigit, "Invalid hex digit"),
    }
}
