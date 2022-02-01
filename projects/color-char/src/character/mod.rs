mod convert;
mod display;

#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "serde")]
mod ser;

pub struct Character {
    repr: u32,
}

impl Character {
    #[inline]
    pub fn get_char(&self) -> char {
        unsafe { char::from_u32_unchecked(self.erase_color()) }
    }
    /// # Arguments
    ///
    /// * `new`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use color_char::Character;
    /// ```
    #[inline]
    pub fn set_char(&mut self, new: char) {
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
