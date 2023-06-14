#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnsiColor {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnsiStyle {
    fg_color: Option<AnsiColor>,
    bg_color: Option<AnsiColor>,
    bold: bool,
    intense: bool,
    underline: bool,
    dimmed: bool,
    italic: bool,
    reset: bool,
    strikethrough: bool,
}
