use std::fmt;

/// Enum for modifier ANSI codes
///
/// You can use macro to get vector with Modifers:
/// ```rust
/// # use termint::{enums::modifier::Modifier, modifiers};
/// // Gets vector with Bold and Italic modifier
/// let modifiers = modifiers!(Bold, Italic);
/// ```
#[derive(Debug, PartialEq)]
pub enum Modifier {
    /// Bold mode
    Bold,
    /// Dim/faint mode
    Dim,
    /// Italic mode
    Italic,
    /// Underline mode
    Underline,
    /// Blinking mode
    Blink,
    /// Inverse/reverse mode
    Inverse,
    /// Hidden/invisible mode
    Hidden,
    /// Strikethrough mode
    Strike,
}

impl Modifier {
    /// Converts [`Modifier`] to ANSI code
    pub fn to_ansi(&self) -> &'static str {
        match self {
            Modifier::Bold => "\x1b[1m",
            Modifier::Dim => "\x1b[2m",
            Modifier::Italic => "\x1b[3m",
            Modifier::Underline => "\x1b[4m",
            Modifier::Blink => "\x1b[5m",
            Modifier::Inverse => "\x1b[7m",
            Modifier::Hidden => "\x1b[8m",
            Modifier::Strike => "\x1b[9m",
        }
    }
}

impl fmt::Display for Modifier {
    /// Automatically converts [`Modifier`] to ANSI code when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}

/// Creates vector with given [`Modifier`]
#[macro_export]
macro_rules! modifiers {
    ($($mod:ident),*) => {
        vec![$(Modifier::$mod, )*]
    };
}
