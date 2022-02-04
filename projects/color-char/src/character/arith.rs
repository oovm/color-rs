use crate::Character;

impl PartialEq<char> for Character {
    fn eq(&self, other: &char) -> bool {
        self.get_char() == *other
    }
}

// impl PartialEq<Character> for char {
//     fn eq(&self, other: &Character) -> bool {
//         *self == other.get_char()
//     }
// }
