use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as c, space0, space1},
    combinator::opt,
    sequence::{delimited, tuple},
    IResult,
};

use const_css_color::RGBA32;

use crate::{alpha_value, float_value};

/// `<rgb> = (rgb|rgba) ((<percentage>|<number>)#{3},<alpha-value>?)`
///
/// ```bnf
/// <rgb> = rgb( [<percentage> | none]{3} [ / [<alpha-value> | none] ]? )
///       | rgb( [<number> | none]{3} [ / [<alpha-value> | none] ]? )
/// ```
/// https://www.w3.org/TR/css-color-4/#rgb-functions
pub fn rgba(input: &str) -> IResult<&str, RGBA32> {
    let (rest, _) = tuple((tag("rgb"), opt(c('a'))))(input)?;
    let (rest, inner) = delimited(c('('), tuple((space0, parse_rgb_inner, space0)), c(')'))(rest)?;
    Ok((rest, inner.1))
}

fn parse_rgb_inner(input: &str) -> IResult<&str, RGBA32> {
    let (rest, inner) = tuple((float_value, split_any, float_value, split_any, float_value, maybe_alpha))(input)?;
    // todo: extra check
    // , , ,
    // ' ' /
    Ok((rest, rgb_maybe(inner.0, inner.2, inner.4, inner.5)))
}

fn rgb_maybe(r: f32, g: f32, b: f32, a: Option<f32>) -> RGBA32 {
    RGBA32 { r, g, b, a: a.unwrap_or(1.0) }
}

fn maybe_alpha(input: &str) -> IResult<&str, Option<f32>> {
    let (rest, inner) = opt(tuple((split_any, alpha_value)))(input)?;
    Ok((rest, inner.map(|c| c.1)))
}

fn split_any(input: &str) -> IResult<&str, char> {
    alt((split_space, split_comma, split_slash))(input)
}

fn split_comma(input: &str) -> IResult<&str, char> {
    let (rest, _) = tuple((space0, c(','), space0))(input)?;
    Ok((rest, ','))
}

fn split_slash(input: &str) -> IResult<&str, char> {
    let (rest, _) = tuple((space0, c('/'), space0))(input)?;
    Ok((rest, '/'))
}

fn split_space(input: &str) -> IResult<&str, char> {
    let (rest, inner) = space1(input)?;
    Ok((rest, ' '))
}
