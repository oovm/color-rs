use super::*;

impl Debug for TextView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ColoredText").field(&self.text()).finish()
    }
}

impl From<&str> for TextView {
    fn from(s: &str) -> Self {
        TextView::new(s)
    }
}

impl From<String> for TextView {
    fn from(s: String) -> Self {
        TextView::new(&s)
    }
}
