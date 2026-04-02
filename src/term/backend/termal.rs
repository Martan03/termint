use std::time::Duration;

use termal::raw::events::{
    Event as TermalEvent, Key, KeyCode as TermalKeyCode,
    Modifiers as TermalMod, mouse::Button as TermalButton,
    mouse::Event as TermalMouseEvent, mouse::Mouse,
};
use termal::raw::events::{StateChange, Status};
use termal::raw::{StdioProvider, Terminal};

use crate::prelude::Vec2;
use crate::term::backend::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton,
    MouseEvent, MouseEventKind,
};
use crate::{
    Error,
    term::backend::{Backend, Event},
};

/// An event-reading backend powered by the `termal` crate. It is used as a
/// generic parameter for [`crate::term::Term`], which then uses it as the
/// backend.
///
/// # Usage:
/// ```rust,no_run
/// use termint::prelude::*;
///
/// # fn main() -> Result<(), Error> {
/// Term::<TermalBackend>::init()?;
/// # Ok(())
/// # }
/// ```
///
/// # Requirements
///
/// This requires `backend-termal` feature to be enabled.
#[derive(Debug, Default)]
pub struct TermalBackend {
    terminal: Terminal<StdioProvider>,
    paste_buffer: Option<String>,
}

impl Backend for TermalBackend {
    fn read_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Event>, Error> {
        let event = match self.terminal.read_timeout(timeout)? {
            Some(e) => e,
            None => return Ok(None),
        };

        match event {
            TermalEvent::StateChange(StateChange::BracketedPasteStart) => {
                self.paste_buffer = Some(String::new());
                self.read_event(timeout)
            }
            TermalEvent::StateChange(StateChange::BracketedPasteEnd) => {
                let content = self.paste_buffer.take().unwrap_or_default();
                Ok(Some(Event::Paste(content)))
            }
            TermalEvent::KeyPress(key) if self.paste_buffer.is_some() => {
                if let Some(c) = key.key_char {
                    if let Some(buf) = self.paste_buffer.as_mut() {
                        buf.push(c);
                    }
                }
                self.read_event(timeout)
            }
            other => Ok(other.try_into().ok()),
        }
    }
}

impl TryFrom<TermalEvent> for Event {
    type Error = &'static str;

    fn try_from(value: TermalEvent) -> Result<Self, Self::Error> {
        match value {
            TermalEvent::KeyPress(key) => Ok(Event::Key(key.into())),
            TermalEvent::Mouse(mouse) => Ok(Event::Mouse(mouse.into())),
            TermalEvent::Status(status) => status.try_into(),
            TermalEvent::Focus => Ok(Event::FocusGained),
            TermalEvent::FocusLost => Ok(Event::FocusLost),
            TermalEvent::StateChange(_state) => Err("unsupported event type"),
        }
    }
}

