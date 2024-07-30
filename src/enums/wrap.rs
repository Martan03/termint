/// Indicates how text should be wrapped
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Wrap {
    /// Wraps after any letter
    Letter,
    /// Wrap after word
    #[default]
    Word,
}
