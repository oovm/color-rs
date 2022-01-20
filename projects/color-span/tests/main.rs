use color_span::{ColoredText, HTMLWriter};
use serde_json::from_str;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test_deserialize() {
    let mut view = ColoredText::new("public static class G {}");
    view.dye(0, 6, 0).ok();
    view.dye(7, 13, 0).ok();
    assert_eq!(view, from_str(include_str!("keyword.json")).unwrap())
}

#[test]
pub fn test_html() {
    let html = HTMLWriter::default();
    let mut out = String::new();
    let mut view = ColoredText::new("public static class G {}");
    view.dye(0, 6, 0).ok();
    view.dye(7, 13, 0).ok();
    html.write_fmt(&mut out, &view).unwrap();
    assert_eq!(out, include_str!("keyword.json"))
}
