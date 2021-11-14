use const_css_color::RGBA32;
use nom::IResult;
use NumberOrPercentage::*;

enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}

pub fn parse_number_or_percentage(input: &str) -> IResult<&str, NumberOrPercentage> {
    let (input, number) = parse_number(input)?;
    let (input, _) = parse_percentage(input)?;
    Ok((input, Percentage(number)))
}

pub fn parse_number(input: &str) -> IResult<&str, f32> {
    let (input, _) = parse_number();
    let (input, number) = parse_number_impl(input)?;
    let (input, _) = parse_whitespace(input)?;
    Ok((input, number))
}

pub fn parse_percent(input: &str) -> IResult<&str, f32> {
    let (input, _) = parse_whitespace(input)?;
    let (input, number) = parse_percent_impl(input)?;
    let (input, _) = parse_whitespace(input)?;
    Ok((input, number))
}

// rgb() = rgb( <percentage>{3} [ / <alpha-value> ]? )
//         rgb( <number>{3} [ / <alpha-value> ]? )
//         rgb( <percentage>#{3} [ , <alpha-value> ]? )
//         rgb( <number>#{3} [ , <alpha-value> ]? )
pub fn parse_rgb() -> IResult<RGBA32, &str> {}
