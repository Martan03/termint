use std::fmt::Display;

use compact_str::CompactString;

use crate::{
    enums::{Color, Modifier},
    style::Style,
};

/// A buffer cell containing foreground, background, modifiers and symbol.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
    pub val: CompactString,
}

impl Cell {
    /// Creates new [`Cell`] with given value
    pub fn new(val: &'static str) -> Self {
        Self {
            val: CompactString::const_new(val),
            ..Default::default()
        }
    }

    /// Creates empty [`Cell`]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Sets value of the [`Cell`]
    pub fn val(&mut self, val: &str) -> &mut Self {
        self.val = CompactString::new(val);
        self
    }

    /// Sets [`Cell`] foreground color to given value
    pub fn fg(&mut self, fg: Color) -> &mut Self {
        self.fg = fg;
        self
    }

    /// Sets [`Cell`] background color to given value
    pub fn bg(&mut self, bg: Color) -> &mut Self {
        self.bg = bg;
        self
    }

    /// Sets [`Cell`] modifier to the given flag
    pub fn modifier(&mut self, flag: Modifier) -> &mut Self {
        self.modifier = Modifier::empty();
        self.modifier.insert(flag);
        self
    }

    /// Sets style of the [`Cell`] to the given value. If `fg` or `bg` are
    /// none, it keeps the original value.
    pub fn style<T>(&mut self, style: T) -> &mut Self
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
        self.val = CompactString::const_new(" ");
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
            val: CompactString::const_new(" "),
        }
    }
}
