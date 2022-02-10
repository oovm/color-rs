use color_core::RGBA32;
use float01::float32::f01;
use crate::palettes::{ColorSpan};


pub trait ColorMap {
    fn get_color(&self, ratio: f32) -> RGBA32 {
        let _ = ratio;
        unimplemented!("get_color_span not implemented for this ColorMap")
    }

    fn get_color_span(&self, ratio: f32) -> ColorSpan {
        let _ = ratio;
        unimplemented!("get_color_span not implemented for this ColorMap")
    }
}


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
    fn get_color(&self, palette: &impl ColorMap, position: f32) -> RGBA32 {
        let ColorSpan { value, min, max } = palette.get_color_span(position);
        let normed = f01::scale(value, min.1, max.1).unwrap();
        self.blend(min.0, max.0, normed)
    }
}
