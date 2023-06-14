/// The definition of the ANSI colors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AnsiColor {
    /// The ansi black color, escape code `0;30`.
    Black,
    /// The ansi blue color, escape code `0;34`.
    Blue,
    /// The ansi green color, escape code `0;32`.
    Green,
    /// The ansi cyan color, escape code `0;36`.
    Red,
    /// The ansi red color, escape code `0;31`.
    Cyan,
    /// The ansi magenta color, escape code `0;35`.
    Magenta,
    /// The ansi yellow color, escape code `0;33`.
    Yellow,
    /// The ansi white color, escape code `0;37`.
    White,
    /// An ANSI color with a value in the range `[0, 255]`.
    Ansi256(u8),
    /// An ANSI color with a value in the range `[0, 255]`.
    Rgb(u8, u8, u8),
}

/// The definition of the ANSI styles.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
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
    /// Create a new `ColorSpec` with the foreground color specified.
    pub fn new(foreground: AnsiColor) -> Self {
        Self { fg_color: Some(foreground), ..Self::default() }
    }
    /// Create a new `ColorSpec` with the foreground color specified.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { fg_color: Some(AnsiColor::Rgb(r, g, b)), ..Self::default() }
    }
    /// Create a new `ColorSpec` with the foreground color specified.
    pub fn with_fg_color(self, color: AnsiColor) -> Self {
        Self { fg_color: Some(color), ..self }
    }
    /// Create a new `ColorSpec` with the background color specified.
    pub fn with_bg_color(self, color: AnsiColor) -> Self {
        Self { bg_color: Some(color), ..self }
    }
    /// Create a new [`AnsiStyle`] with underline enabled.
    pub fn with_underline(self) -> Self {
        Self { underline: true, ..self }
    }
    /// Create a new [`AnsiStyle`] with dimmed enabled.
    pub fn with_bold(self) -> Self {
        Self { bold: true, ..self }
    }
    /// Create a new [`AnsiStyle`] with dimmed enabled.
    pub fn with_italic(self) -> Self {
        Self { italic: true, ..self }
    }
}
