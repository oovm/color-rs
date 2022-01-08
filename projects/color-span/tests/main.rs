use color_span::TextColorView;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn text() {
    let text = TextColorView::new("public static class G {}");
    println!("{:#?}", text)
    // CharacterColor::from(0x10FFFF);
}
