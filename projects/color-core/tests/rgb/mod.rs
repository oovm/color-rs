use color_core::{HSLA32, RGBA32};

#[test]
fn from_rgb() {
    let hsla = HSLA32::new(0.2, 0.3, 0.4, 0.5);

    let rgba: RGBA32 = hsla.into();
    assert_eq!(format!("{}", hsla), "hsla(72 30.000002% 40% / 50%)");
    assert_eq!(format!("{}", rgba), "rgba(120 133 71 / 50%)");
    assert_eq!(format!("{:#}", rgba), "rgba(120 133 71 / 50%)");
    assert_eq!(format!("{:X}", rgba), "rgba(0, 0, 0, 0.5)");
}
