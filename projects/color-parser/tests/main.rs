use color_parser::{hex_color, rgba, RGBA, RGBA32};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_hex() {
    assert_hex("#2F14DF", RGBA32 { r: 0.18431373, g: 0.078431375, b: 0.8745098, a: 1.0 });
}

#[test]
fn test_rgba() {
    assert_rgb("rgb(100%,100%,100%/100%)", RGBA32 { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
}

fn assert_hex(input: &str, expected: RGBA32) {
    assert_eq!(hex_color(input).unwrap().1, expected);
}

fn assert_rgb(input: &str, expected: RGBA32) {
    assert_eq!(rgba(input).unwrap().1, expected);
}
