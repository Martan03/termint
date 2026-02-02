mod key;
mod mouse;

pub use key::*;
pub use mouse::*;

/// Represents a unified terminal event
///
/// This enum is a middle man between the backends (such as Crossterm and
/// Termal) and the application. All events from each backend are converted
/// to this event type, which abstracts away the backend-specific types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// A keyboard event (key press, release, or repeat)
    Key(KeyEvent),
    /// A mouse event (click, movements, scroll)
    ///
    /// **Note**: Mouse support may vary depending on the terminal and whether
    /// mouse capture was enabled during setup (using
    /// [`crate::term::enable_mouse_capture`]).
    Mouse(MouseEvent),
    /// Triggered when the terminal window gains focus.
    FocusGained,
    /// Triggered whent the terminal window loses focus.
    FocusLost,
    /// Triggered when text is pasted into the terminal.
    ///
    /// **Note**: This needs to be enabled during setup (using
    /// [`crate::term::enable_bracketed_paste`]).
    Paste(String),
    /// Triggered when the terminal window is resized. The new dimensions are
    /// in characters.
    ///
    /// When using [`crate::term::Term::run`], this event is intercepted to
    /// automatically render again.
    Resize(usize, usize),
}
