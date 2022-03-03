use pex::{ParseResult, ParseState};

use NumberOrPercentage::*;

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
pub fn alpha_value(input: &str) -> ParseResult<f32> {
    todo!()
}

/// `<float> = <number> | <percentage>`
///
/// - `<number>`: between `0` and `255`
/// - `<percentage>`: between `0%` and `100%`
pub fn float_value(input: &str) -> ParseResult<f32> {
    todo!()
}

fn number_or_percentage(input: ParseState) -> ParseResult<NumberOrPercentage> {
    todo!()
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
pub fn angle_turn(input: ParseState) -> ParseResult<f32> {
    todo!()
}
