use std::time::Duration;

use crate::{
    term::backend::{
        event::{
            Button, KeyCode, KeyEvent, KeyEventKind, KeyEventState,
            KeyModifiers, MediaKeyCode, ModifierKeyCode, MouseEvent,
            MouseEventKind,
        },
        Backend, Event,
    },
    Error,
};

use crossterm::event::{
    Event as CTermEvent, KeyCode as CTermKeyCode, KeyEvent as CTermKeyEvent,
    KeyEventKind as CTermKeyEventKind, KeyEventState as CTermKeyEventState,
    KeyModifiers as CTermKeyModifiers, MediaKeyCode as CTermMediaKeyCode,
    ModifierKeyCode as CTermModifierKeyCode, MouseButton,
    MouseEvent as CTermMouseEvent, MouseEventKind as CTermMouseEventKind,
};

#[derive(Debug, Default)]
pub struct CrosstermBackend;

impl Backend for CrosstermBackend {
    fn read_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Event>, Error> {
        if crossterm::event::poll(timeout)? {
            Ok(crossterm::event::read()?.try_into().ok())
        } else {
            Ok(None)
        }
    }
}

impl TryFrom<CTermEvent> for Event {
    type Error = &'static str;

    fn try_from(value: CTermEvent) -> Result<Self, Self::Error> {
        match value {
            CTermEvent::FocusGained => Ok(Event::FocusGained),
            CTermEvent::FocusLost => Ok(Event::FocusLost),
            CTermEvent::Key(e) => Ok(Event::Key(e.into())),
            CTermEvent::Mouse(e) => Ok(Event::Mouse(e.into())),
            CTermEvent::Paste(v) => Ok(Event::Paste(v)),
            CTermEvent::Resize(w, h) => {
                Ok(Event::Resize(w as usize, h as usize))
            }
        }
    }
}

impl From<CTermKeyEvent> for KeyEvent {
    fn from(value: CTermKeyEvent) -> Self {
        Self {
            code: value.code.into(),
            modifiers: value.modifiers.into(),
            kind: value.kind.into(),
            state: value.state.into(),
        }
    }
}

impl From<CTermKeyCode> for KeyCode {
    fn from(value: CTermKeyCode) -> Self {
        match value {
            CTermKeyCode::Backspace => Self::Backspace,
            CTermKeyCode::Enter => Self::Enter,
            CTermKeyCode::Left => Self::Left,
            CTermKeyCode::Right => Self::Right,
            CTermKeyCode::Up => Self::Up,
            CTermKeyCode::Down => Self::Down,
            CTermKeyCode::Home => Self::Home,
            CTermKeyCode::End => Self::End,
            CTermKeyCode::PageUp => Self::PageUp,
            CTermKeyCode::PageDown => Self::PageDown,
            CTermKeyCode::Tab => Self::Tab,
            CTermKeyCode::BackTab => Self::BackTab,
            CTermKeyCode::Delete => Self::Delete,
            CTermKeyCode::Insert => Self::Insert,
            CTermKeyCode::F(n) => Self::F(n),
            CTermKeyCode::Char(c) => Self::Char(c),
            CTermKeyCode::Null => Self::Null,
            CTermKeyCode::Esc => Self::Esc,
            CTermKeyCode::CapsLock => Self::CapsLock,
            CTermKeyCode::ScrollLock => Self::ScrollLock,
            CTermKeyCode::NumLock => Self::NumLock,
            CTermKeyCode::PrintScreen => Self::PrintScreen,
            CTermKeyCode::Pause => Self::Pause,
            CTermKeyCode::Menu => Self::Menu,
            CTermKeyCode::KeypadBegin => Self::KeypadBegin,
            CTermKeyCode::Media(m) => Self::Media(m.into()),
            CTermKeyCode::Modifier(m) => Self::Modifier(m.into()),
        }
    }
}

impl From<CTermMediaKeyCode> for MediaKeyCode {
    fn from(m: CTermMediaKeyCode) -> Self {
        match m {
            CTermMediaKeyCode::Play => Self::Play,
            CTermMediaKeyCode::Pause => Self::Pause,
            CTermMediaKeyCode::PlayPause => Self::PlayPause,
            CTermMediaKeyCode::Reverse => Self::Reverse,
            CTermMediaKeyCode::Stop => Self::Stop,
            CTermMediaKeyCode::FastForward => Self::FastForward,
            CTermMediaKeyCode::Rewind => Self::Rewind,
            CTermMediaKeyCode::TrackNext => Self::TrackNext,
            CTermMediaKeyCode::TrackPrevious => Self::TrackPrevious,
            CTermMediaKeyCode::Record => Self::Record,
            CTermMediaKeyCode::LowerVolume => Self::LowerVolume,
            CTermMediaKeyCode::RaiseVolume => Self::RaiseVolume,
            CTermMediaKeyCode::MuteVolume => Self::MuteVolume,
        }
    }
}

