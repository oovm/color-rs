use color_core::RGBA32;
use crate::palette::{ColorSpan, Palette};

pub trait ColorGradient {
    fn blend(&self, lhs: RGBA32, rhs: RGBA32, position: f32) -> RGBA32;
    /// Get the color at the given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the color in the gradient.
    ///
    /// # Returns
    ///
    /// The color at the given position.
    fn get_color(&self, palette: &Palette, position: f32) -> RGBA32 {
        let ColorSpan { value, min, max } = palette.get_range(position);
        let normed = float01::float32::f01::scale(value, min.1, max.1).unwrap();
        self.blend(min.0, max.0, normed)
    }
}
