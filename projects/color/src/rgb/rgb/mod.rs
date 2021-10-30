use super::*;

mod convert;
mod display;
mod named_colors;

impl Default for RGB {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl RGB {}
