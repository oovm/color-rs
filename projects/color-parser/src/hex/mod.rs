use crate::RGBA32;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

pub fn parse_hex(input: &str) -> IResult<&str, RGBA32> {
    let (rest, hex) = take_while_m_n(3, 8, |c| c.is_digit(16))(input)?;
    let hex = hex.as_bytes();
    let out = match hex.len() {
        8 => RGBA32::rgba(
            hexdigit(hex[0])? * 16 + hexdigit(hex[1])?,
            hexdigit(hex[2])? * 16 + hexdigit(hex[3])?,
            hexdigit(hex[4])? * 16 + hexdigit(hex[5])?,
            hexdigit(hex[6])? * 16 + hexdigit(hex[7])?,
        ),
        6 => RGBA32::rgb(
            hexdigit(hex[0])? * 16 + hexdigit(hex[1])?,
            hexdigit(hex[2])? * 16 + hexdigit(hex[3])?,
            hexdigit(hex[4])? * 16 + hexdigit(hex[5])?,
        ),
        4 => RGBA32::rgba(hexdigit(hex[0])? * 17, hexdigit(hex[1])? * 17, hexdigit(hex[2])? * 17, hexdigit(hex[3])? * 17),
        3 => RGBA32::rgb(hexdigit(hex[0])? * 17, hexdigit(hex[1])? * 17, hexdigit(hex[2])? * 17),
        _ => Err("Invalid hex number, can take 3,4,6,8 only")?,
    };
    Ok((rest, out))
}

fn hexdigit(c: u8) -> Result<u8, &str> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => Err("Not a hex digit"),
    }
}
