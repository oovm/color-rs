use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as c, space0, space1},
    combinator::opt,
    error::ErrorKind,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{alpha_value, float_value, nom_error, Rgba};

/// `<rgb> = (rgb|rgba) ((<percentage>|<number>)#{3},<alpha-value>?)`
///
/// ```bnf
/// <rgb> = rgb( [<percentage> | none]{3} [ / [<alpha-value> | none] ]? )
///       | rgb( [<number> | none]{3} [ / [<alpha-value> | none] ]? )
/// ```
/// https://www.w3.org/TR/css-color-4/#rgb-functions
pub fn rgba(input: &str) -> IResult<&str, Rgba> {
    let (rest, _) = tuple((tag("rgb"), opt(c('a'))))(input)?;
    let (rest, inner) = delimited(c('('), tuple((space0, parse_rgb_inner, space0)), c(')'))(rest)?;
    Ok((rest, inner.1))
}

fn parse_rgb_inner(input: &str) -> IResult<&str, Rgba> {
    let (rest, inner) = tuple((float_value, split_any, float_value, split_any, float_value, maybe_alpha))(input)?;
    let (splits, colors) = match inner.5 {
        Some((c, a)) => ((inner.1, inner.3, Some(c)), (inner.0, inner.2, inner.4, Some(a))),
        None => ((inner.1, inner.3, None), (inner.0, inner.2, inner.4, None)),
    };
    if cfg!(strict) {
        match splits {
            (',', ',', None) => true,
            (',', ',', Some(',')) => true,
            _ => nom_error(ErrorKind::Char, "invalid pattern")?,
        };
    }
    Ok((rest, Rgba { r: colors.0, g: colors.1, b: colors.2, a: colors.3.unwrap_or(1.0) }))
}

fn maybe_alpha(input: &str) -> IResult<&str, Option<(char, f32)>> {
    let (rest, inner) = opt(tuple((split_any, alpha_value)))(input)?;
    Ok((rest, inner))
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
    let (rest, _) = space1(input)?;
    Ok((rest, ' '))
}
