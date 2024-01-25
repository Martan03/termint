pub enum Bg {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
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
            Bg::Default => "\x1b[49m",
        }
    }
}
