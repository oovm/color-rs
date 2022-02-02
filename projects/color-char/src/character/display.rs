use std::{
    borrow::Cow,
    fmt::{Binary, Debug, Display, Formatter},
};

use crate::character::Character;

impl Default for Character {
    fn default() -> Self {
        Character::new(' ', 0)
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

impl Debug for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Character").field(&self.get_char()).field(&self.get_color()).finish()
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
