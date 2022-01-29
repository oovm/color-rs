#[test]
fn ready() {
    println!("it works!")
}

// #[test]
// pub fn test_deserialize() {
//     let mut view = ClassPalette::new("public static class G {}");
//     view.dye(0, 6, "keyword").ok();
//     view.dye(7, 13, "keyword").ok();
//     // assert_eq!(serde_json::to_string(&view).unwrap(), include_str!("keyword.json"));
//     assert_eq!(view, from_str(include_str!("keyword.json")).unwrap())
// }

// #[test]
// pub fn test_html() {
//     let mut html = HtmlWriter::default();
//     html.style = Some(ONE_DARK.to_string());
//     let mut out = "".to_string();
//     let mut view = ClassPalette::new("public static class G {}");
//     view.dye(0, 6, "keyword").ok();
//     view.dye(7, 13, "keyword").ok();
//     html.write_fmt(&mut out, &view).unwrap();
//     let mut file = File::create("tests/keyword.html").unwrap();
//     file.write_all(out.as_bytes()).unwrap()
// }
