use bitflags::bitflags;

/// Detailed information about a keyboard event.
///
/// # Example
///
/// This is how KeyEvent can be used in combination of the [`Application`]
/// trait:
///
/// ```rust
/// use termint::prelude::*;
///
/// struct MyApp;
///
/// impl Application for MyApp {
///     fn view(&self, _frame: &Frame) -> Element {
///         "Your UI here".into()
///     }
///
///     fn event(&mut self, event: Event) -> Action {
///         if let Event::Key(key) = event {
///             match key.code {
///                 KeyCode::Char('q') => return Action::QUIT,
///                 _ => {}
///             }
///         }
///         Action::NONE
///     }
/// }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// The actual key that was pressed or released.
    pub code: KeyCode,
    /// Keyboard modifiers active during the event (such as Ctrl, Alt,...).
    pub modifiers: KeyModifiers,
    /// The type of the event (Press, Release,...).
    pub kind: KeyEventKind,
    /// State of the keyboard (such as Numpad, CapsLock,...).
    pub state: KeyEventState,
}

/// Represents the specific key associated with an event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCode {
    /// A standard character key (such as 'a', 'B', '#').
    Char(char),
    F(u8),
    Backspace,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
    /// Media specific key presses (such as Play, Volume,...).
    Media(MediaKeyCode),
    /// Standalone modifier key presses (such as pressing `Left Ctrl` only).
    ///
    /// **Note**: This is only reported by terminals supporting advanced
    /// protocols, such as Kitty.
    Modifier(ModifierKeyCode),
}

/// Key codes for the media-related hardware buttons.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaKeyCode {
    Play,
    Pause,
    PlayPause,
    Reverse,
    Stop,
    FastForward,
    Rewind,
    TrackNext,
    TrackPrevious,
    Record,
    LowerVolume,
    RaiseVolume,
    MuteVolume,
}

/// Key codes for the standalone modifier button events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierKeyCode {
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    LeftHyper,
    LeftMeta,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    RightHyper,
    RightMeta,
    IsoLevel3Shift,
    IsoLevel5Shift,
}

bitflags! {
    /// Bitflags representing active keyboard modifiers.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct KeyModifiers: u8 {
        const NONE = 0b0000_0000;
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        /// Super/Windows/Command key
        const SUPER = 0b0000_1000;
        const HYPER = 0b0001_0000;
        const META = 0b0010_0000;
    }
}

/// Type of the keyboard event.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KeyEventKind {
    Press,
    Repeat,
    Release,
}

bitflags! {
    /// Bitflags representing the keyboard state during a key event.
    #[derive(Debug,Clone, Eq, PartialEq)]
    pub struct KeyEventState: u8 {
        const NONE = 0b0000_0000;
        const KEYPAD = 0b0000_0001;
        const CAPS_LOCK = 0b0000_0010;
        const NUM_LOCK = 0b0000_0100;
    }
}
