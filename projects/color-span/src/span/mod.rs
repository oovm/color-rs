use std::rc::Rc;
use string_interner::StringInterner;

pub struct ColorSpan {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

pub struct TextView {
    intern: StringInterner,
    inner: Vec<TextColor>
}

pub struct TextColor {
    char: char,
    color: Rc<String>
}

#[test]
pub fn text() {
    use string_interner::StringInterner;

    let mut interner = StringInterner::default();
    let sym0 = interner.get_or_intern("Elephant");
    let sym1 = interner.get_or_intern("Tiger");
    let sym2 = interner.get_or_intern("Horse");
    let sym3 = interner.get_or_intern("Tiger");
    assert_ne!(sym0, sym1);
    assert_ne!(sym0, sym2);
    assert_ne!(sym1, sym2);
    assert_eq!(sym1, sym3); // same!
}