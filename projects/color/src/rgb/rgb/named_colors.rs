use super::*;

impl RGB {
    /// <span style="color:aliceblue">aliceblue: #AAAAAA</span>
    pub const ALICEBLUE: Self = Self { r: 240, g: 248, b: 255 };
}

#[test]
fn test_colors() {
    println!("{:#X}", RGB::ALICEBLUE);
    println!("{:#x}", RGB::ALICEBLUE);
}
