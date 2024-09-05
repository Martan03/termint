use core::fmt;
use std::cmp::min;

use crate::{
    buffer::Buffer,
    enums::{Color, Modifier, Wrap, RGB},
    geometry::{Direction, TextAlign, Vec2},
    style::Style,
};

use super::{text::Text, widget::Widget};

/// Text with gradient foreground
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     geometry::Rect,
/// #     widgets::{Grad, Widget},
/// # };
/// // Creates text gradient widget
/// let grad = Grad::new(
///     "This text will have a gradient foreground and word wrap",
///     (0, 220, 255),
///     (200, 60, 255),
/// );
///
/// // Renders the text using buffer
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 10, 5));
/// grad.render(&mut buffer);
/// buffer.render();
/// ```
pub struct Grad {
    text: String,
    fg_start: RGB,
    fg_end: RGB,
    direction: Direction,
    bg: Option<Color>,
    modifier: Modifier,
    align: TextAlign,
    wrap: Wrap,
    ellipsis: String,
}

impl Grad {
    /// Creates new [`Grad`] with given text and given gradient
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
            modifier: Modifier::empty(),
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
        T: Into<Option<Color>>,
    {
        self.bg = bg.into();
        self
    }

    /// Sets [`Grad`] modifier to given modifiers
    pub fn modifier(mut self, modifier: u8) -> Self {
        self.modifier.clear();
        self.modifier.add(modifier);
        self
    }

    /// Adds given modifier to current [`Grad`] modifiers
    pub fn add_modifier(mut self, flag: u8) -> Self {
        self.modifier.add(flag);
        self
    }

    /// Removes given modifier from the current [`Grad`] modifiers
    pub fn remove_modifier(mut self, flag: u8) -> Self {
        self.modifier.sub(flag);
        self
    }

    /// Sets [`Grad`] text alignment
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets [`Wrap`] of [`Grad`] to given value
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
    fn render(&self, buffer: &mut Buffer) {
        if buffer.width() == 0 || buffer.height() == 0 {
            return;
        }

        match self.wrap {
            Wrap::Letter => self.render_letter(buffer, 0),
            Wrap::Word => self.render_word(buffer, 0),
        };
    }

    fn height(&self, size: &Vec2) -> usize {
        match self.wrap {
            Wrap::Letter => self.size_letter_wrap(size.x),
            Wrap::Word => self.height_word_wrap(size),
        }
    }

    fn width(&self, size: &Vec2) -> usize {
        match self.wrap {
            Wrap::Letter => self.size_letter_wrap(size.y),
            Wrap::Word => self.width_word_wrap(size),
        }
    }
}

