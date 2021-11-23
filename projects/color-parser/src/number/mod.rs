use const_css_color::RGBA32;
use nom::{bytes::complete::tag, number::complete::float, IResult};
use NumberOrPercentage::*;

pub enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}

pub fn parse_number_or_percentage(input: &str) -> IResult<&str, NumberOrPercentage> {
    unimplemented!()
}

pub fn parse_number(input: &str) -> IResult<&str, f32> {
    unimplemented!()
}

pub fn parse_percent(input: &str) -> IResult<&str, f32> {
    let (rest, n) = float(input)?;
    let (rest, number) = tag("%")(rest)?;
    Ok((rest, n))
}

// rgb() = rgb( <percentage>{3} [ / <alpha-value> ]? )
//         rgb( <number>{3} [ / <alpha-value> ]? )
//         rgb( <percentage>#{3} [ , <alpha-value> ]? )
//         rgb( <number>#{3} [ , <alpha-value> ]? )
pub fn parse_rgb() -> IResult<RGBA32, &'static str> {
    todo!()
}
