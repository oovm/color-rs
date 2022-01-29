use std::{fs::File, io::Write};

use serde_json::from_str;

use color_span::{ClassPalette, html::ONE_DARK, HtmlWriter};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test_deserialize() {
    let mut view = ClassPalette::new("public static class G {}");
    view.dye(0, 6, "keyword").ok();
    view.dye(7, 13, "keyword").ok();
    // assert_eq!(serde_json::to_string(&view).unwrap(), include_str!("keyword.json"));
    assert_eq!(view, from_str(include_str!("keyword.json")).unwrap())
}

#[test]
pub fn test_html() {
    let mut html = HtmlWriter::default();
    html.style = Some(ONE_DARK.to_string());
    let mut out = "".to_string();
    let mut view = ClassPalette::new("public static class G {}");
    view.dye(0, 6, "keyword").ok();
    view.dye(7, 13, "keyword").ok();
    html.write_fmt(&mut out, &view).unwrap();
    let mut file = File::create("tests/keyword.html").unwrap();
    file.write_all(out.as_bytes()).unwrap()
}


#[test]
pub fn test_bits21() {
    assert_eq!('a' as u32, 0b000000000000000000000001100001);
    println!("{}", get_char(0b100000000000000000000001100001));
    println!("{:#032b}", get_color(0b_00000000_001_01111_11111111_11111111))
}

#[inline]
fn get_char(bits: u32) -> char {
    // Remove bits in front of char
    let erased = bits & 0b_00000000_00011111_11111111_11111111;
    unsafe {
        char::from_u32_unchecked(erased)
    }
}

/// Remove bits in front of char
fn get_color(bits: u32) -> u32 {
    let erased = (bits >> 21);
    erased
}