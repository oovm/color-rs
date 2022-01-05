use super::*;

impl From<RGBA32> for RGBA {
    fn from(rgba: RGBA32) -> Self {
        let rgba = rgba.normalized();
        Self { r: (rgba.r * 255.0) as u8, g: (rgba.g * 255.0) as u8, b: (rgba.b * 255.0) as u8, a: (rgba.a * 255.0) as u8 }
    }
}

impl<T> From<[T; 3]> for RGBA
where
    T: Into<u8> + Copy,
{
    fn from(rgba: [T; 3]) -> Self {
        Self { r: rgba[0].into(), g: rgba[1].into(), b: rgba[2].into(), a: 255 }
    }
}

// noinspection DuplicatedCode
impl<T> From<[T; 4]> for RGBA
where
    T: Into<u8> + Copy,
{
    fn from(rgba: [T; 4]) -> Self {
        Self { r: rgba[0].into(), g: rgba[1].into(), b: rgba[2].into(), a: rgba[3].into() }
    }
}

impl From<u32> for RGBA {
    #[track_caller]
    fn from(rgba: u32) -> Self {
        // if rgba < 0xFFFFFF00 {
        //     panic!("Invalid color value: #{:02X}", rgba);
        // }
        let [r, g, b, a] = rgba.to_be_bytes();
        Self { r, g, b, a }
    }
}
