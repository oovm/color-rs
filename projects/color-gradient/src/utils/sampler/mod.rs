use color_core::{HSVA32, RGBA32};
use image::{GenericImageView, ImageResult, Rgba32FImage, RgbaImage};
use std::{collections::BTreeMap, error::Error, io::Write, path::Path};

pub struct GradientSampler {
    pub points: usize,
    pub margin_left: u32,
    pub margin_right: u32,
    pub margin_top: u32,
    pub margin_bottom: u32,
    pub maps: Vec<(String, BTreeMap<u32, RGBA32>)>,
}

impl GradientSampler {
    pub fn new(points: usize) -> Self {
        Self { points, margin_left: 0, margin_right: 0, margin_top: 0, margin_bottom: 0, maps: vec![] }
    }
    pub fn with_margin(mut self, margin: u32) -> Self {
        self.margin_left = margin;
        self.margin_right = margin;
        self.margin_top = margin;
        self.margin_bottom = margin;
        self
    }
    pub fn sample(&self, image: &Rgba32FImage) -> BTreeMap<u32, RGBA32> {
        let width = image.width() - self.margin_left - self.margin_right;
        let height = image.height() - self.margin_top - self.margin_bottom;
        let view = image.view(self.margin_left, self.margin_top, width, height);
        let mut output = BTreeMap::new();
        let step = width as usize / self.points;
        for i in (0..=width).step_by(step) {
            let color = view.get_pixel(i.saturating_sub(1), 0);
            output.insert(i, RGBA32::new(color.0[0], color.0[1], color.0[2], color.0[3]));
        }
        // let last = view.get_pixel(width, 0);
        // output.insert(width - 1, RGBA32::new(last.0[0], last.0[1], last.0[2], last.0[3]));
        output
    }
    pub fn sample_file<P>(&mut self, path: P, name: &str) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let image_path = path.as_ref();
        let file_path = image_path.with_extension("rs");
        let image = image::open(image_path)?.to_rgba32f();
        let map = self.sample(&image);
        self.maps.push((name.to_string(), map));
        Ok(())
    }
    pub fn export_hsv<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        let mut file = std::fs::File::create(path)?;
        writeln!(file, "impl HsvGradient {{")?;
        for (name, map) in self.maps.iter() {
            let width = *map.last_key_value().expect("Empty map").0;
            writeln!(file, "/// {} color map in HSV color space.", name)?;
            writeln!(file, "/// - step:")?;
            writeln!(
                file,
                "/// ![{name}-step](https://raw.githubusercontent.com/oovm/color-rs/dev/projects/color-gradient/assets/hsv/{name}-step.png)",
                name = name.to_lowercase()
            )?;
            writeln!(file, "/// - linear:")?;
            writeln!(
                file,
                "/// ![{name}-linear](https://raw.githubusercontent.com/oovm/color-rs/dev/projects/color-gradient/assets/hsv/{name}-linear.png)",
                name = name.to_lowercase()
            )?;
            writeln!(file, "pub fn {}(min: f32, max: f32) -> HsvGradient {{", name.to_lowercase())?;
            writeln!(file, "let mut grad = HsvGradient::new(0.0, {:.2});", width as f32)?;
            for (x, color) in map.iter() {
                let hsva = HSVA32::from(*color);
                writeln!(file, "grad.insert_hue({:.2}, {:.2});", *x as f32, hsva.h)?;
                writeln!(file, "grad.insert_saturation({:.2}, {:.2});", *x as f32, hsva.s)?;
                writeln!(file, "grad.insert_brightness({:.2}, {:.2});", *x as f32, hsva.v)?;
            }
            writeln!(file, "grad.rescale(min, max);")?;
            writeln!(file, "grad")?;
            writeln!(file, "}}")?;
        }
        writeln!(file, "}}")?;
        Ok(())
    }
}
