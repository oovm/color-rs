use color_span::TextColorView;
use serde_json::to_string;
use std::fmt::Write;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn text() {
    let mut view = TextColorView::new("public static class G {}");
    view.dye(0, 6, "keyword").ok();
    view.dye(7, 13, "keyword").ok();
    println!("{}", to_string(&view).unwrap())
}
