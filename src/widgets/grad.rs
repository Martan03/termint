use core::fmt;
use std::{
    cmp::{max, min},
    io::{stdout, Write},
};

use crate::{
    enums::{
        bg::Bg, cursor::Cursor, fg::Fg, modifier::Modifier, rgb::RGB,
        wrap::Wrap,
    },
    geometry::{coords::Coords, direction::Direction, text_align::TextAlign},
};

use super::{text::Text, widget::Widget};

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
    bg: Option<Bg>,
    modifier: Vec<Modifier>,
    align: TextAlign,
    wrap: Wrap,
    ellipsis: String,
}

impl Grad {
    /// Creates new [`Grad`] with given text
    pub fn new<T, R, S>(text: T, start: R, end: S) -> Self
    where
        T: Into<String>,
        R: Into<RGB>,
        S: Into<RGB>,
    {
        Self {
            text: text.into(),
            fg_start: start.into(),
            fg_end: end.into(),
            direction: Direction::Horizontal,
            bg: None,
            modifier: Vec::new(),
            align: Default::default(),
            wrap: Wrap::Word,
            ellipsis: "...".to_string(),
        }
    }

    /// Sets gradient direction of [`Grad`]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets background of [`Grad`] to given color
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Bg>>,
    {
        self.bg = bg.into();
        self
    }

    /// Sets modifiers of [`Grad`] to given modifiers
    pub fn modifier(mut self, modifier: Vec<Modifier>) -> Self {
        self.modifier = modifier;
        self
    }

    /// Sets text alignment of the [`Grad`]
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
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
}

impl Widget for Grad {
    fn render(&self, pos: &Coords, size: &Coords) {
        print!("{}", self.get_string(pos, size));
        _ = stdout().flush();
    }

    fn get_string(&self, pos: &Coords, size: &Coords) -> String {
        if size.x == 0 || size.y == 0 {
            return String::new();
        }

        let (res, _) = match self.wrap {
            Wrap::Letter => self.render_letter_wrap(pos, size, 0),
            Wrap::Word => self.render_word_wrap(pos, size, 0),
        };

        format!("{}{res}\x1b[0m", self.get_mods())
    }

    fn height(&self, size: &Coords) -> usize {
        match self.wrap {
            Wrap::Letter => self.size_letter_wrap(size.x),
            Wrap::Word => self.height_word_wrap(size),
        }
    }

    fn width(&self, size: &Coords) -> usize {
        match self.wrap {
            Wrap::Letter => self.size_letter_wrap(size.y),
            Wrap::Word => self.width_word_wrap(size),
        }
    }
}

impl Text for Grad {
    fn render_offset(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
        wrap: Option<&Wrap>,
    ) -> Coords {
        let (res, coords) = self.get_offset(pos, size, offset, wrap);
        print!("{res}");
        coords
    }

    fn get_offset(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
        wrap: Option<&Wrap>,
    ) -> (String, Coords) {
        let wrap = wrap.unwrap_or(&self.wrap);
        let (res, coords) = match wrap {
            Wrap::Letter => self.render_letter_wrap(pos, size, offset),
            Wrap::Word => self.render_word_wrap(pos, size, offset),
        };
        (res, coords)
    }

    fn get(&self) -> String {
        let step = self.get_step(self.text.len() as i16 - 1);
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);

        let mut res = self.get_mods();
        for c in self.text.chars() {
            res += &format!("{}{c}", Fg::RGB(r, g, b));
            (r, g, b) = self.add_step((r, g, b), step);
        }
        res += "\x1b[0m";

        res
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_mods(&self) -> String {
        let m = self
            .modifier
            .iter()
            .map(|m| m.to_ansi())
            .collect::<Vec<&str>>()
            .join("");
        format!(
            "{}{}",
            self.bg.map_or_else(|| "".to_string(), |bg| bg.to_string()),
            m
        )
    }
}

impl fmt::Display for Grad {
    /// Automatically converts [`Grad`] to String when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl Grad {
    /// Renders [`Grad`] with word wrapping
    fn render_word_wrap(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
    ) -> (String, Coords) {
        match self.direction {
            Direction::Vertical => {
                let height = min(self.height_word_wrap(size) - 1, size.y);
                let step = self.get_step(height as i16);
                self.render_word(
                    pos,
                    size,
                    (0, 0, 0),
                    step,
                    |size, line, res, rgb, step| {
                        self.render_line_ver(size, line, res, rgb, step)
                    },
                    offset,
                )
            }
            Direction::Horizontal => {
                let width = min(size.x, self.text.len());
                let step = self.get_step(width as i16);
                self.render_word(
                    pos,
                    size,
                    step,
                    (0, 0, 0),
                    |size, line, res, rgb, step| {
                        self.render_line_hor(size, line, res, rgb, step)
                    },
                    offset,
                )
            }
        }
    }

    /// Renders [`Grad`] with letter wrapping
    fn render_letter_wrap(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
    ) -> (String, Coords) {
        match self.direction {
            Direction::Vertical => {
                let height = min(self.size_letter_wrap(size.x) - 1, size.y);
                let step = self.get_step(height as i16);
                self.render_letter(
                    pos,
                    size,
                    (0, 0, 0),
                    step,
                    |size, line, res, rgb, step| {
                        self.render_line_ver(size, line, res, rgb, step)
                    },
                    offset,
                )
            }
            Direction::Horizontal => {
                let width = min(size.x, self.text.len());
                let step = self.get_step(width as i16);
                self.render_letter(
                    pos,
                    size,
                    step,
                    (0, 0, 0),
                    |size, line, res, rgb, step| {
                        self.render_line_hor(size, line, res, rgb, step)
                    },
                    offset,
                )
            }
        }
    }

