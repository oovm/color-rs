use crate::{writer::HTMLWriter, ColorSpan, TextColorView};
use std::fmt::{Arguments, Result, Write};

struct FmtWriter<'i, W: Write> {
    writer: &'i mut W,
    config: &'i HTMLWriter,
}

impl<'i, W> Write for FmtWriter<'i, W>
where
    W: Write,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> Result {
        self.writer.write_str(s)
    }
    #[inline]
    fn write_char(&mut self, c: char) -> Result {
        self.writer.write_char(c)
    }
    #[inline]
    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> Result {
        self.writer.write_fmt(args)
    }
}

impl HTMLWriter {
    /// Write to html span
    ///
    /// # Arguments
    ///
    /// * `writer`:
    /// * `view`:
    ///
    /// returns: Result<(), Error>
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::HTMLWriter;
    /// ```
    pub fn write_fmt(&self, writer: &mut impl Write, view: &TextColorView) -> Result {
        let mut w = FmtWriter { writer, config: self };
        for span in view {
            w.write_span(span)?
        }
        Ok(())
    }
}

impl<'i, W> FmtWriter<'i, W>
where
    W: Write,
{
    fn write_span(&mut self, span: ColorSpan) -> Result {
        Ok(())
    }
}
