use std::fmt;

use crate::{
    buffer::Buffer,
    enums::{Color, Modifier, Wrap},
    geometry::{Rect, TextAlign, Vec2},
    style::Style,
    text::{Text, TextParser},
    widgets::cache::Cache,
};

use super::{widget::Widget, Element};

/// A widget for styling text where all characters share the same style.
///
/// # Supported styles
/// - `style`: style of text, set using [`Style`]
/// - `align`: text alignment, set using [`TextAlign`]
/// - `wrap`: text wrapping type, set using [`Wrap`]
/// - `ellipsis`: string shown when text overflows (default: '...')
///
/// # Examples
/// There are multiple ways to create a [`Span`].
/// ```rust
/// # use termint::{
/// #     enums::{Color, Modifier, Wrap},
/// #     geometry::{TextAlign},
/// #     modifiers,
/// #     widgets::{Span, ToSpan},
/// # };
/// // Using `new` with red foreground:
/// let span = Span::new("Red text").fg(Color::Red);
///
/// // Cyan bold and italic text on yellow background (using `modifiers` macro)
/// let span = "Cyan bold and italic on yellow"
///     .fg(Color::Cyan)
///     .bg(Color::Yellow)
///     .modifier(modifiers!(BOLD, ITALIC))
///     .align(TextAlign::Center)
///     .wrap(Wrap::Letter)
///     .ellipsis("...");
/// ```
///
/// Printing a [`Span`] applies styling but ignores wrapping and ellipsis:
/// ```rust
/// # use termint::{
/// #     widgets::{ToSpan},
/// # };
/// # let span = "test".to_span();
/// println!("{span}");
/// ```
///
/// To apply wrapping and ellipsis, render the span with a [`Term`] (or
/// manually with [`Buffer`]):
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     geometry::Rect,
/// #     widgets::{ToSpan, Widget},
/// #     term::Term,
/// # };
/// # fn example() -> Result<(), termint::Error> {
/// # let span = "test".to_span();
///
/// let mut term = Term::new();
/// term.render(span)?;
/// # Ok(())
/// # }
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
    /// Creates a new [`Span`] from any type convertible to string slice.
    ///
    /// # Example
    /// ```rust
    /// # use termint::widgets::Span;
    /// let span = Span::new("Hello, World!");
    /// let span = Span::new(String::from("Hello, Termint!"));
    /// let span = Span::new(&String::from("Hello, All!"));
    /// ```
    #[must_use]
    pub fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            text: text.as_ref().to_string(),
            ..Default::default()
        }
    }

    /// Sets the base style of the [`Span`].
    ///
    /// You can provide any type convertible to [`Style`].
    ///
    /// # Example
    /// ```rust
    /// # use termint::{widgets::Span, style::Style, enums::{Color, Modifier}};
    /// let span = Span::new("style").style(Style::new().bg(Color::Red));
    /// let span = Span::new("style").style(Color::Blue);
    /// ```
    #[must_use]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the foreground color of the [`Span`].
    ///
    /// You can provide any type convertible to [`Color`].
    #[must_use]
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.fg(fg);
        self
    }

    /// Sets the background color of the [`Span`].
    ///
    /// You can provide any type convertible to [`Color`].
    #[must_use]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.bg(bg);
        self
    }

    /// Sets the modifier flags of the [`Span`].
    ///
    /// # Example
    /// ```rust
    /// # use termint::{widgets::Span, enums::Modifier, modifiers};
    /// // Single modifier
    /// let span = Span::new("modifier").modifier(Modifier::ITALIC);
    /// // Multiple modifiers
    /// let span = Span::new("modifier")
    ///     .modifier(Modifier::ITALIC | Modifier::BOLD);
    /// let span = Span::new("modifier").modifier(modifiers!(BOLD, ITALIC));
    /// ```
    #[must_use]
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.style = self.style.modifier(modifier);
        self
    }

    /// Adds a modifier flag to the existing modifiers.
    ///
    /// # Example
    /// ```rust
    /// # use termint::{widgets::Span, enums::Modifier};
    /// let span = Span::new("add_modifier").add_modifier(Modifier::ITALIC);
    /// ```
    #[must_use]
    pub fn add_modifier(mut self, flag: Modifier) -> Self {
        self.style = self.style.add_modifier(flag);
        self
    }

    /// Removes a modifier flag from the existing modifiers.
    ///
    /// # Example
    /// ```rust
    /// # use termint::{widgets::Span, enums::Modifier};
    /// let span = Span::new("remove_modifier")
    ///     .remove_modifier(Modifier::ITALIC);
    /// ```
    #[must_use]
    pub fn remove_modifier(mut self, flag: Modifier) -> Self {
        self.style = self.style.remove_modifier(flag);
        self
    }

    /// Sets text alignment of the [`Span`].
    ///
    /// Default value is [`TextAlign::Left`].
    #[must_use]
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets text wrapping style of the [`Span`].
    ///
    /// Default value is [`Wrap::Word`].
    #[must_use]
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets the ellipsis string to use when text overflows.
    ///
    /// The default is `"..."``. Any custom string may be used.
    #[must_use]
    pub fn ellipsis<T>(mut self, ellipsis: T) -> Self
    where
        T: AsRef<str>,
    {
        self.ellipsis = ellipsis.as_ref().to_string();
        self
    }
}

