use std::fmt;

pub enum Modifier {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strike,
}

impl Modifier {
    pub fn to_ansi(&self) -> &'static str {
        match self {
            Modifier::Bold => "\x1b[1m",
            Modifier::Dim => "\x1b[2m",
            Modifier::Italic => "\x1b[3m",
            Modifier::Underline => "\x1b[4m",
            Modifier::Blink => "\x1b[5m",
            Modifier::Reverse => "\x1b[7m",
            Modifier::Hidden => "\x1b[8m",
            Modifier::Strike => "\x1b[9m",
        }
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}
