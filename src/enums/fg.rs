use std::fmt;

/// Enum for foreground colors
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Fg {
    Black,
    DarkRed,
    DarkGreen,
    DarkYellow,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
    LightGray,
    Gray,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    RGB(u8, u8, u8),
    #[default]
    Default,
}

impl Fg {
    /// Gets foreground ANSI color code
    pub fn to_ansi(&self) -> String {
        match self {
            Fg::Black => "\x1b[30m".to_string(),
            Fg::DarkRed => "\x1b[31m".to_string(),
            Fg::DarkGreen => "\x1b[32m".to_string(),
            Fg::DarkYellow => "\x1b[33m".to_string(),
            Fg::DarkBlue => "\x1b[34m".to_string(),
            Fg::DarkMagenta => "\x1b[35m".to_string(),
            Fg::DarkCyan => "\x1b[36m".to_string(),
            Fg::LightGray => "\x1b[37m".to_string(),
            Fg::Gray => "\x1b[90m".to_string(),
            Fg::Red => "\x1b[91m".to_string(),
            Fg::Green => "\x1b[92m".to_string(),
            Fg::Yellow => "\x1b[93m".to_string(),
            Fg::Blue => "\x1b[94m".to_string(),
            Fg::Magenta => "\x1b[95m".to_string(),
            Fg::Cyan => "\x1b[96m".to_string(),
            Fg::White => "\x1b[97m".to_string(),
            Fg::RGB(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
            Fg::Default => "\x1b[39m".to_string(),
        }
    }
}

impl fmt::Display for Fg {
    /// Automatically converts [`Fg`] to ANSI code when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}
