use super::*;

impl Debug for ColoredText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ColoredText").field(&self.text()).finish()
    }
}

impl Debug for CharacterColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Colored").field(&self.char).field(&self.color).finish()
    }
}

impl From<&str> for ColoredText {
    fn from(s: &str) -> Self {
        ColoredText::new(s)
    }
}

impl From<String> for ColoredText {
    fn from(s: String) -> Self {
        ColoredText::new(&s)
    }
}
impl From<char> for CharacterColor {
    fn from(c: char) -> Self {
        Self { char: c, color: 0 }
    }
}

impl From<[u8; 4]> for CharacterColor {
    fn from(c: [u8; 4]) -> Self {
        let [l1, l2, l3, color] = c;
        let char_part = u32::from_le_bytes([l1, l2, l3, 0]).min(0x10FFFF);
        Self { char: unsafe { char::from_u32_unchecked(char_part) }, color }
    }
}

impl From<&[u8; 4]> for CharacterColor {
    #[inline]
    fn from(c: &[u8; 4]) -> Self {
        CharacterColor::from(*c)
    }
}

impl Into<[u8; 4]> for CharacterColor {
    fn into(self) -> [u8; 4] {
        let [l1, l2, l3, _] = u32::from(self.char).to_le_bytes();
        [l1, l2, l3, self.color]
    }
}
