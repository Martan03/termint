use std::fmt;

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
    Default,
}

impl Bg {
    pub fn to_ansi(&self) -> &'static str {
        match self {
            Bg::Black => "\x1b[40m",
            Bg::Red => "\x1b[41m",
            Bg::Green => "\x1b[42m",
            Bg::Yellow => "\x1b[43m",
            Bg::Blue => "\x1b[44m",
            Bg::Magenta => "\x1b[45m",
            Bg::Cyan => "\x1b[46m",
            Bg::White => "\x1b[47m",
            Bg::BrightBlack => "\x1b[100m",
            Bg::BrightRed => "\x1b[101m",
            Bg::BrightGreen => "\x1b[102m",
            Bg::BrightYellow => "\x1b[103m",
            Bg::BrightBlue => "\x1b[104m",
            Bg::BrightMagenta => "\x1b[105m",
            Bg::BrightCyan => "\x1b[106m",
            Bg::BrightWhite => "\x1b[107m",
            Bg::Default => "\x1b[49m",
        }
    }
}

impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}
