use crate::Character;

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
