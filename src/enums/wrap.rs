/// Indicates how text should be wrapped
#[derive(Debug, Default, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Wrap {
    /// Wraps after any letter
    Letter,
    /// Wraps after word
    #[default]
    Word,
}
