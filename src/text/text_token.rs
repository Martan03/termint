/// Text token used by the `TextParser`
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TextToken {
    // Text token containing the text and its length
    Text { text: String, len: usize },
    // Newline token
    Newline,
    // End of the text token
    End,
}

impl TextToken {
    /// Creates new text token
    pub fn text(text: String, len: usize) -> Self {
        Self::Text { text, len }
    }
}