impl Text for Grad {
    fn render_offset(
        &self,
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2 {
        if buffer.width() == 0 || buffer.height() == 0 {
            return Vec2::new(0, 0);
        }

        match wrap.unwrap_or(self.wrap) {
            Wrap::Letter => self.render_letter(buffer, offset),
            Wrap::Word => self.render_word(buffer, offset),
        }
    }

    fn get(&self) -> String {
        let step = self.get_step(self.text.len() as i16 - 1);
        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);

        let mut res = self.get_mods();
        for c in self.text.chars() {
            res += &format!("{}{c}", Color::Rgb(r, g, b).to_fg());
            (r, g, b) = self.add_step((r, g, b), step);
        }
        res += "\x1b[0m";

        res
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_mods(&self) -> String {
        format!(
            "{}{}",
            self.modifier,
            self.bg.map_or_else(|| "".to_string(), |bg| bg.to_bg()),
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
    /// Renders [`Grad`] widget with word wrap
    fn render_word(&self, buffer: &mut Buffer, offset: usize) -> Vec2 {
        match self.direction {
            Direction::Vertical => {
                let height = min(
                    self.height_word_wrap(buffer.size()) - 1,
                    buffer.height(),
                );
                let step = self.get_step(height as i16);
                self.render_words(
                    &self.text,
                    buffer,
                    offset,
                    (0, 0, 0),
                    step,
                    |t, b, p, r, s| self.render_ver_line(t, b, p, r, s),
                )
            }
            Direction::Horizontal => {
                let width = min(buffer.width(), self.text.len());
                let step = self.get_step(width as i16);
                self.render_words(
                    &self.text,
                    buffer,
                    offset,
                    step,
                    (0, 0, 0),
                    |t, b, p, r, s| self.render_hor_line(t, b, p, r, s),
                )
            }
        }
    }

    /// Renders [`Grad`] widget with letter wrap
    fn render_letter(&self, buffer: &mut Buffer, offset: usize) -> Vec2 {
        match self.direction {
            Direction::Vertical => {
                let height = min(
                    self.height_word_wrap(buffer.size()) - 1,
                    buffer.height(),
                );
                let step = self.get_step(height as i16);
                self.render_letters(
                    &self.text,
                    buffer,
                    offset,
                    (0, 0, 0),
                    step,
                    |t, b, p, r, s| self.render_ver_line(t, b, p, r, s),
                )
            }
            Direction::Horizontal => {
                let width = min(buffer.width(), self.text.len());
                let step = self.get_step(width as i16);
                self.render_letters(
                    &self.text,
                    buffer,
                    offset,
                    step,
                    (0, 0, 0),
                    |t, b, p, r, s| self.render_hor_line(t, b, p, r, s),
                )
            }
        }
    }

    /// Renders given text with word wrap
    fn render_words<F>(
        &self,
        text: &str,
        buffer: &mut Buffer,
        mut offset: usize,
        step_x: (i16, i16, i16),
        step_y: (i16, i16, i16),
        render_line: F,
    ) -> Vec2
    where
        F: Fn(String, &mut Buffer, &Vec2, (u8, u8, u8), (i16, i16, i16)),
    {
        let mut line = Vec::<&str>::new();
        let mut coords = Vec2::new(offset, buffer.y());

        let mut rgb = (self.fg_start.r, self.fg_start.g, self.fg_start.b);
        if self.text.len() + offset >= buffer.width() {
            for _ in 0..offset {
                rgb = self.add_step(rgb, step_x);
            }
        }

        for word in text.split_whitespace() {
            if coords.x + word.len() + !line.is_empty() as usize
                > buffer.width()
            {
                if coords.y + 1 >= buffer.y() + buffer.height()
                    || word.len() > buffer.width()
                {
                    let mut line_str = line.join(" ");
                    let sum = coords.x + self.ellipsis.len();
                    if sum + offset >= buffer.width() {
                        let end = buffer
                            .width()
                            .saturating_sub(self.ellipsis.len() + offset);
                        line_str = line_str[..end].to_string();
                    }

                    line_str.push_str(&self.ellipsis);
                    coords.x = line.len();
                    render_line(
                        line_str,
                        buffer,
                        &Vec2::new(buffer.x() + offset, coords.y),
                        rgb,
                        step_x,
                    );
                    return coords;
                }

                render_line(
                    line.join(" "),
                    buffer,
                    &Vec2::new(buffer.x() + offset, coords.y),
                    rgb,
                    step_x,
                );
                offset = 0;
                (coords.x, coords.y) = (0, coords.y + 1);
                rgb = self.add_step(rgb, step_y);
                line.clear();
            }
            coords.x += word.len() + !line.is_empty() as usize;
            line.push(word);
        }

        if !line.is_empty() {
            render_line(
                line.join(" "),
                buffer,
                &Vec2::new(buffer.x() + offset, coords.y),
                rgb,
                step_x,
            );
        }

        coords
    }

    /// Renders given text with letter wrap
    fn render_letters<F>(
        &self,
        text: &str,
        buffer: &mut Buffer,
        offset: usize,
        step_x: (i16, i16, i16),
        step_y: (i16, i16, i16),
        render_line: F,
    ) -> Vec2
    where
        F: Fn(String, &mut Buffer, &Vec2, (u8, u8, u8), (i16, i16, i16)),
    {
        let mut chars = text.chars().peekable();
        let mut coords = Vec2::new(offset, buffer.y());
        let mut rgb = (self.fg_start.r, self.fg_start.g, self.fg_start.b);
        for _ in 0..offset {
            rgb = self.add_step(rgb, step_x);
        }

        let mut line = String::new();
        for _ in 0..buffer.height() {
            if chars.peek().is_none() {
                coords.y -= 1;
                return coords;
            }

            line = chars.by_ref().take(buffer.width()).collect();
            coords.x = line.len();
            let pos = Vec2::new(buffer.x(), coords.y);
            render_line(line.clone(), buffer, &pos, rgb, step_x);

            coords.y += 1;
            rgb = self.add_step(rgb, step_y);
        }

        coords.y -= 1;
        if self.text.len() > buffer.area() {
            let end = buffer.width().saturating_sub(self.ellipsis.len());
            line = line[..end].to_string();
            line.push_str(&self.ellipsis);

            let pos = Vec2::new(buffer.x(), coords.y);
            render_line(line, buffer, &pos, rgb, step_x);
        }
        coords
    }

    /// Renders line with horizontal gradient
    fn render_hor_line(
        &self,
        line: String,
        buffer: &mut Buffer,
        pos: &Vec2,
        (mut r, mut g, mut b): (u8, u8, u8),
        step: (i16, i16, i16),
    ) {
        let offset = self.get_align_offset(buffer, line.len());
        for _ in 0..offset {
            (r, g, b) = self.add_step((r, g, b), step);
        }

        let mut style = Style::new()
            .fg(Color::Rgb(r, g, b))
            .bg(self.bg)
            .modifier(self.modifier.val());

        let mut coords = Vec2::new(pos.x + offset, pos.y);
        for c in line.chars() {
            buffer.set_val(c, &coords);
            buffer.set_style(style, &coords);

            coords.x += 1;
            (r, g, b) = self.add_step((r, g, b), step);
            style = style.fg(Color::Rgb(r, g, b));
        }
    }

    /// Renders line with vertical gradient
    fn render_ver_line(
        &self,
        line: String,
        buffer: &mut Buffer,
        pos: &Vec2,
        (r, g, b): (u8, u8, u8),
        _step: (i16, i16, i16),
    ) {
        let offset = self.get_align_offset(buffer, line.len());
        let style = Style::new().fg(Color::Rgb(r, g, b)).bg(self.bg);
        buffer.set_str_styled(line, &Vec2::new(pos.x + offset, pos.y), style);
    }

    /// Gets text alignment offset
    fn get_align_offset(&self, buffer: &Buffer, len: usize) -> usize {
        match self.align {
            TextAlign::Left => 0,
            TextAlign::Center => buffer.width().saturating_sub(len) >> 1,
            TextAlign::Right => buffer.width().saturating_sub(len),
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
    fn height_word_wrap(&self, size: &Vec2) -> usize {
        let mut coords = Vec2::new(0, 0);

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
    fn width_word_wrap(&self, size: &Vec2) -> usize {
        let mut guess = Vec2::new(self.size_letter_wrap(size.y), 0);

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
