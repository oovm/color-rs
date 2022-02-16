use color_core::{RGB8, RGBA8};
use color_macro::{rgb, rgba};

#[test]
fn test_rgb() {
    assert_eq!(rgb!("#346"), RGB8::new(51, 68, 102));
    assert_eq!(rgb!("#334C66"), RGB8::new(51, 76, 102));

    assert_eq!(rgb!("rgb(51, 76, 102)"), RGB8::new(51, 76, 102));
    assert_eq!(rgb!("rgb(20% 30% 40%)"), RGB8::new(51, 76, 102));

    assert_eq!(rgb!(51, 76, 102), RGB8::new(51, 76, 102));
    assert_eq!(rgb!(0.2, 0.3, 0.4), RGB8::new(51, 76, 102));
}

#[test]
fn test_rgba() {
    assert_eq!(rgba!("#3467"), RGBA8::new(51, 68, 102, 119));
    assert_eq!(rgba!("#334C667F"), RGBA8::new(51, 76, 102, 127));

    assert_eq!(rgba!("rgba(51, 76, 102, .5)"), RGBA8::new(51, 76, 102, 127));
    assert_eq!(rgba!("rgba(20% 30% 40% 50%)"), RGBA8::new(51, 76, 102, 127));

    assert_eq!(rgba!(51, 76, 102, 127), RGBA8::new(51, 76, 102, 127));
    assert_eq!(rgba!(0.2, 0.3, 0.4, 0.5), RGBA8::new(51, 76, 102, 127));
}
