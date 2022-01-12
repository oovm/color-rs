use color_span::TextColorView;
use std::fmt::Write;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn text() {
    let mut text = TextColorView::new("public static class G {}");
    text.dye(0, 6, "keyword").ok();
    text.dye(7, 13, "keyword").ok();
    let mut html = String::new();
    for a in text.into_iter() {
        a.write_html(&mut html).unwrap();
        html.write_char('\n').unwrap();
    }
    println!("{}", html)
}
