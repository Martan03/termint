use std::fmt;

/// Background colors enum
pub enum Bg {
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
    RGB(usize, usize, usize),
    Default,
}

impl Bg {
    /// Gets background color ANSI code
    pub fn to_ansi(&self) -> String {
        match self {
            Bg::Black => "\x1b[40m".to_string(),
            Bg::Red => "\x1b[41m".to_string(),
            Bg::Green => "\x1b[42m".to_string(),
            Bg::Yellow => "\x1b[43m".to_string(),
            Bg::Blue => "\x1b[44m".to_string(),
            Bg::Magenta => "\x1b[45m".to_string(),
            Bg::Cyan => "\x1b[46m".to_string(),
            Bg::White => "\x1b[47m".to_string(),
            Bg::BrightBlack => "\x1b[100m".to_string(),
            Bg::BrightRed => "\x1b[101m".to_string(),
            Bg::BrightGreen => "\x1b[102m".to_string(),
            Bg::BrightYellow => "\x1b[103m".to_string(),
            Bg::BrightBlue => "\x1b[104m".to_string(),
            Bg::BrightMagenta => "\x1b[105m".to_string(),
            Bg::BrightCyan => "\x1b[106m".to_string(),
            Bg::BrightWhite => "\x1b[107m".to_string(),
            Bg::RGB(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
            Bg::Default => "\x1b[49m".to_string(),
        }
    }
}

impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}
