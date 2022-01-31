
#[test]
pub fn test_bits21() {
    assert_eq!('a' as u32, 0b000000000000000000000001100001);
    let mut c = Character::from(0b_00000001_001_0000_00111111_11111111);
    c.set_char('a');
    c.set_color(2047);
    println!("{:#?}", c);
    println!("{:#b}", c);
}


impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

impl Debug for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Character")
            .field(&self.get_char())
            .field(&self.get_color())
            .finish()
    }
}

impl Binary for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Character")
            .field(&StringWrap::from(format!("{:#021b}", self.erase_color())))
            .field(&StringWrap::from(format!("{:#011b}", self.get_color())))
            .finish()
    }
}

struct StringWrap<'i> {
    repr: Cow<'i, str>,
}

impl From<String> for StringWrap<'_> {
    fn from(s: String) -> Self {
        Self { repr: Cow::Owned(s) }
    }
}

impl Debug for StringWrap<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}


pub struct Character {
    repr: u32,
}

impl From<char> for Character {
    fn from(value: char) -> Self {
        Self {
            repr: value as u32,
        }
    }
}

impl From<u32> for Character {
    fn from(value: u32) -> Self {
        Self {
            repr: value,
        }
    }
}

impl Character {
    #[inline]
    pub fn get_char(&self) -> char {
        unsafe {
            char::from_u32_unchecked(self.erase_color())
        }
    }
    #[inline]
    pub fn set_char(&mut self, new: char) {
        // Remove bits in front of char
        self.repr = self.erase_char() | new as u32
    }
    #[inline]
    pub fn get_color(&self) -> u32 {
        self.repr >> 21
    }
    #[inline]
    pub fn set_color(&mut self, new: u32) {
        debug_assert!(new <= 0x07FF);
        self.repr = self.erase_color() | new << 21
    }
    #[inline]
    fn erase_color(&self) -> u32 {
        self.repr & 0x001FFFFF
    }
    #[inline]
    fn erase_char(&self) -> u32 {
        self.repr & 0xFFE00000
    }
}


