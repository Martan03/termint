use std::fmt::Display;

use crate::enums::{Color, Modifier};

/// Style struct containing foreground, background and modifiers
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub modifier: Modifier,
}

impl Style {
    /// Creates new [`Style`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the [`Style`] to default style
    pub fn reset(&mut self) {
        self.fg = None;
        self.bg = None;
        self.modifier.clear();
    }

    /// Sets foreground color to given value
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.fg = fg.into();
        self
    }

    /// Sets background color to given value
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.bg = bg.into();
        self
    }

    /// Sets modifier to the given flag
    pub fn modifier(mut self, flag: u8) -> Self {
        self.modifier.clear();
        self.modifier.add(flag);
        self
    }

    /// Adds given modifier to the already set modifiers
    pub fn add_modifier(mut self, flag: u8) -> Self {
        self.modifier.add(flag);
        self
    }

    /// Removes given modifier from the already set modifiers
    pub fn remove_modifier(mut self, flag: u8) -> Self {
        self.modifier.sub(flag);
        self
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.modifier)?;
        if let Some(fg) = self.fg {
            write!(f, "{}", fg.to_fg())?;
        }
        if let Some(bg) = self.bg {
            write!(f, "{}", bg.to_bg())?;
        }
        Ok(())
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            modifier: Modifier::empty(),
        }
    }
}