impl From<CTermModifierKeyCode> for ModifierKeyCode {
    fn from(m: CTermModifierKeyCode) -> Self {
        match m {
            CTermModifierKeyCode::LeftShift => Self::LeftShift,
            CTermModifierKeyCode::LeftControl => Self::LeftControl,
            CTermModifierKeyCode::LeftAlt => Self::LeftAlt,
            CTermModifierKeyCode::LeftSuper => Self::LeftSuper,
            CTermModifierKeyCode::LeftHyper => Self::LeftHyper,
            CTermModifierKeyCode::LeftMeta => Self::LeftMeta,
            CTermModifierKeyCode::RightShift => Self::RightShift,
            CTermModifierKeyCode::RightControl => Self::RightControl,
            CTermModifierKeyCode::RightAlt => Self::RightAlt,
            CTermModifierKeyCode::RightSuper => Self::RightSuper,
            CTermModifierKeyCode::RightHyper => Self::RightHyper,
            CTermModifierKeyCode::RightMeta => Self::RightMeta,
            CTermModifierKeyCode::IsoLevel3Shift => Self::IsoLevel3Shift,
            CTermModifierKeyCode::IsoLevel5Shift => Self::IsoLevel5Shift,
        }
    }
}

impl From<CTermKeyModifiers> for KeyModifiers {
    fn from(value: CTermKeyModifiers) -> Self {
        let mut m = KeyModifiers::NONE;
        if value.contains(CTermKeyModifiers::SHIFT) {
            m |= KeyModifiers::SHIFT;
        }
        if value.contains(CTermKeyModifiers::CONTROL) {
            m |= KeyModifiers::CONTROL;
        }
        if value.contains(CTermKeyModifiers::ALT) {
            m |= KeyModifiers::ALT;
        }
        if value.contains(CTermKeyModifiers::SUPER) {
            m |= KeyModifiers::SUPER;
        }
        if value.contains(CTermKeyModifiers::HYPER) {
            m |= KeyModifiers::HYPER;
        }
        if value.contains(CTermKeyModifiers::META) {
            m |= KeyModifiers::META;
        }
        m
    }
}

impl From<CTermKeyEventKind> for KeyEventKind {
    fn from(value: CTermKeyEventKind) -> Self {
        match value {
            CTermKeyEventKind::Press => KeyEventKind::Press,
            CTermKeyEventKind::Repeat => KeyEventKind::Repeat,
            CTermKeyEventKind::Release => KeyEventKind::Release,
        }
    }
}

impl From<CTermKeyEventState> for KeyEventState {
    fn from(value: CTermKeyEventState) -> Self {
        let mut state = KeyEventState::NONE;

        if value.contains(CTermKeyEventState::KEYPAD) {
            state |= KeyEventState::KEYPAD;
        }
        if value.contains(CTermKeyEventState::CAPS_LOCK) {
            state |= KeyEventState::CAPS_LOCK;
        }
        if value.contains(CTermKeyEventState::NUM_LOCK) {
            state |= KeyEventState::NUM_LOCK;
        }

        state
    }
}

impl From<CTermMouseEvent> for MouseEvent {
    fn from(value: CTermMouseEvent) -> Self {
        Self {
            kind: value.kind.into(),
            modifiers: value.modifiers.into(),
            x: value.column.into(),
            y: value.row.into(),
        }
    }
}

impl From<CTermMouseEventKind> for MouseEventKind {
    fn from(value: CTermMouseEventKind) -> Self {
        match value {
            CTermMouseEventKind::Down(btn) => Self::Down(btn.into()),
            CTermMouseEventKind::Up(btn) => Self::Up(btn.into()),
            CTermMouseEventKind::Drag(btn) => Self::Drag(btn.into()),
            CTermMouseEventKind::Moved => Self::Move,
            CTermMouseEventKind::ScrollDown => Self::ScrollDown,
            CTermMouseEventKind::ScrollUp => Self::ScrollUp,
            CTermMouseEventKind::ScrollLeft => Self::ScrollLeft,
            CTermMouseEventKind::ScrollRight => Self::ScrollRight,
        }
    }
}

impl From<MouseButton> for Button {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => Self::Left,
            MouseButton::Right => Self::Right,
            MouseButton::Middle => Self::Middle,
        }
    }
}
