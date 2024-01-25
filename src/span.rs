use crate::enums::{bg::Bg, fg::Fg};

pub struct Span {
    text: String,
    fg: Fg,
    bg: Bg,
}

impl Span {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
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

    pub fn fg(&mut self, fg: Fg) -> &mut Self {
        self.fg = fg;
        self
    }

    pub fn bg(&mut self, bg: Bg) -> &mut Self {
        self.bg = bg;
        self
    }
}