impl Widget for Span {
    fn render(&self, buffer: &mut Buffer, rect: Rect, _cache: &mut Cache) {
        _ = self.render_offset(buffer, rect, 0, None);
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
        rect: Rect,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2 {
        if rect.is_empty() {
            return Vec2::new(0, rect.y());
        }

        let wrap = wrap.unwrap_or(self.wrap);
        let mut chars = self.text.chars();
        let mut parser = TextParser::new(&mut chars).wrap(wrap);

        let mut pos = Vec2::new(rect.x() + offset, rect.y());
        let mut fin_pos = pos;

        let right_end = rect.x() + rect.width();
        while pos.y <= rect.bottom() {
            let line_len = right_end.saturating_sub(pos.x);
            let Some((text, len)) = parser.next_line(line_len) else {
                break;
            };

            fin_pos.x =
                self.render_line(buffer, &rect, &parser, text, len, &pos);
            fin_pos.y = pos.y;
            pos.x = rect.x();
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
        rect: &Rect,
        parser: &TextParser,
        mut line: String,
        mut len: usize,
        pos: &Vec2,
    ) -> usize {
        if pos.y >= rect.bottom() && !parser.is_end() {
            len += self.ellipsis.len();
            if len > rect.width() {
                len = rect.width();
                let end = rect.width().saturating_sub(self.ellipsis.len());
                line = line[..end].to_string();
            }
            line.push_str(&self.ellipsis);
        }

        let x = match self.align {
            TextAlign::Left => 0,
            TextAlign::Center => rect.width().saturating_sub(len) >> 1,
            TextAlign::Right => rect.width().saturating_sub(len),
        };
        buffer.set_str_styled(line, &Vec2::new(pos.x + x, pos.y), self.style);
        pos.x + x + len - 1
    }

    /// Gets height of the [`Span`] when using word wrap
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

    /// Gets width of the [`Span`] when using word wrap
    fn width_word_wrap(&self, size: &Vec2) -> usize {
        let mut guess =
            Vec2::new(self.size_letter_wrap(size.y).saturating_sub(1), 0);

        while self.height_word_wrap(&guess) > size.y {
            let Some(val) = guess.x.checked_add(1) else {
                break;
            };
            guess.x = val;
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

/// Enables creating [`Span`] by calling one of the functions on type
/// implementing this trait.
///
/// It's recommended to use `std::fmt::Display` trait. Types implementing this
/// trait will contain `ToSpan` as well and can be converted to `Span`.
pub trait ToSpan {
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
    fn modifier(self, modifier: Modifier) -> Span;

    /// Creates [`Span`] from string and add given modifier to it
    fn add_modifier(self, flag: Modifier) -> Span;

    /// Creates [`Span`] from string and sets its alignment to given value
    fn align(self, align: TextAlign) -> Span;

    /// Creates [`Span`] from string and sets its wrapping to given value
    fn wrap(self, wrap: Wrap) -> Span;

    /// Creates [`Span`] from string and sets its ellipsis to given value
    fn ellipsis<T>(self, ellipsis: T) -> Span
    where
        T: AsRef<str>;

    /// Converts type to [`Span`]
    fn to_span(self) -> Span;
}

impl<T> ToSpan for &T
where
    T: std::fmt::Display,
{
    fn style<S>(self, style: S) -> Span
    where
        S: Into<Style>,
    {
        Span::new(self.to_string()).style(style)
    }

    fn fg<C>(self, fg: C) -> Span
    where
        C: Into<Option<Color>>,
    {
        Span::new(self.to_string()).fg(fg)
    }

    fn bg<C>(self, bg: C) -> Span
    where
        C: Into<Option<Color>>,
    {
        Span::new(self.to_string()).bg(bg)
    }

    fn modifier(self, modifier: Modifier) -> Span {
        Span::new(self.to_string()).modifier(modifier)
    }

    fn add_modifier(self, flag: Modifier) -> Span {
        Span::new(self.to_string()).add_modifier(flag)
    }

    fn align(self, align: TextAlign) -> Span {
        Span::new(self.to_string()).align(align)
    }

    fn wrap(self, wrap: Wrap) -> Span {
        Span::new(self.to_string()).wrap(wrap)
    }

    fn ellipsis<R>(self, ellipsis: R) -> Span
    where
        R: AsRef<str>,
    {
        Span::new(self.to_string()).ellipsis(ellipsis.as_ref())
    }

    fn to_span(self) -> Span {
        Span::new(self.to_string())
    }
}

// From implementations
impl<T> From<T> for Span
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Span::new(value)
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

impl<T> From<T> for Box<dyn Text>
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Box::new(Span::new(value))
    }
}

impl<T> From<T> for Element
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Element::new(Span::new(value))
    }
}

impl From<Span> for Box<dyn Widget> {
    fn from(value: Span) -> Self {
        Box::new(value)
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
