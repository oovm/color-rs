use color_core::{HSLA32, RGBA32};

#[test]
fn from_rgb_1() {
    let hsla = HSLA32::new(0.2, 0.3, 0.4, 0.5);
    assert_eq!(format!("{}", hsla), "hsla(72 30% 40% / 50%)");

    let rgba: RGBA32 = hsla.into();
    assert_eq!(format!("{}", rgba), "rgba(120 133 71 / 50%)");
    assert_eq!(format!("{:#}", rgba), "rgba(120.36 132.59999 71.4 / 50%)");
    assert_eq!(format!("{:#X}", rgba), "#7884477F");
}

#[test]
fn from_rgb_2() {
    let hsla = HSLA32::new(0.0, 0.5, 0.5, 0.75);
    assert_eq!(format!("{}", hsla), "hsla(0 50% 50% / 75%)");

    let rgba: RGBA32 = hsla.into();
    assert_eq!(format!("{}", rgba), "rgba(191 64 64 / 75%)");
    assert_eq!(format!("{:#}", rgba), "rgba(191.25 63.75 63.75 / 75%)");
    assert_eq!(format!("{:#X}", rgba), "#BF3F3FBF");
}
