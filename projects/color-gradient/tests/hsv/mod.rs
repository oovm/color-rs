use super::*;
use colormap::GradientSampler;
use image::ImageResult;

#[test]
fn test_sample() {
    let sample = GradientSampler { points: 10, margin_left: 2, margin_right: 2, margin_top: 2, margin_bottom: 2 };
    sample.sample_as_hsv(tests("hsv/colormap_parula_update17a.png")).unwrap()
}

pub fn export_hsv_step(hsv: HsvGradient, path: &str) -> ImageResult<()> {
    let mut img = ImageBuffer::new(1024, 128);
    for (x, _, pixel) in img.enumerate_pixels_mut() {
        let hsva = hsv.get_step(x as f32);
        let RGBA8 { r, g, b, a } = hsva.into();
        *pixel = Rgba([r, g, b, a]);
    }
    img.save(assets(path))?;
    Ok(())
}

pub fn export_hsv_linear(hsv: HsvGradient, path: &str) -> ImageResult<()> {
    let mut img = ImageBuffer::new(1024, 128);
    for (x, _, pixel) in img.enumerate_pixels_mut() {
        let hsva = hsv.get_linear(x as f32);
        let RGBA8 { r, g, b, a } = hsva.into();
        *pixel = Rgba([r, g, b, a]);
    }
    img.save(assets(path))?;
    Ok(())
}

#[test]
fn test() {
    println!("{:#?}", HsvGradient::parula(0.0, 1024.0));
    export_hsv_step(HsvGradient::standard(0.0, 1024.0), "hsv/standard-step.png").unwrap();
    export_hsv_linear(HsvGradient::standard(0.0, 1024.0), "hsv/standard-linear.png").unwrap();
    export_hsv_step(HsvGradient::parula(0.0, 1024.0), "hsv/parula-step.png").unwrap();
    export_hsv_linear(HsvGradient::parula(0.0, 1024.0), "hsv/parula-linear.png").unwrap();
}
