use std::fmt;

/// ANSI cursor manipulation
#[derive(Debug, PartialEq, Clone, Copy)]
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

impl fmt::Display for Cursor {
    /// Automatically converts [`Cursor`] to ANSI code when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cursor::Home => write!(f, "\x1b[H"),
            Cursor::Pos(x, y) => write!(f, "\x1b[{};{}H", y, x),
            Cursor::Up(n) => write!(f, "\x1b[{n}A"),
            Cursor::Down(n) => write!(f, "\x1b[{n}B"),
            Cursor::Right(n) => write!(f, "\x1b[{n}C"),
            Cursor::Left(n) => write!(f, "\x1b[{n}D"),
            Cursor::NextBeg(n) => write!(f, "\x1b[{n}E"),
            Cursor::PrevBeg(n) => write!(f, "\x1b[{n}F"),
            Cursor::Col(n) => write!(f, "\x1b[{n}G"),
        }
    }
}
