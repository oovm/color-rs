use crate::{AnsiColor, AnsiStyle};
use std::io::Write;

#[derive(Copy, Clone, Debug)]
pub enum AnsiAbility {
    /// The writer supports ANSI escape codes.
    Nothing,
    /// The writer does not support ANSI escape codes.
    Windows,
    /// The writer may or may not support ANSI escape codes.
    Full,
}

impl Default for AnsiAbility {
    fn default() -> Self {
        Self::Full
    }
}

pub struct AnsiWriter<W> {
    ability: AnsiAbility,
    writer: W,
}

impl<W: Write> Write for AnsiWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> AnsiWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { ability: AnsiAbility::default(), writer }
    }
    pub fn get_ability(&self) -> AnsiAbility {
        self.ability
    }
    pub fn set_ability(&mut self, ability: AnsiAbility) {
        self.ability = ability;
    }
    pub fn with_ability(mut self, ability: AnsiAbility) -> Self {
        self.ability = ability;
        self
    }
    pub fn set_style(&mut self, style: &AnsiStyle) -> std::io::Result<()> {
        if style.reset {
            self.reset_style()?;
        }
        if style.bold {
            self.write_str("\x1B[1m")?;
        }
        if style.dimmed {
            self.write_str("\x1B[2m")?;
        }
        if style.italic {
            self.write_str("\x1B[3m")?;
        }
        if style.underline {
            self.write_str("\x1B[4m")?;
        }
        if style.strikethrough {
            self.write_str("\x1B[9m")?;
        }
        if let Some(c) = &style.fg_color {
            self.write_color(true, c, style.intense)?;
        }
        if let Some(c) = &style.bg_color {
            self.write_color(false, c, style.intense)?;
        }
        Ok(())
    }
    #[inline]
    fn reset_style(&mut self) -> std::io::Result<()> {
        self.write_str("\x1B[0m")
    }
    fn write_str(&mut self, s: &str) -> std::io::Result<()> {
        self.write_all(s.as_bytes())
    }

    fn write_color(&mut self, fg: bool, c: &AnsiColor, intense: bool) -> std::io::Result<()> {
        macro_rules! write_intense {
            ($clr:expr) => {
                if fg {
                    self.write_str(concat!("\x1B[38;5;", $clr, "m"))
                }
                else {
                    self.write_str(concat!("\x1B[48;5;", $clr, "m"))
                }
            };
        }
        macro_rules! write_normal {
            ($clr:expr) => {
                if fg { self.write_str(concat!("\x1B[3", $clr, "m")) } else { self.write_str(concat!("\x1B[4", $clr, "m")) }
            };
        }
        macro_rules! write_var_ansi_code {
            ($pre:expr, $($code:expr),+) => {{
                // The loop generates at worst a literal of the form
                // '255,255,255m' which is 12-bytes.
                // The largest `pre` expression we currently use is 7 bytes.
                // This gives us the maximum of 19-bytes for our work buffer.
                let pre_len = $pre.len();
                assert!(pre_len <= 7);
                let mut fmt = [0u8; 19];
                fmt[..pre_len].copy_from_slice($pre);
                let mut i = pre_len - 1;
                $(
                    let c1: u8 = ($code / 100) % 10;
                    let c2: u8 = ($code / 10) % 10;
                    let c3: u8 = $code % 10;
                    let mut printed = false;

                    if c1 != 0 {
                        printed = true;
                        i += 1;
                        fmt[i] = b'0' + c1;
                    }
                    if c2 != 0 || printed {
                        i += 1;
                        fmt[i] = b'0' + c2;
                    }
                    // If we received a zero value we must still print a value.
                    i += 1;
                    fmt[i] = b'0' + c3;
                    i += 1;
                    fmt[i] = b';';
                )+

                fmt[i] = b'm';
                self.write_all(&fmt[0..i+1])
            }}
        }
        macro_rules! write_custom {
            ($ansi256:expr) => {
                if fg { write_var_ansi_code!(b"\x1B[38;5;", $ansi256) } else { write_var_ansi_code!(b"\x1B[48;5;", $ansi256) }
            };

            ($r:expr, $g:expr, $b:expr) => {{
                if fg {
                    write_var_ansi_code!(b"\x1B[38;2;", $r, $g, $b)
                }
                else {
                    write_var_ansi_code!(b"\x1B[48;2;", $r, $g, $b)
                }
            }};
        }
        if intense {
            match *c {
                AnsiColor::Black => write_intense!("8"),
                AnsiColor::Blue => write_intense!("12"),
                AnsiColor::Green => write_intense!("10"),
                AnsiColor::Red => write_intense!("9"),
                AnsiColor::Cyan => write_intense!("14"),
                AnsiColor::Magenta => write_intense!("13"),
                AnsiColor::Yellow => write_intense!("11"),
                AnsiColor::White => write_intense!("15"),
                AnsiColor::Ansi256(c) => write_custom!(c),
                AnsiColor::Rgb(r, g, b) => write_custom!(r, g, b),
            }
        }
        else {
            match *c {
                AnsiColor::Black => write_normal!("0"),
                AnsiColor::Blue => write_normal!("4"),
                AnsiColor::Green => write_normal!("2"),
                AnsiColor::Red => write_normal!("1"),
                AnsiColor::Cyan => write_normal!("6"),
                AnsiColor::Magenta => write_normal!("5"),
                AnsiColor::Yellow => write_normal!("3"),
                AnsiColor::White => write_normal!("7"),
                AnsiColor::Ansi256(c) => write_custom!(c),
                AnsiColor::Rgb(r, g, b) => write_custom!(r, g, b),
            }
        }
    }
}
