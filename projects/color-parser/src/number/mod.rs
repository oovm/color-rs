use nom::{branch::alt, bytes::complete::tag, character::complete::char, combinator::opt, number::complete::float, IResult};
use std::f32::consts::PI;
use NumberOrPercentage::*;

use crate::clamp;

enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}

/// `<alpha> = <number> | <percentage>`
///
/// - `<number>`: between `0` and `1`
/// - `<percentage>`: between `0%` and `100%`
///
/// <https://www.w3.org/TR/css-color-4/#typedef-alpha-value>
pub fn alpha_value(input: &str) -> IResult<&str, f32> {
    let (rest, value) = number_or_percentage(input)?;
    let alpha = match value {
        Number(value) => value,
        Percentage(value) => value / 100.0,
    };
    Ok((rest, clamp(alpha)))
}

/// `<float> = <number> | <percentage>`
///
/// - `<number>`: between `0` and `255`
/// - `<percentage>`: between `0%` and `100%`
pub fn float_value(input: &str) -> IResult<&str, f32> {
    let (rest, value) = number_or_percentage(input)?;
    let alpha = match value {
        Number(value) => value / 255.0,
        Percentage(value) => value / 100.0,
    };
    Ok((rest, clamp(alpha)))
}

fn number_or_percentage(input: &str) -> IResult<&str, NumberOrPercentage> {
    let (rest, f) = float(input)?;
    let (rest, p) = opt(char('%'))(rest)?;
    let value = match p.is_some() {
        true => NumberOrPercentage::Percentage(f),
        false => NumberOrPercentage::Number(f),
    };
    Ok((rest, value))
}

/// `<angle> = <number> <angle-measure>`
///
/// ### `<angle-measure>`
/// - `deg`: Degrees, there are 360 degrees in a full circle.
/// - `grad` Gradians, also known as "gons" or "grades". There are 400 gradians in a full circle.
/// - `rad`: radians, there are 2π radians in a full circle.
/// - `turn`: Turns, there is 1 turn in a full circle.
///
/// <https://www.w3.org/TR/css-values-4/#angle-value>
pub fn angle_turn(input: &str) -> IResult<&str, f32> {
    let (rest, f) = float(input)?;
    let units = (tag("deg"), tag("deg"), tag("deg"), tag("deg"));
    let (rest, value) = opt(alt(units))(rest)?;
    let angle = match value.unwrap_or("") {
        "deg" => f,
        "grad" => f / 400.0,
        "rad" => f / (2.0 * PI),
        "turn" => f / 100.0,
        _ => f / 360.0,
    };
    Ok((rest, angle))
}
