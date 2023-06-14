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

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct AnsiStyle {
    /// The foreground color.
    pub fg_color: Option<AnsiColor>,
    /// The background color.
    pub bg_color: Option<AnsiColor>,
    /// Get whether this is bold or not.
    pub bold: bool,
    /// Get whether this is intense or not.
    ///
    /// On Unix-like systems, this will output the ANSI escape sequence
    /// that will print a high-intensity version of the color
    /// specified.
    ///
    /// On Windows systems, this will output the ANSI escape sequence
    /// that will print a brighter version of the color specified.
    pub intense: bool,
    /// Get whether this is underlined or not.
    pub underline: bool,
    /// Get whether this is dimmed or not.
    ///
    /// Note that the dimmed setting has no effect in a Windows console.
    pub dimmed: bool,
    /// Get whether this is italic or not.
    ///
    /// Note that the italic setting has no effect in a Windows console.
    pub italic: bool,
    /// Get whether reset is enabled or not.
    ///
    /// reset is enabled by default. When disabled and using ANSI escape
    /// sequences, a "reset" code will be emitted every time a `ColorSpec`'s
    /// settings are applied.
    ///
    /// Note that the reset setting has no effect in a Windows console.
    pub reset: bool,

    /// Get whether this is strikethrough or not.
    ///
    /// Note that the strikethrough setting has no effect in a Windows console.
    pub strikethrough: bool,
}

impl AnsiStyle {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { fg_color: Some(AnsiColor::Rgb(r, g, b)), ..Self::default() }
    }
    pub fn with_fg_color(self, color: AnsiColor) -> Self {
        Self { fg_color: Some(color), ..self }
    }
    pub fn with_bg_color(self, color: AnsiColor) -> Self {
        Self { bg_color: Some(color), ..self }
    }
}
