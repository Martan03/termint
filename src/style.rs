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
    /// Creates a new [`Style`]
    #[must_use]
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
    #[must_use]
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.fg = fg.into();
        self
    }

    /// Sets background color to given value
    #[must_use]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.bg = bg.into();
        self
    }

    /// Sets modifier to the given flag
    #[must_use]
    pub fn modifier(mut self, flag: u8) -> Self {
        self.modifier.clear();
        self.modifier.add(flag);
        self
    }

    /// Adds given modifier to the already set modifiers
    #[must_use]
    pub fn add_modifier(mut self, flag: u8) -> Self {
        self.modifier.add(flag);
        self
    }

    /// Removes given modifier from the already set modifiers
    #[must_use]
    pub fn remove_modifier(mut self, flag: u8) -> Self {
        self.modifier.sub(flag);
        self
    }

    /// Combines two styles, equivalent to applying two styles after each
    /// other. Only modifier gets overriden.
    ///
    /// You can provide any type convertible to [`Style`].
    #[must_use]
    pub fn combine<S>(mut self, other: S) -> Self
    where
        S: Into<Style>,
    {
        let other = other.into();
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);
        self.modifier = other.modifier;
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

impl From<Color> for Style {
    /// Creates a new [`Style`] with given foreground color
    fn from(value: Color) -> Self {
        Self::new().fg(value)
    }
}

impl From<(Color, Color)> for Style {
    /// Creates a new [`Style`] with given foreground and background color
    fn from((fg, bg): (Color, Color)) -> Self {
        Self::new().fg(fg).bg(bg)
    }
}

impl From<Modifier> for Style {
    /// Creates a new [`Style`] with given modifier
    fn from(value: Modifier) -> Self {
        Self::new().modifier(value.val())
    }
}

impl From<(Color, Color, Modifier)> for Style {
    /// Creates a new [`Style`] with given foreground and background color and
    /// with given modifier
    fn from((fg, bg, modifier): (Color, Color, Modifier)) -> Self {
        Self::new().fg(fg).bg(bg).modifier(modifier.val())
    }
}
