use std::fmt;

/// Foreground colors enum
pub enum Fg {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Default,
}

impl Fg {
    /// Gets foreground ANSI color code
    pub fn to_ansi(&self) -> &'static str {
        match self {
            Fg::Black => "\x1b[30m",
            Fg::Red => "\x1b[31m",
            Fg::Green => "\x1b[32m",
            Fg::Yellow => "\x1b[33m",
            Fg::Blue => "\x1b[34m",
            Fg::Magenta => "\x1b[35m",
            Fg::Cyan => "\x1b[36m",
            Fg::White => "\x1b[37m",
            Fg::BrightBlack => "\x1b[90m",
            Fg::BrightRed => "\x1b[91m",
            Fg::BrightGreen => "\x1b[92m",
            Fg::BrightYellow => "\x1b[93m",
            Fg::BrightBlue => "\x1b[94m",
            Fg::BrightMagenta => "\x1b[95m",
            Fg::BrightCyan => "\x1b[96m",
            Fg::BrightWhite => "\x1b[97m",
            Fg::Default => "\x1b[39m",
        }
    }
}

impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}
