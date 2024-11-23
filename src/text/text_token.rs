/// Text token for parsing the text
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TextToken {
    Text { text: String, len: usize },
    Newline,
    End,
}

impl TextToken {
    /// Creates new text token
    pub fn text(text: String, len: usize) -> Self {
        Self::Text { text, len }
    }
}
