use color_core::{RGBA32, RGBA8};
use colormap::{ColorGradient, ColorMap, HsvGradient, Palette, QuadraticGradient};
use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::{Path, PathBuf};

mod hsv;

#[test]
fn ready() {
    println!("it works!")
}

pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().expect("failed to get root")
}
pub fn assets(path: &str) -> PathBuf {
    root().join("assets").join(path)
}
pub fn tests(path: &str) -> PathBuf {
    root().join("tests").join(path)
}
