use color_core::{HSLA32, RGBA32};

#[test]
fn from_rgb() {
    let hsla: HSLA32 = RGBA32::new(0.2, 0.3, 0.4, 0.5).into();
    assert_eq!(hsla, HSLA32 { h: 0.5833333, s: 0.3333333, l: 0.3, a: 0.5 });
}
