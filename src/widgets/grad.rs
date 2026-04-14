use core::fmt;
use std::{
    borrow::Cow,
    cmp::min,
    hash::{DefaultHasher, Hash, Hasher},
};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    enums::{Color, Modifier, RGB, Wrap},
    geometry::{Direction, Rect, TextAlign, Vec2},
    style::Style,
    text::{Line, StrStyle, Text, TextParser, text_render},
    widgets::layout::LayoutNode,
};

use super::{Element, widget::Widget};

/// A widget for rendering text with a gradient foreground color.
///
/// # Example
///
/// ```rust
/// use termint::{prelude::*, widgets::Grad};
///
/// // Text with blue-green foreground gradient
/// let grad = Grad::new("Hello Termint", (0, 0, 255), (0, 255, 0))
///     // Adds a white background
///     .bg(Color::White)
///     // Centers the text
///     .align(TextAlign::Center)
///     // Sets the wrapping to letter (new line after any character)
///     .wrap(Wrap::Letter)
///     // Adds `...` ellipsis (text shown when text overflows)
///     .ellipsis("...");
/// ```
///
/// [`Grad`] can also be used for printing the text directly to the terminal.
///
/// **Note**: text wrapping and ellipsis won't work in this mode, and the
/// gradient will be interpolated across the entire string length, rather than
/// per-line.
///
/// ```rust
/// use termint::widgets::Grad;
///
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
    /// Creates a new [`Grad`] with the given text and start/end colors.
    ///
    /// The `start` and `end` colors can be any type convertible into [`RGB`],
    /// such as `u32`, `(u8 ,u8, u8)`. You can read more in the [`RGB`]
    /// documentation.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::Grad, enums::RGB};
    ///
    /// // You can use RGB constructors for the colors.
    /// let grad = Grad::new("Hello, World!",
    ///     RGB::new(0, 220, 255),
    ///     RGB::from_hex(0xC83CFF)
    /// );
    /// // Or any type convertible into `RGB`, such as tuple and `u32` (hex).
    /// let grad = Grad::new("Hello, Termint!", (0, 220, 255), 0xC83CFF);
    /// ```
    #[must_use]
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

    /// Sets the direction of the color gradient.
    ///
    /// The default direction is [`Direction::Horizontal`].
    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the background color of the [`Grad`].
    ///
    /// The `bg` can be any type convertible into `Option<Color>`. You can
    /// supply `None` for transparent background.
    #[must_use]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.bg = bg.into();
        self
    }

    /// Replaces the current text modifiers with the given modifers.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::Grad, modifiers};
    ///
    /// // Italic and Bold modifiers using the bitwise or for chaining.
    /// let grad = Grad::new("modifier", (0, 220, 255), 0xC83CFF)
    ///     .modifier(Modifier::ITALIC | Modifier::BOLD);
    /// // Or shorther using `modifiers!` macro
    /// let grad = Grad::new("modifier", (0, 220, 255), 0xC83CFF)
    ///     .modifier(modifiers!(BOLD, ITALIC));
    /// ```
    #[must_use]
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = Modifier::empty();
        self.modifier.insert(modifier);
        self
    }

    /// Adds a modifier to the existing set of modifiers.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::Grad};
    ///
    /// let grad = Grad::new("add_modifier", (0, 220, 255), 0xC83CFF)
    ///     // Sets modifiers to bold.
    ///     .modifier(Modifier::BOLD)
    ///     // Adds italic to the modifiers, resulting in italic bold text.
    ///     .add_modifier(Modifier::ITALIC);
    /// ```
    #[must_use]
    pub fn add_modifier(mut self, flag: Modifier) -> Self {
        self.modifier.insert(flag);
        self
    }

    /// Removes a specific from the current set of modifiers.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::Grad};
    ///
    /// let grad = Grad::new("remove_modifier", (0, 220, 255), 0xC83CFF)
    ///     // Makes text italic and bold.
    ///     .modifier(Modifier::ITALIC | Modifier::BOLD)
    ///     // Removes the italic modifier, resulting in only bold text.
    ///     .remove_modifier(Modifier::ITALIC);
    /// ```
    #[must_use]
    pub fn remove_modifier(mut self, flag: Modifier) -> Self {
        self.modifier.remove(flag);
        self
    }

    /// Sets the text alignment of the [`Grad`].
    ///
    /// The default alignment is [`TextAlign::Left`].
    #[must_use]
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the wrapping strategy of the [`Grad`].
    ///
    /// The default wrapping is [`Wrap::Word`], which wraps text only after
    /// a word. You can also use [`Wrap::Letter`], which wraps after any
    /// character.
    #[must_use]
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets the ellipsis string to use when text overflows.
    ///
    /// The default value is `"..."`.
    #[must_use]
    pub fn ellipsis(mut self, ellipsis: &str) -> Self {
        self.ellipsis = ellipsis.to_string();
        self
    }
}

