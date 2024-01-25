use std::fmt;

use crate::enums::{bg::Bg, fg::Fg, modifier::Modifier};

pub struct Span {
    text: String,
    fg: Fg,
    bg: Bg,
    modifier: Vec<Modifier>,
}

impl Span {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            fg: Fg::Default,
            bg: Bg::Default,
            modifier: Vec::new(),
        }
    }

    pub fn get(&self) -> String {
        let m = self.modifier
            .iter()
            .map(|m| m.to_ansi())
            .collect::<Vec<&str>>()
            .join("");
        format!(
            "{}{}{}{}\x1b[0m",
            self.fg, self.bg, m, self.text
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

    pub fn modifier(mut self, mods: Vec<Modifier>) -> Self {
        self.modifier = mods;
        self
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}
