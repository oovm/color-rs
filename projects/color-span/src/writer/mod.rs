mod html_fmt;

///
#[derive(Debug)]
pub struct HtmlWriter {
    /// has pre block
    pub pre_block: Option<String>,
    /// has code block
    pub code_block: Option<String>,
}
