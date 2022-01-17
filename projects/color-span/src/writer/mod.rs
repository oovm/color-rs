mod html_fmt;

///
#[derive(Debug)]
pub struct HTMLWriter {
    /// has pre block
    pub pre_block: Option<String>,
    /// has code block
    pub code_block: Option<String>,
}

impl Default for HTMLWriter {
    fn default() -> Self {
        Self { pre_block: None, code_block: None }
    }
}
