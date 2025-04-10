use core::fmt;
use std::cmp::min;

use crate::{
    buffer::Buffer,
    enums::{Color, Modifier, Wrap, RGB},
    geometry::{Direction, TextAlign, Vec2},
    style::Style,
    text::{Text, TextParser},
};

use super::{widget::Widget, Element};

/// Widget for styling text with gradient foreground.
///
/// # Examples:
/// You can render text using the Term like this:
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     widgets::{Grad, Widget},
/// #     term::Term,
/// # };
/// let grad = Grad::new(
///     "This text will have a gradient foreground and word wrap",
///     (0, 220, 255),
///     (200, 60, 255),
/// );
///
/// // Renders the text using Term
/// let mut term = Term::new();
/// term.render(grad);
/// ```
///
/// You can also print the text directly to the terminal, but text wrapping
/// and ellipsis won't work and the gradient will be generated for the whole
/// text, not per line:
///
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     widgets::{Grad, Widget},
/// #     term::Term,
/// # };
/// let grad = Grad::new(
///     "Printing gradient also works",
///     (0, 220, 255),
///     (200, 60, 255),
/// );
///
/// println!("{grad}");
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
    /// Creates new [`Grad`] with given text and given gradient.
    ///
    /// - `start` is any type convertible to [`RGB`] and represents the start
    /// color of the gradient.
    /// - `end` is any type convertible to [`RGB`] and represents the end color
    /// of the gradient.
    ///
    /// ### Examples
    /// ```rust
    /// use termint::{enums::RGB, widgets::Grad};
    ///
    /// let grad = Grad::new("Hello, World!",
    ///     RGB::new(0, 220, 255),
    ///     RGB::from_hex(0xC83CFF)
    /// );
    /// let grad = Grad::new("Hello, Termint!", (0, 220, 255), 0xC83CFF);
    /// ```
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
            wrap: Default::default(),
            ellipsis: "...".to_string(),
        }
    }

    /// Sets text gradient direction.
    ///
    /// ### Examples
    /// ```rust
    /// use termint::{geometry::Direction, widgets::Grad};
    ///
    /// let grad = Grad::new("direction", (0, 220, 255), 0xC83CFF)
    ///     .direction(Direction::Vertical);
    /// ```
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets background of the [`Grad`] to given color.
    ///
    /// - `bg` can be any type convertible to [`Color`].
    ///
    /// ### Examples
    /// ```rust
    /// use termint::{widgets::Grad, enums::Color};
    ///
    /// let grad = Grad::new("bg", (0, 220, 255), 0xC83CFF).bg(Color::White);
    /// ```
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.bg = bg.into();
        self
    }

    /// Sets [`Grad`] modifier to given modifiers.
    ///
    /// ### Examples
    /// ```rust
    /// use termint::{widgets::Grad, enums::Modifier, modifiers};
    ///
    /// let grad = Grad::new("modifier", (0, 220, 255), 0xC83CFF)
    ///     .modifier(Modifier::ITALIC | Modifier::BOLD);
    /// let grad = Grad::new("modifier", (0, 220, 255), 0xC83CFF)
    ///     .modifier(modifiers!(BOLD, ITALIC));
    /// ```
    pub fn modifier(mut self, modifier: u8) -> Self {
        self.modifier.clear();
        self.modifier.add(modifier);
        self
    }

    /// Adds given modifier to current [`Grad`] modifiers.
    ///
    /// ### Examples
    /// ```rust
    /// use termint::{widgets::Grad, enums::Modifier};
    ///
    /// let grad = Grad::new("add_modifier", (0, 220, 255), 0xC83CFF)
    ///     .add_modifier(Modifier::ITALIC);
    /// ```
    pub fn add_modifier(mut self, flag: u8) -> Self {
        self.modifier.add(flag);
        self
    }

    /// Removes given modifier from the current [`Grad`] modifiers.
    ///
    /// ### Examples
    /// ```rust
    /// use termint::{widgets::Grad, enums::Modifier};
    ///
    /// let grad = Grad::new("remove_modifier", (0, 220, 255), 0xC83CFF)
    ///     .remove_modifier(Modifier::ITALIC);
    /// ```
    pub fn remove_modifier(mut self, flag: u8) -> Self {
        self.modifier.sub(flag);
        self
    }

    /// Sets text alignment of the [`Grad`].
    ///
    /// Default value is [`TextAlign::Left`].
    ///
    /// ### Examples
    /// ```rust
    /// use termint::{widgets::Grad, geometry::TextAlign};
    ///
    /// let grad = Grad::new("align", (0, 220, 255), 0xC83CFF)
    ///     .align(TextAlign::Center);
    /// ```
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets text wrapping style of the [`Grad`].
    ///
    /// Default value is [`Wrap::Word`].
    ///
    /// ### Examples
    /// ```rust
    /// use termint::{widgets::Grad, enums::Wrap};
    ///
    /// let grad = Grad::new("wrap", (0, 220, 255), 0xC83CFF)
    ///     .wrap(Wrap::Letter);
    /// ```
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets ellipsis string of the [`Grad`] to use when text can't fit. It is
    /// used to signal that text is overflown.
    ///
    /// Default value is "...". It can be any string.
    ///
    /// ### Examples
    /// ```rust
    /// use termint::widgets::Grad;
    ///
    /// // Overflown text will end with "~.~" sequence to signal overflow
    /// let grad = Grad::new("ellipsis", (0, 220, 255), 0xC83CFF)
    ///     .ellipsis("~.~");
    /// ```
    pub fn ellipsis(mut self, ellipsis: &str) -> Self {
        self.ellipsis = ellipsis.to_string();
        self
    }
}

