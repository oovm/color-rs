mod arith;
mod convert;
mod display;

#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "serde")]
mod ser;
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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Character {
    repr: u32,
}

impl Character {
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
    pub fn new(char: char, color: u32) -> Self {
        Self { repr: color << 21 | char as u32 }
    }
    /// Get the character for last 21 bits
    ///
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
    /// let c = Character::from(0b000011010_000000000000001011010);
    /// assert_eq!(c.get_char(), 'Z');
    /// assert_eq!(c.get_color(), 26);
    /// ```
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
    pub fn with_char(self, new: char) -> Self {
        Self { repr: self.erase_char() | new as u32 }
    }
    /// Get the color id from first 11 bits
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use color_char::Character;
    /// let c = Character::from(0b000011010_000000000000001011010);
    /// assert_eq!(c.get_color(), 26);
    /// ```
    #[inline]
    pub fn get_color(&self) -> u32 {
        self.repr >> 21
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
    pub fn set_color(&mut self, new: u32) {
        debug_assert!(new <= 0x07FF);
        self.repr = self.erase_color() | new << 21
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
    pub fn with_color(self, new: u32) -> Self {
        debug_assert!(new <= 0x07FF);
        Self { repr: self.erase_color() | new << 21 }
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
