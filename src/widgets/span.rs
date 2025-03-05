use std::fmt;

use crate::{
    buffer::Buffer,
    enums::{Color, Wrap},
    geometry::{TextAlign, Vec2},
    style::Style,
    text::{Text, TextParser, TextToken},
};

use super::{widget::Widget, Element};

/// Widget for styling text
///
/// Available styles:
/// - foreground: can be set using [`Fg`]
/// - background: can be set using [`Bg`]
/// - modifications: can be set using [`Modifier`] (Bold, italic,...)
/// - align: can be set using [`TextAlign`]
/// - wrap: how text should be wrapped, can be set using [`Wrap`]
/// - ellipsis: indication of overflown text, can be set to any string
///     (default: '...')
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     enums::{Color, Modifier},
/// #     geometry::Rect,
/// #     modifiers,
/// #     widgets::{Span, StrSpanExtension, Widget},
/// # };
///
/// // Creating span using new with red foreground:
/// let span = Span::new("Red text").fg(Color::Red);
/// // Creating span using &str conversion with red text and white background
/// let span = "Red text on white".fg(Color::Red).bg(Color::White);
///
/// // Cyan bold and italic text on yellow background
/// // Using macro for getting modifiers
/// let span = "Cyan bold and italic on yellow"
///     .fg(Color::Cyan)
///     .bg(Color::Yellow)
///     .modifier(modifiers!(BOLD, ITALIC));
///
/// // Span can be printed like this
/// println!("{span}");
///
/// // Or rendered using the buffer
/// // Text will be wrapping based on set value in wrap (Wrap::Word is default)
/// // Text will use ellipsis when can't fit ("..." is default)
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 10, 3));
/// span.render(&mut buffer);
/// buffer.render();
/// ```
#[derive(Debug)]
pub struct Span {
    text: String,
    style: Style,
    align: TextAlign,
    wrap: Wrap,
    ellipsis: String,
}

impl Span {
    /// Creates new [`Span`] with given text
    pub fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            text: text.as_ref().to_string(),
            ..Default::default()
        }
    }

    /// Sets [`Span`] style to given style
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets foreground of [`Span`] to given color
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.fg(fg);
        self
    }

    /// Sets background of [`Span`] to given color
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.bg(bg);
        self
    }

    /// Sets [`Span`] modifier to given modifier
    pub fn modifier(mut self, modifier: u8) -> Self {
        self.style = self.style.modifier(modifier);
        self
    }

    /// Sets modifiers of [`Span`] to given modifiers
    pub fn add_modifier(mut self, flag: u8) -> Self {
        self.style = self.style.add_modifier(flag);
        self
    }

    /// Removes given modifier from [`Span`] modifiers
    pub fn remove_modifier(mut self, flag: u8) -> Self {
        self.style = self.style.remove_modifier(flag);
        self
    }

    /// Sets [`Span`] text alignment
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets [`Wrap`] of [`Span`] to given value
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets [`Span`] ellipsis to given string
    pub fn ellipsis<T: Into<String>>(mut self, ellipsis: T) -> Self {
        self.ellipsis = ellipsis.into();
        self
    }
}

