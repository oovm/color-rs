use crate::{writer::HTMLWriter, ColorSpan, ColoredText};
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
    pub fn write_fmt(&self, writer: &mut impl Write, view: &ColoredText) -> Result {
        let mut w = FmtWriter { writer, config: self };
        // for span in view {
        //     w.write_span(span)?
        // }
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

// struct HtmlText<'i> {
//     text: &'i str,
// }
//
// impl ColorSpan {
//     /// Write color span into html
//     ///
//     /// # Arguments
//     ///
//     /// * `w`:
//     ///
//     /// returns: Result<(), Error>
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use color_span::ColorSpan;
//     /// ```
//     pub fn write_html(&self, mut w: impl Write) -> std::fmt::Result {
//         let text = HtmlText { text: &self.text };
//         match self.color.as_str() {
//             "" => write!(w, r#"{text}"#),
//             class => write!(w, r#"<span class="{class}">{text}</span>"#),
//         }
//     }
// }
//
// impl Display for HtmlText<'_> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         for c in self.text.chars() {
//             match c {
//                 ' ' => f.write_str("&nbsp;")?,
//                 // drop CR
//                 '\r' => {},
//                 // write LF
//                 '\n' => f.write_str("<br/>")?,
//                 _ => f.write_char(c)?,
//             }
//         }
//         Ok(())
//     }
// }
