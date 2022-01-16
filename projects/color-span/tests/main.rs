use color_span::TextColorView;
use serde_json::from_str;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test_deserialize() {
    let mut view = TextColorView::new("public static class G {}");
    view.dye(0, 6, "keyword").ok();
    view.dye(7, 13, "keyword").ok();
    assert_eq!(view, from_str(include_str!("keyword.json")).unwrap())
}
