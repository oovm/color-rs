use color_core::{HSLA32, RGBA32};

#[test]
fn from_hex() {
    // assert_eq!(format!("{:#X}", RGBA32::from(0xABCDEF)), "#ABCDEFFF");
    // assert_eq!(format!("{:#X}", RGBA32::from(0xABCDEF00)), "#ABCDEF00");
}

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
    let hsla = HSLA32::new(0.0, 0.25, 0.5, 0.75);
    assert_eq!(format!("{}", hsla), "hsla(0 25% 50% / 75%)");

    let rgba: RGBA32 = hsla.into();
    assert_eq!(format!("{}", rgba), "rgba(159 96 96 / 75%)");
    assert_eq!(format!("{:#}", rgba), "rgba(159.375 95.625 95.625 / 75%)");
    assert_eq!(format!("{:#X}", rgba), "#9F5F5FBF");
}
