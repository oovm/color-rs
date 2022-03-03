use color_core::RGBA8;
use color_gradient::{HsvGradient, Interpolator};
use image::{ImageBuffer, ImageResult, Rgba};
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

#[test]
fn test_step() {
    let mut map = Interpolator::new(0.0, 1.0);
    assert_eq!(map.get_step(0), 0.0);
    assert_eq!(map.get_step(30000), 0.0);
    assert_eq!(map.get_step(65535), 1.0);
    map.insert(0, 0.1);
    assert_eq!(map.get_step(0), 0.1);
    assert_eq!(map.get_step(30000), 0.1);
    assert_eq!(map.get_step(65535), 1.0);
    map.insert(65535, 0.9);
    assert_eq!(map.get_step(0), 0.1);
    assert_eq!(map.get_step(30000), 0.1);
    assert_eq!(map.get_step(65535), 0.9);
    map.insert(10000, 0.5);
    assert_eq!(map.get_step(0), 0.1);
    assert_eq!(map.get_step(30000), 0.5);
    assert_eq!(map.get_step(65535), 0.9);
}

#[test]
fn test_linear() {
    let mut map = Interpolator::new(0.0, 1.0);
    assert_eq!(map.get_linear(0), 0.0);
    assert_eq!(map.get_linear(30000), 0.45777065);
    assert_eq!(map.get_linear(65535), 1.0);
    map.insert(0, 0.1);
    assert_eq!(map.get_linear(0), 0.1);
    assert_eq!(map.get_linear(30000), 0.5119936);
    assert_eq!(map.get_linear(65535), 1.0);
    map.insert(65535, 0.9);
    assert_eq!(map.get_linear(0), 0.1);
    assert_eq!(map.get_linear(30000), 0.4662165);
    assert_eq!(map.get_linear(65535), 0.9);
    map.insert(10000, 0.5);
    assert_eq!(map.get_linear(0), 0.1);
    assert_eq!(map.get_linear(30000), 0.6440533);
    assert_eq!(map.get_linear(65535), 0.9);
}
