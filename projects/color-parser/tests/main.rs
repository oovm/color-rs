use color_parser::hex_color;
use const_css_color::RGBA32;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_hex() {
    assert_eq!(hex_color("#2F14DF").unwrap().1, RGBA32::rgb(47, 20, 223));
}
