use color_span::{ClassPalette, HtmlWriter};
use serde_json::from_str;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test_deserialize() {
    let mut view = ClassPalette::new("public static class G {}");
    view.dye(0, 6, "keyword").ok();
    view.dye(7, 13, "keyword").ok();
    // assert_eq!(to_string(&view).unwrap(), include_str!("keyword.json"));
    assert_eq!(view, from_str(include_str!("keyword.json")).unwrap())
}

#[test]
pub fn test_html() {
    let html = HtmlWriter::default();
    let mut out = String::new();
    let mut view = ClassPalette::new("public static class G {}");
    view.dye(0, 6, "keyword").ok();
    view.dye(7, 13, "keyword").ok();
    html.write_fmt(&mut out, &view).unwrap();
    assert_eq!(out, include_str!("keyword.html"))
}
