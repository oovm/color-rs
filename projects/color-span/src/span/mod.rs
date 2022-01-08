use std::fmt::{Display, Formatter, Write};

/// Write color span into html
///
/// # Arguments
///
/// * `w`:
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// use color_span::ColorSpan;
/// ```
#[derive(Debug)]
pub struct ColorSpan {
    /// Color name of the span
    pub color: String,
    /// Text of the span
    pub text: String,
}

struct HtmlText<'i> {
    text: &'i str,
}

impl ColorSpan {
    /// Write color span into html
    ///
    /// # Arguments
    ///
    /// * `w`:
    ///
    /// returns: Result<(), Error>
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ColorSpan;
    /// ```
    pub fn write_html(&self, mut w: impl Write) -> std::fmt::Result {
        let text = HtmlText { text: &self.text };
        match self.color.as_str() {
            "" => write!(w, r#"{text}"#),
            class => write!(w, r#"<span class="{class}">{text}</span>"#),
        }
    }
}

impl Display for HtmlText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
