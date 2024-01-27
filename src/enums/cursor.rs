use std::fmt;

/// Cursor ANSI Codes
#[derive(Debug)]
pub enum Cursor {
    /// Moves cursor to home position (0, 0)
    Home,
    /// Sets cursor position to given coordinates (x, y)
    Pos(usize, usize),
    /// Moves cursor up by given number
    Up(usize),
    /// Moves cursor down by given number
    Down(usize),
    /// Moves cursor right by given number
    Right(usize),
    /// Moves cursor left by given number
    Left(usize),
    /// Moves cursor down by given number and to the beginning
    NextBeg(usize),
    /// Moves cursor up by given number and to the beginning
    PrevBeg(usize),
    /// Moves cursor to column given by given number
    Col(usize),
}

impl Cursor {
    /// Converts [`Cursor`] to coresponding ANSI code
    pub fn to_ansi(&self) -> String {
        match self {
            Cursor::Home => "\x1b[H".to_string(),
            Cursor::Pos(x, y) => format!("\x1b[{};{}H", y, x),
            Cursor::Up(n) => format!("\x1b[{n}A"),
            Cursor::Down(n) => format!("\x1b[{n}B"),
            Cursor::Right(n) => format!("\x1b[{n}C"),
            Cursor::Left(n) => format!("\x1b[{n}D"),
            Cursor::NextBeg(n) => format!("\x1b[{n}E"),
            Cursor::PrevBeg(n) => format!("\x1b[{n}F"),
            Cursor::Col(n) => format!("\x1b[{n}G"),
        }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}
