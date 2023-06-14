use crate::AnsiColor;
use std::io::Write;

pub struct Ansi<W> {
    writer: W,
}

/// This trait describes the behavior of writers that support colored output.
pub trait WriteColor: Write {
    /// Returns true if and only if the underlying writer supports colors.
    fn supports_color(&self) -> bool;

    /// Set the color settings of the writer.
    ///
    /// Subsequent writes to this writer will use these settings until either
    /// `reset` is called or new color settings are set.
    ///
    /// If there was a problem setting the color settings, then an error is
    /// returned.
    fn set_color(&mut self, spec: &ColorSpec) -> std::io::Result<()>;

    /// Reset the current color settings to their original settings.
    ///
    /// If there was a problem resetting the color settings, then an error is
    /// returned.
    fn reset(&mut self) -> std::io::Result<()>;

    /// Returns true if and only if the underlying writer must synchronously
    /// interact with an end user's device in order to control colors. By
    /// default, this always returns `false`.
    ///
    /// In practice, this should return `true` if the underlying writer is
    /// manipulating colors using the Windows console APIs.
    ///
    /// This is useful for writing generic code (such as a buffered writer)
    /// that can perform certain optimizations when the underlying writer
    /// doesn't rely on synchronous APIs. For example, ANSI escape sequences
    /// can be passed through to the end user's device as is.
    fn is_synchronous(&self) -> bool {
        false
    }
}

impl<'a, T: ?Sized + WriteColor> WriteColor for &'a mut T {
    fn supports_color(&self) -> bool {
        (&**self).supports_color()
    }
    fn set_color(&mut self, spec: &ColorSpec) -> std::io::Result<()> {
        (&mut **self).set_color(spec)
    }
    fn reset(&mut self) -> std::io::Result<()> {
        (&mut **self).reset()
    }
    fn is_synchronous(&self) -> bool {
        (&**self).is_synchronous()
    }
}

impl<T: ?Sized + WriteColor> WriteColor for Box<T> {
    fn supports_color(&self) -> bool {
        (&**self).supports_color()
    }
    fn set_color(&mut self, spec: &ColorSpec) -> std::io::Result<()> {
        (&mut **self).set_color(spec)
    }
    fn reset(&mut self) -> std::io::Result<()> {
        (&mut **self).reset()
    }
    fn is_synchronous(&self) -> bool {
        (&**self).is_synchronous()
    }
}

impl<W: Write> Ansi<W> {
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
