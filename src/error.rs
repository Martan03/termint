use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot rerender: no previous widget tree found.")]
    NoPreviousWidget,
    #[error("Cannot determine terminal size.")]
    UnknownTerminalSize,
    #[error(transparent)]
    Termal(#[from] termal::error::Error),
}
