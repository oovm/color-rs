use color_parser::{hex_color, rgba};
use const_css_color::RGBA32;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_hex() {
    assert_hex("#2F14DF", RGBA32::rgb(47, 20, 223));
}

#[test]
fn test_rgba() {
    assert_rgb("rgb(100%,100%,100%/100%)", RGBA32::rgba(255, 255, 255, 255));
}

fn assert_hex(input: &str, expected: RGBA32) {
    assert_eq!(hex_color(input).unwrap().1, expected);
}

fn assert_rgb(input: &str, expected: RGBA32) {
    assert_eq!(rgba(input).unwrap().1, expected);
}
