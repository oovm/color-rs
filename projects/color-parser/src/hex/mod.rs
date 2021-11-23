use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    error::{Error, ErrorKind},
    sequence::tuple,
    Err, IResult,
};

use crate::RGBA32;

pub fn hex_color(input: &str) -> IResult<&str, RGBA32> {
    let (rest, _) = tag("#")(input)?;
    let (rest, hex) = take_while_m_n(3, 8, |c: char| c.is_digit(16))(rest)?;
    let hex = hex.as_bytes();
    let out = match hex.len() {
        8 => RGBA32::rgba(
            hex_digit(hex[0])? * 16 + hex_digit(hex[1])?,
            hex_digit(hex[2])? * 16 + hex_digit(hex[3])?,
            hex_digit(hex[4])? * 16 + hex_digit(hex[5])?,
            hex_digit(hex[6])? * 16 + hex_digit(hex[7])?,
        ),
        6 => RGBA32::rgb(
            hex_digit(hex[0])? * 16 + hex_digit(hex[1])?,
            hex_digit(hex[2])? * 16 + hex_digit(hex[3])?,
            hex_digit(hex[4])? * 16 + hex_digit(hex[5])?,
        ),
        4 => RGBA32::rgba(hex_digit(hex[0])? * 17, hex_digit(hex[1])? * 17, hex_digit(hex[2])? * 17, hex_digit(hex[3])? * 17),
        3 => RGBA32::rgb(hex_digit(hex[0])? * 17, hex_digit(hex[1])? * 17, hex_digit(hex[2])? * 17),
        _ => return Err(Err::Error(Error::new("Invalid hex pattern, can take 3,4,6,8 hex number only", ErrorKind::Digit))),
    };
    Ok((rest, out))
}

fn hex_digit(c: u8) -> Result<u8, Err<Error<&'static str>>> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => Err(Err::Error(Error::new("Invalid hex digit", ErrorKind::Digit))),
    }
}
