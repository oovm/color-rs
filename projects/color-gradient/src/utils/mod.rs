use color_core::{HSVA32, RGBA32};
use image::{GenericImageView, ImageResult, RgbaImage};
use std::{collections::BTreeMap, error::Error, path::Path};

pub struct GradientSampler {
    pub points: usize,
}

impl GradientSampler {
    pub fn sample(&self, image: &RgbaImage) -> BTreeMap<u32, RGBA32> {
        let mut output = BTreeMap::new();
        let width = image.width();
        let step = width as usize / self.points;
        for i in (0..width).step_by(step) {
            let color = image.get_pixel(i, 0);
            output.insert(i, color);
        }
        output
    }
    pub fn sample_as_hsv<P>(&self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let image_path = path.as_ref();
        let file_path = image_path.with_extension("rs");
        let file = std::fs::File::create(file_path)?;
        let image = image::open(path)?;
        let map = self.sample(image);
        writeln!(file, "pub fn standard(min: f32, max: f32) -> HsvGradient {{")?;
        writeln!(file, "    let mut grad = HsvGradient::new(0.0, 360.0);")?;
        for (x, color) in map.iter() {
            let hsva = HSVA32::from(*color);
            writeln!(file, "    grad.insert_hue({}, {});", *x as f32, h)?;
            writeln!(file, "    grad.insert_saturation({}, {});", *x as f32, s)?;
            writeln!(file, "    grad.insert_brightness({}, {});", *x as f32, v)?;
        }
        writeln!(file, "    grad.rescale(min, max);")?;
        writeln!(file, "    grad")?;
        writeln!(file, "}}")?;
        Ok(())
    }
}
