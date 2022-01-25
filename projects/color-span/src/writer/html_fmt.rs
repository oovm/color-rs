use crate::{writer::HtmlWriter, ClassPalette};
use std::fmt::{Arguments, Display, Formatter, Result, Write};

impl Default for HtmlWriter {
    fn default() -> Self {
        Self { pre_block: None, code_block: None }
    }
}

struct FmtWriter<'i, W: Write> {
    writer: &'i mut W,
    config: &'i HtmlWriter,
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

impl HtmlWriter {
    /// Write to html palette
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
    /// use color_span::HtmlWriter;
    /// ```
    pub fn write_fmt(&self, writer: &mut impl Write, view: &ClassPalette) -> Result {
        let mut w = FmtWriter { writer, config: self };
        for (class, text) in view {
            w.write_span(&class, HtmlText { text: &text })?
        }
        Ok(())
    }
}

impl<'i, W> FmtWriter<'i, W>
where
    W: Write,
{
    fn write_span(&mut self, class: &str, text: HtmlText) -> Result {
        match class {
            "" => write!(self, r#"{text}"#)?,
            class => write!(self, r#"<span class="{class}">{text}</span>"#)?,
        }
        Ok(())
    }
}

struct HtmlText<'i> {
    text: &'i str,
}

impl Display for HtmlText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for c in self.text.chars() {
            match c {
                ' ' => f.write_str("&nbsp;")?,
                // drop CR
                '\r' => {},
                // write LF
                '\n' => f.write_str("<br/>")?,
                _ => f.write_char(c)?,
            }
        }
        Ok(())
    }
}