impl Widget for Grad {
    fn render(&self, buffer: &mut Buffer) {
        _ = self.render_offset(buffer, 0, None);
    }

    fn height(&self, size: &Vec2) -> usize {
        match self.wrap {
            Wrap::Letter => self.height_letter_wrap(size),
            Wrap::Word => self.height_word_wrap(size),
        }
    }

    fn width(&self, size: &Vec2) -> usize {
        match self.wrap {
            Wrap::Letter => self.width_letter_wrap(size),
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
        if buffer.area() == 0 {
            return Vec2::new(0, buffer.y());
        }

        match self.direction {
            Direction::Vertical => self.render_vertical(buffer, offset, wrap),
            Direction::Horizontal => {
                self.render_horizontal(buffer, offset, wrap)
            }
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
    fn render_vertical(
        &self,
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2 {
        let height = min(
            self.height(buffer.size()).saturating_sub(1),
            buffer.height(),
        );
        let step = self.get_step(height as i16);
        self._render(
            buffer,
            offset,
            wrap,
            (0, 0, 0),
            step,
            |b, t, l, p, r, s| self.render_ver_line(b, t, l, p, r, s),
        )
    }

    fn render_horizontal(
        &self,
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2 {
        let width = if self.height(buffer.size()) <= 1 {
            self.text.chars().count()
        } else {
            buffer.width()
        };
        let step = self.get_step(width as i16);
        self._render(
            buffer,
            offset,
            wrap,
            step,
            (0, 0, 0),
            |b, t, l, p, r, s| self.render_hor_line(b, t, l, p, r, s),
        )
    }

    fn _render<F>(
        &self,
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<Wrap>,
        step_x: (i16, i16, i16),
        step_y: (i16, i16, i16),
        render_line: F,
    ) -> Vec2
    where
        F: Fn(
            &mut Buffer,
            String,
            usize,
            &Vec2,
            (u8, u8, u8),
            (i16, i16, i16),
        ),
    {
        let wrap = wrap.unwrap_or(self.wrap);
        let mut chars = self.text.chars();
        let mut parser = TextParser::new(&mut chars).wrap(wrap);

        let mut pos = Vec2::new(buffer.x() + offset, buffer.y());
        let mut fin_pos = pos;

        let mut rgb = (self.fg_start.r, self.fg_start.g, self.fg_start.b);
        if self.text.chars().count() + offset >= buffer.width() {
            for _ in 0..offset {
                rgb = self.add_step(rgb, step_x);
            }
        }

        let right_end = buffer.x() + buffer.width();
        while pos.y <= buffer.bottom() {
            let line_len = right_end.saturating_sub(pos.x);
            let Some((mut text, mut len)) = parser.next_line(line_len) else {
                break;
            };

            if pos.y >= buffer.bottom() && !parser.is_end() {
                len += self.ellipsis.len();
                if len > buffer.width() {
                    len = buffer.width();
                    let end =
                        buffer.width().saturating_sub(self.ellipsis.len());
                    text = text[..end].to_string();
                }
                text.push_str(&self.ellipsis);
            }

            render_line(buffer, text, len, &pos, rgb, step_x);
            (fin_pos.x, fin_pos.y) =
                ((pos.x + len).saturating_sub(buffer.x()), pos.y);
            (pos.x, pos.y) = (buffer.x(), pos.y + 1);
            rgb = self.add_step(rgb, step_y);
        }
        fin_pos
    }

    /// Renders line with horizontal gradient
    fn render_hor_line(
        &self,
        buffer: &mut Buffer,
        line: String,
        len: usize,
        pos: &Vec2,
        (mut r, mut g, mut b): (u8, u8, u8),
        step: (i16, i16, i16),
    ) {
        let offset = self.get_align_offset(buffer, len);
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
        buffer: &mut Buffer,
        line: String,
        len: usize,
        pos: &Vec2,
        (r, g, b): (u8, u8, u8),
        _step: (i16, i16, i16),
    ) {
        let offset = self.get_align_offset(buffer, len);
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
        let mut chars = self.text.chars();
        let mut parser = TextParser::new(&mut chars);

        let mut pos = Vec2::new(0, 0);
        loop {
            if parser.next_line(size.x).is_none() {
                break;
            }
            pos.y += 1;
        }
        pos.y
    }

    /// Gets width of the [`Grad`] when using word wrap
    fn width_word_wrap(&self, size: &Vec2) -> usize {
        let mut guess =
            Vec2::new(self.size_letter_wrap(size.y).saturating_sub(1), 0);

        while self.height_word_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets height of the [`Grad`] when using letter wrap
    fn height_letter_wrap(&self, size: &Vec2) -> usize {
        self.text
            .lines()
            .map(|l| {
                (l.chars().count() as f32 / size.x as f32).ceil() as usize
            })
            .sum()
    }

    /// Gets width of the [`Grad`] when using letter wrap
    fn width_letter_wrap(&self, size: &Vec2) -> usize {
        let mut guess = Vec2::new(self.size_letter_wrap(size.y), 0);
        while self.height_letter_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets size of the [`Grad`] when using letter wrap
    fn size_letter_wrap(&self, size: usize) -> usize {
        (self.text.chars().count() as f32 / size as f32).ceil() as usize
    }
}

// From implementations
impl From<Grad> for Box<dyn Widget> {
    fn from(value: Grad) -> Self {
        Box::new(value)
    }
}

impl From<Grad> for Element {
    fn from(value: Grad) -> Self {
        Element::new(value)
    }
}

impl From<Grad> for Box<dyn Text> {
    fn from(value: Grad) -> Self {
        Box::new(value)
    }
}
