use color_core::{HSVA32, RGBA32, RGBA8};

#[test]
fn from_rgb() {
    let rgba = RGBA32::from(RGBA8::new(17, 34, 51, 255));
    let hsva = HSVA32::from(rgba);
    assert_eq!(hsva, HSVA32 { h: 210.0, s: 66.666664, v: 20.0, a: 100.0 });
    let rgba = RGBA32::from(RGBA8::new(69, 96, 144, 0));
    let hsva = HSVA32::from(rgba);
    assert_eq!(hsva, HSVA32 { h: 218.40001, s: 52.083332, v: 56.470592, a: 0.0 });
}

#[test]
fn into_rgb() {
    let hsva = HSVA32::new(30.0, 60.0, 90.0, 100.0);
    let rgba = RGBA8::from(hsva);
    assert_eq!(rgba, RGBA8 { r: 229, g: 160, b: 91, a: 255 });
    let hsva = HSVA32::new(15.0, 30.0, 60.0, 0.0);
    let rgba = RGBA8::from(hsva);
    assert_eq!(rgba, RGBA8 { r: 153, g: 118, b: 107, a: 0 });
}
