use std::fmt;

use crate::enums::{bg::Bg, fg::Fg};

pub struct Span {
    text: String,
    fg: Fg,
    bg: Bg,
}

impl Span {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            fg: Fg::Default,
            bg: Bg::Default,
        }
    }

    pub fn get(&self) -> String {
        format!(
            "{}{}{}\x1b[0m",
            self.fg.to_ansi(), self.bg.to_ansi(), self.text
        )
    }

    pub fn fg(mut self, fg: Fg) -> Self {
        self.fg = fg;
        self
    }

    pub fn bg(mut self, bg: Bg) -> Self {
        self.bg = bg;
        self
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}
