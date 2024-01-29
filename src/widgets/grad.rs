use core::fmt;
use std::cmp::min;

use crate::{
    enums::{
        bg::Bg, cursor::Cursor, fg::Fg, modifier::Modifier, rgb::RGB,
        wrap::Wrap,
    },
    geometry::{coords::Coords, direction::Direction},
};

use super::widget::Widget;

/// Text with gradient foreground
///
/// ## Example usage:
/// ```
/// # use termint::{
/// #     geometry::coords::Coords,
/// #     widgets::{grad::Grad, widget::Widget},
/// # };
/// let grad = Grad::new(
///     "This text will have a gradient foreground and word wrap",
///     (0, 220, 255),
///     (200, 60, 255),
/// );
/// grad.render(&Coords::new(1, 1), &Coords::new(10, 5));
/// ```
pub struct Grad {
    text: String,
    fg_start: RGB,
    fg_end: RGB,
    direction: Direction,
    bg: Bg,
    modifier: Vec<Modifier>,
    wrap: Wrap,
    ellipsis: String,
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
            direction: Direction::Horizontal,
            bg: Bg::Default,
            modifier: Vec::new(),
            wrap: Wrap::Word,
            ellipsis: "...".to_string(),
        }
    }

    /// Gets [`Grad`] as string
    pub fn get(&self) -> String {
        let step = self.get_step(self.text.len() as i16 - 1);
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);

        let mut res = self.get_ansi();
        for c in self.text.chars() {
            res += &format!("{}{c}", Fg::RGB(r, g, b));
            (r, g, b) = self.add_step((r, g, b), step);
        }
        res += "\x1b[0m";

        return res;
    }

    /// Sets gradient direction of [`Grad`]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
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

    /// Sets wrap of [`Grad`] to given value
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets [`Grad`] ellipsis to given string
    pub fn ellipsis(mut self, ellipsis: &str) -> Self {
        self.ellipsis = ellipsis.to_string();
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
    fn render_word_wrap(&self, pos: &Coords, size: &Coords) {
        match self.direction {
            Direction::Vertical => self.render_word_wrap_ver(pos, size),
            Direction::Horizontal => self.render_word_wrap_hor(pos, size),
        }
    }

    /// Renders [`Grad`] with word wrapping and horizontal gradient
    fn render_word_wrap_hor(&self, pos: &Coords, size: &Coords) {
        let width = min(size.x, self.text.len());
        let step = self.get_step(width as i16);
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);

        let mut coords = Coords::new(0, pos.y);
        print!("{}", Cursor::Pos(pos.x, pos.y));

        let words: Vec<&str> = self.text.split_whitespace().collect();
        for word in words {
            let print_str = if coords.x == 0 {
                format!("{word}")
            } else {
                format!(" {word}")
            };

            if coords.x + print_str.len() <= size.x {
                for c in print_str.chars() {
                    print!("{}{c}", Fg::RGB(r, g, b));
                    (r, g, b) = self.add_step((r, g, b), step);
                }
                coords.x += print_str.len();
                continue;
            }

            coords.y += 1;
            if coords.y >= pos.y + size.y || print_str.len() > size.x {
                self.render_ellipsis_hor(&coords, size, (r, g, b), step);
                break;
            }

            coords.x = 0;
            print!("{}", Cursor::Pos(pos.x, coords.y));
            (r, g, b) = (self.fg_start.r, self.fg_start.g, self.fg_start.b);
        }
    }

    /// Renders [`Grad`] ellipsis when horizontal direction and word wrap
    fn render_ellipsis_hor(
        &self,
        coords: &Coords,
        size: &Coords,
        rgb: (u8, u8, u8),
        step: (i16, i16, i16),
    ) {
        let (mut r, mut g, mut b) = (rgb.0, rgb.1, rgb.2);
        let sum = coords.x + self.ellipsis.len();
        if sum > size.x {
            if size.x < self.ellipsis.len() {
                return;
            }

            print!("{}", Cursor::Left(sum - size.x));
        }
        for c in self.ellipsis.chars() {
            print!("{}{c}", Fg::RGB(r, g, b));
            (r, g, b) = self.add_step((r, g, b), step);
        }
    }

    /// Renders [`Grad`] with word wrapping and vertical gradient
    fn render_word_wrap_ver(&self, pos: &Coords, size: &Coords) {
        let height = min(self.height_word_wrap(size) - 1, size.y);
        let step = self.get_step(height as i16);
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);

        let mut coords = Coords::new(0, pos.y);
        print!("{}", Cursor::Pos(pos.x, pos.y));

        let words: Vec<&str> = self.text.split_whitespace().collect();
        for word in words {
            let len = word.len();
            if coords.x + len > size.x {
                coords.y += 1;

                if coords.y >= pos.y + size.y || len > size.x {
                    break;
                }

                coords.x = 0;
                print!("{}", Cursor::Pos(pos.x, coords.y));
                (r, g, b) = self.add_step((r, g, b), step);
            }

            if coords.x != 0 {
                print!(" ");
                coords.x += 1;
            }

            for c in word.chars() {
                print!("{}{c}", Fg::RGB(r, g, b));
            }
            coords.x += len;
        }
    }

    /// Renders [`Grad`] with letter wrapping
    fn render_letter_wrap(&self, pos: &Coords, size: &Coords) {
        match self.direction {
            Direction::Vertical => self.render_letter_wrap_ver(pos, size),
            Direction::Horizontal => self.render_letter_wrap_hor(pos, size),
        }
    }

    /// Renders [`Grad`] with letter wrapping and horizontal gradient
    fn render_letter_wrap_hor(&self, pos: &Coords, size: &Coords) {
        let width = min(size.x, self.text.len());
        let step = self.get_step(width as i16);
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
            (r, g, b) = self.add_step((r, g, b), step);
        }
    }

    /// Renders [`Grad`] with letter wrapping and vertical gradient
    fn render_letter_wrap_ver(&self, pos: &Coords, size: &Coords) {
        let height = min(self.height_letter_wrap(size) - 1, size.y);
        let step = self.get_step(height as i16);
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);

        let chars = size.x * size.y;

        for (i, c) in self.text.chars().enumerate() {
            if i >= chars {
                break;
            }

            if i % size.x == 0 {
                print!("{}", Cursor::Pos(pos.x, pos.y + i / size.x));
                (r, g, b) = self.add_step((r, g, b), step);
            }

            print!("{}{c}", Fg::RGB(r, g, b));
        }
    }

    /// Gets step per character based on start and end foreground color
    fn get_step(&self, len: i16) -> (i16, i16, i16) {
        (
            (self.fg_end.r as i16 - self.fg_start.r as i16) / len,
            (self.fg_end.g as i16 - self.fg_start.g as i16) / len,
            (self.fg_end.b as i16 - self.fg_start.b as i16) / len,
        )
    }

    /// Adds given step to RGB value in tuple
    fn add_step(
        &self,
        rgb: (u8, u8, u8),
        step: (i16, i16, i16),
    ) -> (u8, u8, u8) {
        (
            (rgb.0 as i16 + step.0) as u8,
            (rgb.1 as i16 + step.1) as u8,
            (rgb.2 as i16 + step.2) as u8,
        )
    }

    /// Gets height of the [`Grad`] when using word wrap
    fn height_word_wrap(&self, size: &Coords) -> usize {
        let mut coords = Coords::new(0, 0);

        let words: Vec<&str> = self.text.split_whitespace().collect();
        for word in words {
            let len = word.len();
            if coords.x + len + 1 > size.x {
                coords.y += 1;
                coords.x = 0;
            }

            if coords.x != 0 {
                coords.x += 1;
            }
            coords.x += len;
        }
        coords.y + 1
    }

    /// Gets height of the [`Grad`] when using letter wrap
    fn height_letter_wrap(&self, size: &Coords) -> usize {
        (self.text.len() as f32 / size.x as f32).ceil() as usize
    }

    /// Gets width of the [`Grad`] when using word wrap
    fn width_word_wrap(&self, size: &Coords) -> usize {
        let mut guess = Coords::new(self.width_letter_wrap(size), 0);

        while self.height_word_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets width of the [`Grad`] when using letter wrap
    fn width_letter_wrap(&self, size: &Coords) -> usize {
        (self.text.len() as f32 / size.y as f32).ceil() as usize
    }
}

impl Widget for Grad {
    fn render(&self, pos: &Coords, size: &Coords) {
        if size.x == 0 || size.y == 0 {
            return;
        }
        print!("{}", self.get_ansi());

        match self.wrap {
            Wrap::Letter => self.render_letter_wrap(pos, size),
            Wrap::Word => self.render_word_wrap(pos, size),
        }

        println!("\x1b[0m");
    }

    fn height(&self, size: &Coords) -> usize {
        match self.wrap {
            Wrap::Letter => self.height_letter_wrap(size),
            Wrap::Word => self.height_word_wrap(size),
        }
    }

    fn width(&self, size: &Coords) -> usize {
        match self.wrap {
            Wrap::Letter => self.width_letter_wrap(size),
            Wrap::Word => self.width_word_wrap(size),
        }
    }
}

impl fmt::Display for Grad {
    /// Automatically converts [`Grad`] to String when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}
