pub enum Fg {
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

impl Fg {
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
            Fg::Default => "\x1b[39m",
        }
    }
}
