use color_core::{RGB, RGBA};
use color_macro::{rgb, rgba};

#[test]
fn test_rgb() {
    assert_eq!(rgb!("#334D66"), RGB::new(51, 76, 102));
    assert_eq!(rgb!("rgb(51, 77, 102)"), RGB::new(51, 76, 102));
    assert_eq!(rgb!("rgb(20% 30% 40%)"), RGB::new(51, 76, 102));
}

#[test]
fn test_rgba() {
    // assert_eq!(rgba!("#334D6677"), RGBA::new(51, 76, 102));
    assert_eq!(rgba!("rgba(51, 77, 102, .5)"), RGBA::new(51, 77, 102, 127));
    assert_eq!(rgba!("rgba(20% 30% 40% 50%)"), RGBA::new(51, 77, 102, 127));
}
