use crate::{ClassPalette, HtmlWriter};
use std::fmt::{Arguments, Display, Formatter, Result, Write};

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
        w.write_style()?;
        w.pre_start()?;
        for (class, text) in view {
            w.write_span(&class, HtmlText { pre: self.pre_block.is_some(), text: &text })?
        }
        w.pre_end()?;
        Ok(())
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
    fn write_style(&mut self) -> Result {
        match &self.config.style {
            Some(s) => write!(self, "<style>{s}</style>"),
            None => Ok(()),
        }
    }
    fn pre_start(&mut self) -> Result {
        let class = match &self.config.pre_block {
            Some(s) => s.as_str(),
            None => return Ok(()),
        };
        match class {
            "" => write!(self, "<pre>"),
            _ => write!(self, "<pre class='{}'>", class),
        }
    }
    fn pre_end(&mut self) -> Result {
        if self.config.pre_block.is_some() {
            self.write_str("</pre>")?
        }
        Ok(())
    }
}

struct HtmlText<'i> {
    pre: bool,
    text: &'i str,
}

impl Display for HtmlText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for c in self.text.chars() {
            match c {
                ' ' if self.pre => f.write_char(c)?,
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
