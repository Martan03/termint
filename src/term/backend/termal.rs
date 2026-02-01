use std::time::Duration;

use termal::raw::events::Status;
use termal::raw::events::{
    mouse::Button as TermalButton, mouse::Event as TermalMouseEvent,
    mouse::Mouse, Event as TermalEvent, Key, KeyCode as TermalKeyCode,
    Modifiers as TermalMod,
};
use termal::raw::{StdioProvider, Terminal};

use crate::term::backend::event::{
    Button, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
    MouseEvent, MouseEventKind,
};
use crate::{
    term::backend::{Backend, Event},
    Error,
};

#[derive(Debug, Default)]
pub struct TermalBackend(pub(crate) Terminal<StdioProvider>);

impl Backend for TermalBackend {
    fn read_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Event>, Error> {
        let event = self
            .0
            .read_timeout(timeout)?
            .map(TryInto::try_into)
            .transpose()
            .unwrap_or(None);
        Ok(event)
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
                value.button.try_into().unwrap_or(Button::Left),
            ),
            TermalMouseEvent::Up => MouseEventKind::Up(
                value.button.try_into().unwrap_or(Button::Left),
            ),
            TermalMouseEvent::ScrollUp => MouseEventKind::ScrollUp,
            TermalMouseEvent::ScrollDown => MouseEventKind::ScrollDown,
            TermalMouseEvent::Move => MouseEventKind::Move,
        };
        Self {
            kind,
            modifiers: value.modifiers.into(),
            x: value.x,
            y: value.y,
        }
    }
}

impl TryFrom<TermalButton> for Button {
    type Error = &'static str;

    fn try_from(value: TermalButton) -> Result<Self, Self::Error> {
        match value {
            TermalButton::None => Err("unknown button variant"),
            TermalButton::Left => Ok(Button::Left),
            TermalButton::Middle => Ok(Button::Middle),
            TermalButton::Right => Ok(Button::Right),
            TermalButton::Button4 => Ok(Button::Button4),
            TermalButton::Button5 => Ok(Button::Button5),
            TermalButton::Button6 => Ok(Button::Button6),
            TermalButton::Button7 => Ok(Button::Button7),
            TermalButton::Other(v) => Ok(Button::Other(v)),
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
