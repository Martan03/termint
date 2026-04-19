use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{
    buffer::Buffer,
    enums::{Modifier, Wrap},
    prelude::{TextAlign, Vec2},
    style::{Style, Styleable},
    text::{Line, Text, TextParser, text_render},
    widgets::{Element, LayoutNode, Widget},
};

mod to_span;
pub use to_span::ToSpan;

/// A widget for styling text where all characters share the same style.
///
/// # Supported styles
/// - `style`: style of text, set using [`Style`]
/// - `align`: text alignment, set using [`TextAlign`]
/// - `wrap`: text wrapping type, set using [`Wrap`]
/// - `ellipsis`: string shown when text overflows (default: '...')
///
/// # Examples
///
/// There are multiple ways to create a [`Span`].
/// ```rust
/// use termint::{prelude::*, modifiers};
///
/// // Using `new` constructor with red foreground and bold modifier:
/// let span = Span::new("Red text").fg(Color::Red).modifier(Modifier::BOLD);
///
/// // Using the `Stylize` trait to get colored span
/// let span = Span::new("Bold white text on black")
///     .white()
///     .on_black()
///     .bold();
///
/// // Cyan bold and italic text on yellow background
/// let span = "Cyan bold and italic on yellow"
///     .fg(Color::Cyan)
///     .bg(Color::Yellow)
///     // Using `modifiers!` macro to combine the modifiers easier
///     .modifier(modifiers!(BOLD, ITALIC))
///     // Center the text
///     .align(TextAlign::Center)
///     // Wrap to new line after any character
///     .wrap(Wrap::Letter)
///     // Display `...` if text doesn't fit
///     .ellipsis("...");
/// ```
///
/// Printing a [`Span`] applies styling but ignores wrapping and ellipsis:
/// ```rust
/// use termint::prelude::*;
///
/// let span = "Some text".fg(Color::Green);
/// println!("{span}");
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
    /// use termint::prelude::*;
    ///
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
    /// The `style` can be any type convertible to [`Style`].
    ///
    /// # Example
    /// ```rust
    /// use termint::prelude::*;
    ///
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

    /// Replaces the current text modifiers with the given modifers.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, modifiers};
    ///
    /// // Single modifier
    /// let span = Span::new("modifier").modifier(Modifier::ITALIC);
    /// // Multiple modifiers
    /// let span = Span::new("modifier")
    ///     .modifier(Modifier::ITALIC | Modifier::BOLD);
    /// // Or using modifiers macro
    /// let span = Span::new("modifier")
    ///     .modifier(modifiers!(BOLD, ITALIC));
    /// ```
    #[must_use]
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.style = self.style.modifier(modifier);
        self
    }

    /// Adds a modifier to the existing set of modifiers.
    ///
    /// # Example
    /// ```rust
    /// use termint::prelude::*;
    ///
    /// let span = Span::new("add_modifier")
    ///     // Sets modifiers to bold.
    ///     .modifier(Modifier::BOLD)
    ///     // Adds italic to the modifiers, resulting in italic bold text.
    ///     .add_modifier(Modifier::ITALIC);
    /// ```
    #[must_use]
    pub fn add_modifier(mut self, flag: Modifier) -> Self {
        self.style = self.style.add_modifier(flag);
        self
    }

    /// Removes a specific from the current set of modifiers.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::Grad};
    ///
    /// let span = Span::new("remove_modifier")
    ///     // Makes text italic and bold.
    ///     .modifier(Modifier::ITALIC | Modifier::BOLD)
    ///     // Removes the italic modifier, resulting in only bold text.
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

    /// Sets text wrapping strategy of the [`Span`].
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
    pub fn ellipsis<T>(mut self, ellipsis: T) -> Self
    where
        T: AsRef<str>,
    {
        self.ellipsis = ellipsis.as_ref().to_string();
        self
    }
}

impl<M: Clone + 'static> Widget<M> for Span {
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        text_render(self, buffer, layout.area, &self.ellipsis, self.align);
    }

    fn height(&self, size: &Vec2) -> usize {
        let mut parser = TextParser::new(&self.text).wrap(self.wrap);
        parser.height(size)
    }

    fn width(&self, size: &Vec2) -> usize {
        let mut parser = TextParser::new(&self.text).wrap(self.wrap);
        parser.width(size)
    }

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.text.hash(&mut hasher);
        self.wrap.hash(&mut hasher);

        hasher.finish()
    }
}

impl Text for Span {
    fn append_lines<'a>(
        &'a self,
        lines: &mut Vec<Line<'a>>,
        size: &Vec2,
        wrap: Option<Wrap>,
    ) -> bool {
        let wrap = wrap.unwrap_or(self.wrap);
        let mut parser = TextParser::new(&self.text).wrap(wrap);

        let height = lines.len().saturating_sub(1);
        let is_end = parser.is_end();
        if size.x == 0 || height >= size.y || is_end {
            return is_end;
        }

        let mut line = lines.pop().unwrap_or_else(Line::empty);
        for _ in height..size.y {
            let line_len = size.x.saturating_sub(line.width);
            let Some((text, len)) = parser.next_line(line_len) else {
                break;
            };

            line.push(text, len, self.style);
            lines.push(line);
            line = Line::empty();
        }
        parser.is_end()
    }

    fn get_align(&self) -> TextAlign {
        self.align
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

impl Styleable for Span {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
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

impl<M: Clone + 'static, T> From<T> for Box<dyn Widget<M>>
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

impl<M, T> From<T> for Element<M>
where
    M: Clone + 'static,
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Element::new(Span::new(value))
    }
}

impl<M: Clone + 'static> From<Span> for Box<dyn Widget<M>> {
    fn from(value: Span) -> Self {
        Box::new(value)
    }
}

impl From<Span> for Box<dyn Text> {
    fn from(value: Span) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Span> for Element<M> {
    fn from(value: Span) -> Self {
        Element::new(value)
    }
}
