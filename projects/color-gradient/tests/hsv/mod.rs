use crate::assets;
use color_core::RGBA8;
use color_gradient::HsvGradient;
use image::{ImageBuffer, Rgba};

#[test]
fn test() {
    let hsv = HsvGradient::standard(0.0, 1000.0);
    let mut img = ImageBuffer::new(1000, 100);
    for (x, _, pixel) in img.enumerate_pixels_mut() {
        let hsva = hsv.get_step(x as f32);
        let RGBA8 { r, g, b, a } = hsva.into();
        *pixel = Rgba([r, g, b, a]);
    }
    img.save(assets("hsv/hsv-standard-step.png")).unwrap();
}

#[test]
fn test2() {
    let hsv = HsvGradient::standard(0.0, 1000.0);
    let mut img = ImageBuffer::new(1000, 100);
    for (x, _, pixel) in img.enumerate_pixels_mut() {
        let hsva = hsv.get_linear(x as f32);
        let RGBA8 { r, g, b, a } = hsva.into();
        *pixel = Rgba([r, g, b, a]);
    }
    img.save(assets("hsv/hsv-standard-linear.png")).unwrap();
}
