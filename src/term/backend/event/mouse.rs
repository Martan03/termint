use crate::{prelude::Vec2, term::backend::event::KeyModifiers};

/// Detailed information about a mouse event.
///
/// Mouse events are captured only if the terminal emulator supports them and
/// mouse capture is enabled (can be done using
/// [`crate::term::enable_mouse_capture`]).
///
/// # Example:
///
/// This is how MouseEvent can be used in combination of the
/// [`crate::term::Application`] trait:
///
/// ```rust
/// use termint::prelude::*;
/// use termint::term::backend::{MouseButton, MouseEvent, MouseEventKind};
///
/// struct MyApp;
///
/// impl Application for MyApp {
///     type Message = ();
///
///     fn view(&self, _frame: &Frame) -> Element<Self::Message> {
///         "Your UI here".into()
///     }
///
///     fn event(&mut self, event: Event) -> Action {
///         if let Event::Mouse(mouse) = event {
///             match mouse.kind {
///                 MouseEventKind::Down(MouseButton::Left) => {
///                     // Handle the click at mouse.x, mouse.y
///                 }
///                 _ => {}
///             }
///         }
///         Action::NONE
///     }
/// }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    /// The kind of the mouse action that occured (press, release,...)
    pub kind: MouseEventKind,
    /// Keyboard modifiers active during the mouse event (Shift + Click,...)
    pub modifiers: KeyModifiers,
    pub pos: Vec2,
}

/// The type of the mouse action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseEventKind {
    Down(MouseButton),
    Up(MouseButton),
    Drag(MouseButton),
    Move,
    ScrollDown,
    ScrollUp,
    ScrollLeft,
    ScrollRight,
}

/// Represents the mouse buttons
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Button4,
    Button5,
    Button6,
    Button7,
    Other(u32),
}
