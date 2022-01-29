#![doc = include_str!("readme.md")]
mod html_fmt;

///
#[derive(Debug)]
pub struct HtmlWriter {
    /// has pre block
    pub pre_block: Option<String>,
    /// add style
    pub style: Option<String>,
}

impl Default for HtmlWriter {
    fn default() -> Self {
        Self { pre_block: Some("highlight-block rust".to_string()), style: None }
    }
}

/// one
pub const ONE_DARK: &str = include_str!("one-dark.css");
