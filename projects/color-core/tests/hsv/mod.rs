use color_core::{HSLA32, HSVA32, RGBA32, RGBA8};

#[test]
fn from_rgb() {
    let rgba = RGBA32::from(RGBA8::new(17, 34, 51, 255));
    let hsla = HSVA32::from(rgba);
    assert_eq!(hsla, HSVA32 { h: 210.0, s: 66.666664, v: 20.0, a: 100.0 });
    let rgba = RGBA32::from(RGBA8::new(69, 96, 144, 0));
    let hsla = HSVA32::from(rgba);
    assert_eq!(hsla, HSVA32 { h: 218.39998, s: 52.083332, v: 56.470592, a: 0.0 });
}