impl Widget for Span {
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

impl Text for Span {
    fn render_offset(
        &self,
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2 {
        if buffer.area() == 0 {
            return *buffer.pos();
        }

        let wrap = wrap.unwrap_or(self.wrap);
        let mut chars = self.text.chars();
        let mut parser = TextParser::new(&mut chars).wrap(wrap);

        let mut pos = Vec2::new(buffer.x() + offset, buffer.y());
        let mut fin_pos = pos;
        let bottom = buffer.bottom();
        while pos.y <= bottom {
            match parser.next_line(buffer.right().saturating_sub(pos.x)) {
                TextToken::Text { mut text, mut len } => {
                    if pos.y + 1 >= buffer.y() + buffer.height()
                        && !parser.is_end()
                    {
                        len += self.ellipsis.len();
                        if len > buffer.width() {
                            len = buffer.width();
                            let end = buffer
                                .width()
                                .saturating_sub(self.ellipsis.len());
                            text = text[..end].to_string();
                        }
                        text.push_str(&self.ellipsis);
                    }
                    self.render_line(buffer, text, len, &pos);
                    fin_pos.x = len;
                }
                TextToken::Newline => {}
                TextToken::End => break,
            }
            fin_pos.y = pos.y;
            pos.x = buffer.x();
            pos.y += 1;
        }
        fin_pos
    }

    fn get(&self) -> String {
        format!("{}{}\x1b[0m", self.get_mods(), self.text)
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_mods(&self) -> String {
        self.style.to_string()
    }
}

impl Default for Span {
    fn default() -> Self {
        Self {
            text: Default::default(),
            style: Default::default(),
            align: Default::default(),
            wrap: Default::default(),
            ellipsis: "...".to_string(),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl Span {
    /// Renders one line of text and aligns it based on set alignment
    fn render_line(
        &self,
        buffer: &mut Buffer,
        line: String,
        len: usize,
        pos: &Vec2,
    ) {
        let x = match self.align {
            TextAlign::Left => 0,
            TextAlign::Center => buffer.width().saturating_sub(len) >> 1,
            TextAlign::Right => buffer.width().saturating_sub(len),
        };
        buffer.set_str_styled(line, &Vec2::new(pos.x + x, pos.y), self.style);
    }

    /// Gets height of the [`Span`] when using word wrap
    fn height_word_wrap(&self, size: &Vec2) -> usize {
        let mut chars = self.text.chars();
        let mut parser = TextParser::new(&mut chars);

        let mut pos = Vec2::new(0, 0);
        loop {
            match parser.next_line(size.x) {
                TextToken::Text { .. } => {}
                TextToken::Newline => continue,
                TextToken::End => break,
            }
            pos.y += 1;
        }
        pos.y
    }

    /// Gets width of the [`Span`] when using word wrap
    fn width_word_wrap(&self, size: &Vec2) -> usize {
        let mut guess =
            Vec2::new(self.size_letter_wrap(size.y).saturating_sub(1), 0);

        while self.height_word_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets height of the [`Span`] when using letter wrap
    fn height_letter_wrap(&self, size: &Vec2) -> usize {
        self.text
            .lines()
            .map(|l| {
                (l.chars().count() as f32 / size.x as f32).ceil() as usize
            })
            .sum()
    }

    /// Gets width of the [`Span`] when using letter wrap
    fn width_letter_wrap(&self, size: &Vec2) -> usize {
        let mut guess = Vec2::new(self.size_letter_wrap(size.y), 0);
        while self.height_letter_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets size of the [`Span`] when using letter wrap
    fn size_letter_wrap(&self, size: usize) -> usize {
        (self.text.chars().count() as f32 / size as f32).ceil() as usize
    }
}

/// Enables creating [`Span`] by calling one of the functions on string
pub trait StrSpanExtension {
    /// Creates [`Span`] from string and sets its style to given value
    fn style<T>(self, style: T) -> Span
    where
        T: Into<Style>;

    /// Creates [`Span`] from string and sets its fg to given color
    fn fg<T>(self, fg: T) -> Span
    where
        T: Into<Option<Color>>;

    /// Creates [`Span`] from string and sets its bg to given color
    fn bg<T>(self, bg: T) -> Span
    where
        T: Into<Option<Color>>;

    /// Creates [`Span`] from string and sets its modifier to given value
    fn modifier(self, modifier: u8) -> Span;

    /// Creates [`Span`] from string and add given modifier to it
    fn add_modifier(self, flag: u8) -> Span;

    /// Creates [`Span`] from string and sets its alignment to given value
    fn align(self, align: TextAlign) -> Span;

    /// Creates [`Span`] from string and sets its wrapping to given value
    fn wrap(self, wrap: Wrap) -> Span;

    /// Creates [`Span`] from string and sets its ellipsis to given value
    fn ellipsis<T>(self, ellipsis: T) -> Span
    where
        T: AsRef<str>;

    /// Converts &str to [`Span`]
    fn to_span(self) -> Span;
}

impl StrSpanExtension for &str {
    fn style<T>(self, style: T) -> Span
    where
        T: Into<Style>,
    {
        Span::new(self).style(style)
    }

    fn fg<T>(self, fg: T) -> Span
    where
        T: Into<Option<Color>>,
    {
        Span::new(self).fg(fg)
    }

    fn bg<T>(self, bg: T) -> Span
    where
        T: Into<Option<Color>>,
    {
        Span::new(self).bg(bg)
    }

    fn modifier(self, modifier: u8) -> Span {
        Span::new(self).modifier(modifier)
    }

    fn add_modifier(self, flag: u8) -> Span {
        Span::new(self).add_modifier(flag)
    }

    fn align(self, align: TextAlign) -> Span {
        Span::new(self).align(align)
    }

    fn wrap(self, wrap: Wrap) -> Span {
        Span::new(self).wrap(wrap)
    }

    fn ellipsis<R>(self, ellipsis: R) -> Span
    where
        R: AsRef<str>,
    {
        Span::new(self).ellipsis(ellipsis.as_ref())
    }

    fn to_span(self) -> Span {
        Span::new(self)
    }
}

// From implementations
impl From<Span> for Box<dyn Widget> {
    fn from(value: Span) -> Self {
        Box::new(value)
    }
}

impl<T> From<T> for Box<dyn Widget>
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Box::new(Span::new(value.as_ref()))
    }
}

impl From<Span> for Box<dyn Text> {
    fn from(value: Span) -> Self {
        Box::new(value)
    }
}

impl From<Span> for Element {
    fn from(value: Span) -> Self {
        Element::new(value)
    }
}

impl From<&str> for Box<dyn Text> {
    fn from(value: &str) -> Self {
        Box::new(Span::new(value))
    }
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        Element::new(Span::new(value))
    }
}
