use std::fmt;

/// Enum for background colors
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Bg {
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
    RGB(usize, usize, usize),
    #[default]
    Default,
}

impl Bg {
    /// Gets background color ANSI code
    pub fn to_ansi(&self) -> String {
        match self {
            Bg::Black => "\x1b[40m".to_string(),
            Bg::DarkRed => "\x1b[41m".to_string(),
            Bg::DarkGreen => "\x1b[42m".to_string(),
            Bg::DarkYellow => "\x1b[43m".to_string(),
            Bg::DarkBlue => "\x1b[44m".to_string(),
            Bg::DarkMagenta => "\x1b[45m".to_string(),
            Bg::DarkCyan => "\x1b[46m".to_string(),
            Bg::LightGray => "\x1b[47m".to_string(),
            Bg::Gray => "\x1b[100m".to_string(),
            Bg::Red => "\x1b[101m".to_string(),
            Bg::Green => "\x1b[102m".to_string(),
            Bg::Yellow => "\x1b[103m".to_string(),
            Bg::Blue => "\x1b[104m".to_string(),
            Bg::Magenta => "\x1b[105m".to_string(),
            Bg::Cyan => "\x1b[106m".to_string(),
            Bg::White => "\x1b[107m".to_string(),
            Bg::RGB(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
            Bg::Default => "\x1b[49m".to_string(),
        }
    }
}

impl fmt::Display for Bg {
    /// Automatically converts [`Bg`] to ANSI code when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}
