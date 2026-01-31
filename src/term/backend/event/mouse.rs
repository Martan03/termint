use crate::term::backend::event::KeyModifiers;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub modifiers: KeyModifiers,
    pub x: usize,
    pub y: usize,
}

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
