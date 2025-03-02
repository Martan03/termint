use std::fmt::Display;

use crate::{
    enums::{Color, Modifier},
    style::Style,
};

/// A buffer cell containing foreground, background, modifiers and symbol.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
    pub val: char,
}

impl Cell {
    /// Creates new [`Cell`] with given value
    pub fn new(val: char) -> Self {
        Self {
            val,
            ..Default::default()
        }
    }

    /// Creates empty [`Cell`]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Sets value of the [`Cell`]
    pub fn val(mut self, val: char) -> Self {
        self.val = val;
        self
    }

    /// Sets [`Cell`] foreground color to given value
    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    /// Sets [`Cell`] background color to given value
    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    /// Sets [`Cell`] modifier to the given flag
    pub fn modifier(mut self, flag: u8) -> Self {
        self.modifier.clear();
        self.modifier.add(flag);
        self
    }

    /// Sets style of the [`Cell`] to the given value. If `fg` or `bg` are
    /// none, it keeps the original value.
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        let style = style.into();
        if let Some(fg) = style.fg {
            self.fg = fg;
        }
        if let Some(bg) = style.bg {
            self.bg = bg;
        }
        self.modifier = style.modifier;
        self
    }

    /// Resets the [`Cell`] to defalt values
    pub fn reset(&mut self) {
        self.fg = Color::Default;
        self.bg = Color::Default;
        self.modifier = Modifier::empty();
        self.val = ' ';
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.modifier,
            self.fg.to_fg(),
            self.bg.to_bg(),
            self.val
        )
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            fg: Color::Default,
            bg: Color::Default,
            modifier: Modifier::empty(),
            val: ' ',
        }
    }
}