    fn render_word<F>(
        &self,
        pos: &Coords,
        size: &Coords,
        step_x: (i16, i16, i16),
        step_y: (i16, i16, i16),
        render_line: F,
        offset: usize,
    ) -> (String, Coords)
    where
        F: Fn(&Coords, String, &mut String, (u8, u8, u8), (i16, i16, i16)),
    {
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);
        for _ in 0..offset {
            (r, g, b) = self.add_step((r, g, b), step_x);
        }

        let mut res = Cursor::Pos(pos.x + offset, pos.y).to_string();
        let mut line: Vec<&str> = vec![];
        let mut coords = Coords::new(offset, pos.y);

        for word in self.text.split_whitespace() {
            if coords.x + word.len() + !line.is_empty() as usize > size.x {
                if coords.y + 1 >= pos.y + size.y || word.len() > size.x {
                    let mut line_str = line.join(" ");
                    let sum = coords.x + self.ellipsis.len();
                    if sum >= size.x {
                        let end = size.x.saturating_sub(self.ellipsis.len());
                        line_str = line_str[..end].to_string();
                    }

                    line_str.push_str(&self.ellipsis);
                    coords.x = line.len();
                    render_line(size, line_str, &mut res, (r, g, b), step_x);
                    return (res, coords);
                }

                (coords.x, coords.y) = (0, coords.y + 1);
                render_line(size, line.join(" "), &mut res, (r, g, b), step_x);
                res.push_str(&Cursor::Pos(pos.x, coords.y).to_string());
                (r, g, b) = self.add_step((r, g, b), step_y);
                line = vec![];
            }
            coords.x += word.len() + !line.is_empty() as usize;
            line.push(word);
        }

        if !line.is_empty() {
            render_line(size, line.join(" "), &mut res, (r, g, b), step_x);
        }
        (res, coords)
    }

    fn render_letter<F>(
        &self,
        pos: &Coords,
        size: &Coords,
        step_x: (i16, i16, i16),
        step_y: (i16, i16, i16),
        render_line: F,
        offset: usize,
    ) -> (String, Coords)
    where
        F: Fn(&Coords, String, &mut String, (u8, u8, u8), (i16, i16, i16)),
    {
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);
        for _ in 0..offset {
            (r, g, b) = self.add_step((r, g, b), step_x);
        }

        let mut coords = Coords::new(offset, pos.y);
        let mut res = Cursor::Pos(pos.x + offset, pos.y).to_string();

        let fits = self.text.len() <= size.x * size.y;
        for chunk in self.text.chars().collect::<Vec<char>>().chunks(size.x) {
            let mut chunk_str: String = chunk.iter().collect();
            coords.x = chunk_str.len();
            if !fits && coords.y + 1 == size.y + pos.y {
                let sum = coords.x + self.ellipsis.len();
                if sum >= size.x {
                    let end = size.x.saturating_sub(self.ellipsis.len());
                    chunk_str = chunk_str[..end].to_string();
                }

                chunk_str.push_str(&self.ellipsis);
                coords.x = chunk_str.len();
                render_line(size, chunk_str, &mut res, (r, g, b), step_x);
                return (res, coords);
            }

            render_line(size, chunk_str, &mut res, (r, g, b), step_x);
            (r, g, b) = self.add_step((r, g, b), step_y);
            coords.y += 1;
            res.push_str(&Cursor::Pos(pos.x, coords.y).to_string());
        }
        (res, Coords::new(coords.x, max(coords.y - 1, pos.y)))
    }

    fn render_line_ver(
        &self,
        _size: &Coords,
        line: String,
        res: &mut String,
        (r, g, b): (u8, u8, u8),
        _step: (i16, i16, i16),
    ) {
        // _ = self.set_alignment(size, line.len());
        res.push_str(&Fg::RGB(r, g, b).to_string());
        res.push_str(&line);
    }

    fn render_line_hor(
        &self,
        size: &Coords,
        line: String,
        res: &mut String,
        (r, g, b): (u8, u8, u8),
        step: (i16, i16, i16),
    ) {
        let offset = self.set_alignment(size, line.len());

        let (mut r, mut g, mut b) = (r, g, b);
        if self.text.len() > size.x {
            for _ in 0..offset {
                (r, g, b) = self.add_step((r, g, b), step);
            }
        };
        for c in line.chars() {
            res.push_str(&Fg::RGB(r, g, b).to_string());
            res.push(c);
            (r, g, b) = self.add_step((r, g, b), step);
        }
    }

    /// Sets text alignment and returns its offset
    fn set_alignment(&self, size: &Coords, len: usize) -> usize {
        match self.align {
            TextAlign::Left => 0,
            TextAlign::Center => {
                let offset = size.x.saturating_sub(len) >> 1;
                if offset > 0 {
                    print!("{}", Cursor::Right(offset))
                }
                offset
            }
            TextAlign::Right => {
                let offset = size.x.saturating_sub(len);
                if offset > 0 {
                    print!("{}", Cursor::Right(offset));
                }
                offset
            }
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

    /// Gets width of the [`Grad`] when using word wrap
    fn width_word_wrap(&self, size: &Coords) -> usize {
        let mut guess = Coords::new(self.size_letter_wrap(size.y), 0);

        while self.height_word_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets size of the [`Grad`] when using letter wrap
    fn size_letter_wrap(&self, size: usize) -> usize {
        (self.text.len() as f32 / size as f32).ceil() as usize
    }
}

// From implementations
impl From<Grad> for Box<dyn Widget> {
    fn from(value: Grad) -> Self {
        Box::new(value)
    }
}

impl From<Grad> for Box<dyn Text> {
    fn from(value: Grad) -> Self {
        Box::new(value)
    }
}