impl From<Key> for KeyEvent {
    fn from(value: Key) -> Self {
        Self {
            code: value.code.into(),
            modifiers: value.modifiers.into(),
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }
}

impl From<TermalKeyCode> for KeyCode {
    fn from(value: TermalKeyCode) -> Self {
        match value {
            TermalKeyCode::Up => KeyCode::Up,
            TermalKeyCode::Down => KeyCode::Down,
            TermalKeyCode::Right => KeyCode::Right,
            TermalKeyCode::Left => KeyCode::Left,
            TermalKeyCode::Space => KeyCode::Char(' '),
            TermalKeyCode::Tab => KeyCode::Tab,
            TermalKeyCode::Enter => KeyCode::Enter,
            TermalKeyCode::F0 => KeyCode::F(0),
            TermalKeyCode::F1 => KeyCode::F(1),
            TermalKeyCode::F2 => KeyCode::F(2),
            TermalKeyCode::F3 => KeyCode::F(3),
            TermalKeyCode::F4 => KeyCode::F(4),
            TermalKeyCode::F5 => KeyCode::F(5),
            TermalKeyCode::F6 => KeyCode::F(6),
            TermalKeyCode::F7 => KeyCode::F(7),
            TermalKeyCode::F8 => KeyCode::F(8),
            TermalKeyCode::F9 => KeyCode::F(9),
            TermalKeyCode::F10 => KeyCode::F(10),
            TermalKeyCode::F11 => KeyCode::F(11),
            TermalKeyCode::F12 => KeyCode::F(12),
            TermalKeyCode::F13 => KeyCode::F(13),
            TermalKeyCode::F14 => KeyCode::F(14),
            TermalKeyCode::F15 => KeyCode::F(15),
            TermalKeyCode::F16 => KeyCode::F(16),
            TermalKeyCode::F17 => KeyCode::F(17),
            TermalKeyCode::F18 => KeyCode::F(18),
            TermalKeyCode::F19 => KeyCode::F(19),
            TermalKeyCode::F20 => KeyCode::F(20),
            TermalKeyCode::Delete => KeyCode::Delete,
            TermalKeyCode::Insert => KeyCode::Insert,
            TermalKeyCode::End => KeyCode::End,
            TermalKeyCode::Home => KeyCode::Home,
            TermalKeyCode::PgUp => KeyCode::PageUp,
            TermalKeyCode::PgDown => KeyCode::PageDown,
            TermalKeyCode::Backspace => KeyCode::Backspace,
            TermalKeyCode::Esc => KeyCode::Esc,
            TermalKeyCode::Char(c) => KeyCode::Char(c),
        }
    }
}

impl From<TermalMod> for KeyModifiers {
    fn from(t_mod: TermalMod) -> Self {
        let mut my_mod = KeyModifiers::NONE;

        if t_mod.contains(TermalMod::SHIFT) {
            my_mod |= KeyModifiers::SHIFT;
        }
        if t_mod.contains(TermalMod::CONTROL) {
            my_mod |= KeyModifiers::CONTROL;
        }
        if t_mod.contains(TermalMod::ALT) {
            my_mod |= KeyModifiers::ALT;
        }
        if t_mod.contains(TermalMod::META) {
            my_mod |= KeyModifiers::META;
        }

        my_mod
    }
}

impl From<Mouse> for MouseEvent {
    fn from(value: Mouse) -> Self {
        let kind = match value.event {
            // Button shouldn't be None when event Down or Up
            TermalMouseEvent::Down => MouseEventKind::Down(
                value.button.try_into().unwrap_or(MouseButton::Left),
            ),
            TermalMouseEvent::Up => MouseEventKind::Up(
                value.button.try_into().unwrap_or(MouseButton::Left),
            ),
            TermalMouseEvent::ScrollUp => MouseEventKind::ScrollUp,
            TermalMouseEvent::ScrollDown => MouseEventKind::ScrollDown,
            TermalMouseEvent::Move => MouseEventKind::Move,
        };
        Self {
            kind,
            modifiers: value.modifiers.into(),
            pos: Vec2::new(value.x, value.y),
        }
    }
}

impl TryFrom<TermalButton> for MouseButton {
    type Error = &'static str;

    fn try_from(value: TermalButton) -> Result<Self, Self::Error> {
        match value {
            TermalButton::None => Err("unknown button variant"),
            TermalButton::Left => Ok(MouseButton::Left),
            TermalButton::Middle => Ok(MouseButton::Middle),
            TermalButton::Right => Ok(MouseButton::Right),
            TermalButton::Button4 => Ok(MouseButton::Button4),
            TermalButton::Button5 => Ok(MouseButton::Button5),
            TermalButton::Button6 => Ok(MouseButton::Button6),
            TermalButton::Button7 => Ok(MouseButton::Button7),
            TermalButton::Other(v) => Ok(MouseButton::Other(v)),
        }
    }
}

impl TryFrom<Status> for Event {
    type Error = &'static str;

    fn try_from(value: Status) -> Result<Self, Self::Error> {
        match value {
            Status::TextAreaSize { w, h } => Ok(Event::Resize(w, h)),
            _ => Err("unsupported event type"),
        }
    }
}
