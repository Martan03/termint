use crate::term::backend::event::KeyModifiers;

/// Detailed information about a mouse event.
///
/// Mouse events are captured only if the terminal emulator supports them.
///
/// # Example:
///
/// This is how MouseEvent can be used in combination of the [`Application`]
/// trait:
///
/// ```rust
/// use termint::prelude::*;
/// use termint::term::backend::{Button, MouseEvent, MouseEventKind};
///
/// struct MyApp;
///
/// impl Application for MyApp {
///     fn view(&self, _frame: &Frame) -> Element {
///         "Your UI here".into()
///     }
///
///     fn event(&mut self, event: Event) -> Action {
///         if let Event::Mouse(mouse) = event {
///             match mouse.kind {
///                 MouseEventKind::Down(Button::Left) => {
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
    pub x: usize,
    pub y: usize,
}

/// The type of the mouse action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseEventKind {
    Down(Button),
    Up(Button),
    Drag(Button),
    Move,
    ScrollDown,
    ScrollUp,
    ScrollLeft,
    ScrollRight,
}

/// Represents the mouse buttons
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Button {
    Left,
    Middle,
    Right,
    Button4,
    Button5,
    Button6,
    Button7,
    Other(u32),
}
