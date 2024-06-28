use std::fmt::Display;

use crate::enums::Color;

/// Represents rendering buffer cell
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    fg: Color,
    bg: Color,
    val: char,
}

impl Cell {
    /// Creates new [`Cell`] with given value
    pub fn new(val: char) -> Self {
        Self {
            val,
            ..Default::default()
        }
    }

    /// Sets [`Cell`] foreground color to given value
    pub fn fg(&mut self, fg: Color) {
        self.fg = fg;
    }

    /// Sets [`Cell`] background color to given value
    pub fn bg(&mut self, bg: Color) {
        self.bg = bg;
    }

    /// Sets value of the [`Cell`]
    pub fn val(&mut self, val: char) {
        self.val = val;
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.fg.to_fg(), self.bg.to_bg(), self.val)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            val: ' ',
        }
    }
}
