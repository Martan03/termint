use core::fmt;

use crate::{
    enums::{
        bg::Bg, cursor::Cursor, fg::Fg, modifier::Modifier, rgb::RGB,
        wrap::Wrap,
    },
    geometry::coords::Coords,
};

use super::widget::Widget;

/// Text with gradient foreground
pub struct Grad {
    text: String,
    fg_start: RGB,
    fg_end: RGB,
    bg: Bg,
    modifier: Vec<Modifier>,
    wrap: Wrap,
}

impl Grad {
    /// Creates new [`Grad`] with given text
    pub fn new<T: Into<String>, R: Into<RGB>>(
        text: T,
        start: R,
        end: R,
    ) -> Self {
        Self {
            text: text.into(),
            fg_start: start.into(),
            fg_end: end.into(),
            bg: Bg::Default,
            modifier: Vec::new(),
            wrap: Wrap::Letter,
        }
    }

    /// Gets [`Grad`] as string
    pub fn get(&self) -> String {
        let len = self.text.len() as i16;
        let (r_step, g_step, b_step): (i16, i16, i16) = (
            (self.fg_end.r as i16 - self.fg_start.r as i16) / len,
            (self.fg_end.g as i16 - self.fg_start.g as i16) / len,
            (self.fg_end.b as i16 - self.fg_start.b as i16) / len,
        );
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);

        println!("{}", self.get_ansi());

        let mut res = "".to_string();
        for c in self.text.chars() {
            res += &format!("{}{c}", Fg::RGB(r, g, b));
            (r, g, b) = (
                (r as i16 + r_step) as u8,
                (g as i16 + g_step) as u8,
                (b as i16 + b_step) as u8,
            );
        }
        res += "\x1b[0m";

        return res;
    }

    /// Sets background of [`Grad`] to given color
    pub fn bg(mut self, bg: Bg) -> Self {
        self.bg = bg;
        self
    }

    /// Sets modifiers of [`Grad`] to given modifiers
    pub fn modifier(mut self, modifier: Vec<Modifier>) -> Self {
        self.modifier = modifier;
        self
    }

    /// Gets ANSI codes to set bg and modifiers properties
    fn get_ansi(&self) -> String {
        let m = self
            .modifier
            .iter()
            .map(|m| m.to_ansi())
            .collect::<Vec<&str>>()
            .join("");
        format!("{}{}", self.bg, m)
    }

    /// Renders [`Grad`] with word wrapping
    fn render_word_wrap(&self, _pos: &Coords, _size: &Coords) {
        todo!();
    }

    /// Renders [`Grad`] with letter wrapping
    fn render_letter_wrap(&self, pos: &Coords, size: &Coords) {
        let len = size.x as i16;
        let (r_step, g_step, b_step): (i16, i16, i16) = (
            (self.fg_end.r as i16 - self.fg_start.r as i16) / len,
            (self.fg_end.g as i16 - self.fg_start.g as i16) / len,
            (self.fg_end.b as i16 - self.fg_start.b as i16) / len,
        );
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);

        let chars = size.x * size.y;

        for (i, c) in self.text.chars().enumerate() {
            if i >= chars {
                break;
            }

            if i % size.x == 0 {
                print!("{}", Cursor::Pos(pos.x, pos.y + i / size.x));
                (r, g, b) =
                    (self.fg_start.r, self.fg_start.g, self.fg_start.b);
            }

            print!("{}{c}", Fg::RGB(r, g, b));
            (r, g, b) = (
                (r as i16 + r_step) as u8,
                (g as i16 + g_step) as u8,
                (b as i16 + b_step) as u8,
            );
        }
    }
}

impl Widget for Grad {
    fn render(&self, pos: &Coords, size: &Coords) {
        print!("{}", self.get_ansi());

        match self.wrap {
            Wrap::Letter => self.render_letter_wrap(pos, size),
            Wrap::Word => self.render_word_wrap(pos, size),
        }

        println!("\x1b[0m");
    }
}

impl fmt::Display for Grad {
    /// Automatically converts [`Grad`] to String when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}
