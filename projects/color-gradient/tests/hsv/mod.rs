use super::*;
use colormap::GradientSampler;
use image::ImageResult;

#[test]
fn test_sample() {
    let mut sample = GradientSampler::new(10).with_margin(2);
    sample.sample_file(tests("hsv/colormap_parula_update17a.png"), "Parula").unwrap();
    sample.sample_file(tests("hsv/colormap_jet.png"), "Jet").unwrap();
    sample.sample_file(tests("hsv/colormap_turbo.png"), "Turbo").unwrap();
    sample.sample_file(tests("hsv/colormap_hot.png"), "Hot").unwrap();
    sample.sample_file(tests("hsv/colormap_cool.png"), "Cool").unwrap();
    sample.sample_file(tests("hsv/colormap_spring.png"), "Spring").unwrap();
    sample.sample_file(tests("hsv/colormap_summer.png"), "Summer").unwrap();
    sample.sample_file(tests("hsv/colormap_autumn.png"), "Autumn").unwrap();
    sample.sample_file(tests("hsv/colormap_winter.png"), "Winter").unwrap();
    sample.export_hsv(tests("hsv/colormap.rs")).unwrap();
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
fn test_matlab() {
    println!("{:#?}", HsvGradient::parula(0.0, 1024.0));
    export_hsv_step(HsvGradient::standard(0.0, 1024.0), "hsv/standard-step.png").unwrap();
    export_hsv_linear(HsvGradient::standard(0.0, 1024.0), "hsv/standard-linear.png").unwrap();
    export_hsv_step(HsvGradient::parula(0.0, 1024.0), "hsv/parula-step.png").unwrap();
    export_hsv_linear(HsvGradient::parula(0.0, 1024.0), "hsv/parula-linear.png").unwrap();
    export_hsv_step(HsvGradient::jet(0.0, 1024.0), "hsv/jet-step.png").unwrap();
    export_hsv_linear(HsvGradient::jet(0.0, 1024.0), "hsv/jet-linear.png").unwrap();
    export_hsv_step(HsvGradient::turbo(0.0, 1024.0), "hsv/turbo-step.png").unwrap();
    export_hsv_linear(HsvGradient::turbo(0.0, 1024.0), "hsv/turbo-linear.png").unwrap();
    export_hsv_step(HsvGradient::hot(0.0, 1024.0), "hsv/hot-step.png").unwrap();
    export_hsv_linear(HsvGradient::hot(0.0, 1024.0), "hsv/hot-linear.png").unwrap();
    export_hsv_step(HsvGradient::cool(0.0, 1024.0), "hsv/cool-step.png").unwrap();
    export_hsv_linear(HsvGradient::cool(0.0, 1024.0), "hsv/cool-linear.png").unwrap();
    export_hsv_step(HsvGradient::spring(0.0, 1024.0), "hsv/spring-step.png").unwrap();
    export_hsv_linear(HsvGradient::spring(0.0, 1024.0), "hsv/spring-linear.png").unwrap();
    export_hsv_step(HsvGradient::summer(0.0, 1024.0), "hsv/summer-step.png").unwrap();
    export_hsv_linear(HsvGradient::summer(0.0, 1024.0), "hsv/summer-linear.png").unwrap();
    export_hsv_step(HsvGradient::autumn(0.0, 1024.0), "hsv/autumn-step.png").unwrap();
    export_hsv_linear(HsvGradient::autumn(0.0, 1024.0), "hsv/autumn-linear.png").unwrap();
    export_hsv_step(HsvGradient::winter(0.0, 1024.0), "hsv/winter-step.png").unwrap();
    export_hsv_linear(HsvGradient::winter(0.0, 1024.0), "hsv/winter-linear.png").unwrap();
}
