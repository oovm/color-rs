use color_macro::rgb;
use color_parser::RGB;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() {
    assert_eq!(rgb!("rgb(20%, 30%, 40%, 50%)"), RGB::new(2, 3, 4));
}
