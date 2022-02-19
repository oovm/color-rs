use color_core::RGBA32;
use color_gradient::{ColorGradient, ColorMap, Palette, QuadraticGradient};
use image::{ImageBuffer, RgbaImage};

#[test]
fn ready() {
    println!("it works!")
}

pub fn render_bar<G: ColorGradient, M: ColorMap>(width: usize, gradient: G, palette: M) -> RgbaImage {
    let mut img = ImageBuffer::new(width as u32, 100);
    for (x, _, pixel) in img.enumerate_pixels_mut() {
        let color = gradient.get_color(&palette, x as f32);
        let r = color.r * 255.0;
        let g = color.g * 255.0;
        let b = color.b * 255.0;
        *pixel = image::Rgba::from([r as u8, g as u8, b as u8, 255])
    }
    img
}

#[test]
fn render_image_bar() {
    let linear = QuadraticGradient::default();
    let mut p = Palette::new(RGBA32::as_rgb(0.0, 1.0, 1.0), RGBA32::as_rgb(1.0, 1.0, 0.0));
    p.add(0.5, RGBA32::as_rgb(1.0, 0.0, 1.0));
    let img = render_bar(1000, linear, p);
    img.save("linear.png").unwrap();
}

#[test]
fn render_gamma_bar() {
    let linear = QuadraticGradient::default();
    let mut p = Palette::new(RGBA32::as_rgb(0.0, 1.0, 1.0), RGBA32::as_rgb(1.0, 1.0, 0.0));
    p.add(0.5, RGBA32::as_rgb(1.0, 0.0, 1.0));
    let img = render_bar(1000, linear, p);
    img.save("linear.png").unwrap();
}
