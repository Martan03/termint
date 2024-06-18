use std::fmt::Display;

use crate::enums::{bg::Bg, fg::Fg};

/// Represents rendering buffer cell
#[derive(Debug, Clone)]
pub struct Cell {
    fg: Option<Fg>,
    bg: Option<Bg>,
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
    pub fn fg<T>(&mut self, fg: T)
    where
        T: Into<Option<Fg>>,
    {
        self.fg = fg.into();
    }

    /// Sets [`Cell`] background color to given value
    pub fn bg<T>(&mut self, bg: T)
    where
        T: Into<Option<Bg>>,
    {
        self.bg = bg.into();
    }

    /// Sets value of the [`Cell`]
    pub fn val(&mut self, val: char) {
        self.val = val;
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(fg) = self.fg {
            write!(f, "{}", fg)?
        }
        if let Some(bg) = self.bg {
            write!(f, "{}", bg)?;
        }
        write!(f, "{}", self.val)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            fg: Default::default(),
            bg: None,
            val: ' ',
        }
    }
}