impl<M: Clone + 'static> Widget<M> for Grad {
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        text_render(self, buffer, layout.area, &self.ellipsis, self.align);
    }

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.text.hash(&mut hasher);
        self.wrap.hash(&mut hasher);

        hasher.finish()
    }

    fn height(&self, size: &Vec2) -> usize {
        self.inner_height(size)
    }

    fn width(&self, size: &Vec2) -> usize {
        self.inner_width(size)
    }
}

impl<'a> Text<'a> for Grad {
    fn render_offset(
        &self,
        buffer: &mut Buffer,
        rect: Rect,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2 {
        if rect.is_empty() {
            return Vec2::new(0, rect.y());
        }

        match self.direction {
            Direction::Vertical => {
                self.render_vertical(buffer, &rect, offset, wrap)
            }
            Direction::Horizontal => {
                self.render_horizontal(buffer, &rect, offset, wrap)
            }
        }
    }

    fn append_lines(
        &'a self,
        lines: &mut Vec<Line<'a>>,
        size: Vec2,
        wrap: Option<Wrap>,
    ) -> bool {
        let wrap = wrap.unwrap_or(self.wrap);
        let mut parser = TextParser::new(&self.text).wrap(wrap);
        let frags = self.get_frags(&mut parser, lines, size);
        if frags.is_empty() {
            return true;
        }

        let fit = parser.is_end();
        match self.direction {
            Direction::Vertical => {
                self.get_lines_vert(lines, frags, parser, size)
            }
            Direction::Horizontal => self.get_lines_hor(lines, frags, fit),
        }
        fit
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
    fn inner_height(&self, size: &Vec2) -> usize {
        match self.wrap {
            Wrap::Letter => self.height_letter_wrap(size),
            Wrap::Word => self.height_word_wrap(size),
        }
    }

    fn inner_width(&self, size: &Vec2) -> usize {
        match self.wrap {
            Wrap::Letter => self.width_letter_wrap(size),
            Wrap::Word => self.width_word_wrap(size),
        }
    }

    fn get_frags<'a>(
        &self,
        parser: &mut TextParser<'a>,
        lines: &mut Vec<Line<'a>>,
        size: Vec2,
    ) -> Vec<(&'a str, usize)> {
        if size.x == 0 || size.y == 0 || parser.is_end() {
            return vec![];
        }

        let mut frags = Vec::new();
        let last_width = lines.last().map(|l| l.width).unwrap_or_default();
        let mut fwidth = size.x.saturating_sub(last_width);

        for _ in 0..size.y {
            let Some(line) = parser.next_line(fwidth) else {
                break;
            };
            frags.push(line);
            fwidth = size.x;
        }
        frags
    }

    /// Assumes frags is not empty, otherwise it will not work.
    fn get_lines_vert<'a>(
        &self,
        lines: &mut Vec<Line<'a>>,
        frags: Vec<(&'a str, usize)>,
        mut parser: TextParser<'a>,
        size: Vec2,
    ) {
        let mut height = frags.len().saturating_sub(1);
        while let Some(_) = parser.next_line(size.x) {
            height += 1;
        }

        let step = self.get_step(height as i16);

        let (mut r, mut g, mut b) =
            (self.fg_start.r, self.fg_start.g, self.fg_start.b);
        let base_style = Style::new().bg(self.bg);

        let mut line = lines.pop().unwrap_or_else(Line::empty);
        for (text, len) in frags {
            let style = StrStyle::Static(base_style.fg(Color::Rgb(r, g, b)));
            line.push(text, len, style);
            lines.push(line);

            line = Line::empty();
            (r, g, b) = self.add_step((r, g, b), step);
        }
    }

    /// Assumes frags is not empty, otherwise it will not work.
    fn get_lines_hor<'a>(
        &self,
        lines: &mut Vec<Line<'a>>,
        frags: Vec<(&'a str, usize)>,
        fits: bool,
    ) {
        let style = if frags.len() <= 1 && fits {
            StrStyle::LocalGrad(self.fg_start, self.fg_end)
        } else {
            StrStyle::GlobalGrad(self.fg_start, self.fg_end)
        };

        let mut line = lines.pop().unwrap_or_else(Line::empty);
        for (text, len) in frags {
            line.push(text, len, style.clone());
            lines.push(line);
            line = Line::empty();
        }
    }

    fn render_vertical(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2 {
        let height = min(
            self.inner_height(rect.size()).saturating_sub(1),
            rect.height(),
        );
        let step = self.get_step(height as i16);
        self._render(
            buffer,
            rect,
            offset,
            wrap,
            (0, 0, 0),
            step,
            |b, a, t, l, p, r, s| self.render_ver_line(b, a, t, l, p, r, s),
        )
    }

    fn render_horizontal(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2 {
        let width = if self.inner_height(rect.size()) <= 1 {
            self.text.chars().count()
        } else {
            rect.width()
        };
        let step = self.get_step(width as i16);
        self._render(
            buffer,
            rect,
            offset,
            wrap,
            step,
            (0, 0, 0),
            |b, a, t, l, p, r, s| self.render_hor_line(b, a, t, l, p, r, s),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn _render<F>(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        offset: usize,
        wrap: Option<Wrap>,
        step_x: (i16, i16, i16),
        step_y: (i16, i16, i16),
        render_line: F,
    ) -> Vec2
    where
        F: Fn(
            &mut Buffer,
            &Rect,
            &str,
            usize,
            &Vec2,
            (u8, u8, u8),
            (i16, i16, i16),
        ),
    {
        let wrap = wrap.unwrap_or(self.wrap);
        let mut parser = TextParser::new(&self.text).wrap(wrap);

        let mut pos = Vec2::new(rect.x() + offset, rect.y());
        let mut fin_pos = pos;

        let mut rgb = (self.fg_start.r, self.fg_start.g, self.fg_start.b);
        if self.text.chars().count() + offset >= rect.width() {
            for _ in 0..offset {
                rgb = self.add_step(rgb, step_x);
            }
        }

        let right_end = rect.x() + rect.width();
        while pos.y <= rect.bottom() {
            let line_len = right_end.saturating_sub(pos.x);
            let Some((raw_text, mut len)) = parser.next_line(line_len) else {
                break;
            };

            let mut text = Cow::Borrowed(raw_text);
            if pos.y >= rect.bottom() && !parser.is_end() {
                let el_width = self.ellipsis.width();
                let target = line_len.saturating_sub(el_width);

                let mut width = len;
                let mut sid = raw_text.len();

                for (idx, grapheme) in raw_text.grapheme_indices(true).rev() {
                    if width <= target
                        && !grapheme.starts_with(char::is_whitespace)
                    {
                        break;
                    }
                    width -= grapheme.width();
                    sid = idx;
                }

                let trunc = &raw_text[..sid];
                text = Cow::Owned(format!("{}{}", trunc, self.ellipsis));
                len = width + el_width;
            }

            render_line(buffer, rect, &text, len, &pos, rgb, step_x);
            (fin_pos.x, fin_pos.y) =
                ((pos.x + len).saturating_sub(rect.x()), pos.y);
            (pos.x, pos.y) = (rect.x(), pos.y + 1);
            rgb = self.add_step(rgb, step_y);
        }
        fin_pos
    }

    /// Renders line with horizontal gradient
    #[allow(clippy::too_many_arguments)]
    fn render_hor_line(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        line: &str,
        len: usize,
        pos: &Vec2,
        (mut r, mut g, mut b): (u8, u8, u8),
        step: (i16, i16, i16),
    ) {
        let offset = self.get_align_offset(rect, len);
        for _ in 0..offset {
            (r, g, b) = self.add_step((r, g, b), step);
        }

        let mut style = Style::new()
            .fg(Color::Rgb(r, g, b))
            .bg(self.bg)
            .modifier(self.modifier);

        let mut coords = Vec2::new(pos.x + offset, pos.y);
        for c in line.chars() {
            buffer[coords].char(c).style(style);

            coords.x += 1;
            (r, g, b) = self.add_step((r, g, b), step);
            style = style.fg(Color::Rgb(r, g, b));
        }
    }

    /// Renders line with vertical gradient
    #[allow(clippy::too_many_arguments)]
    fn render_ver_line(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        line: &str,
        len: usize,
        pos: &Vec2,
        (r, g, b): (u8, u8, u8),
        _step: (i16, i16, i16),
    ) {
        let offset = self.get_align_offset(rect, len);
        let style = Style::new().fg(Color::Rgb(r, g, b)).bg(self.bg);
        buffer.set_str_styled(line, &Vec2::new(pos.x + offset, pos.y), style);
    }

    /// Gets text alignment offset
    fn get_align_offset(&self, rect: &Rect, len: usize) -> usize {
        match self.align {
            TextAlign::Left => 0,
            TextAlign::Center => rect.width().saturating_sub(len) >> 1,
            TextAlign::Right => rect.width().saturating_sub(len),
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
        let mut parser = TextParser::new(&self.text);

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
impl<M: Clone + 'static> From<Grad> for Box<dyn Widget<M>> {
    fn from(value: Grad) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Grad> for Element<M> {
    fn from(value: Grad) -> Self {
        Element::new(value)
    }
}

impl<'a> From<Grad> for Box<dyn Text<'a>> {
    fn from(value: Grad) -> Self {
        Box::new(value)
    }
}
