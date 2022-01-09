use color_span::TextColorView;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn text() {
    let mut text = TextColorView::new("public static class G {}");
    text.dye(0, 5, "keyword").ok();

    for a in text.into_iter() {
        println!("{:#?}", a)
    }
}
