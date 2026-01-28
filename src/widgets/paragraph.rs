use core::fmt;

use crate::{
    buffer::Buffer,
    enums::Wrap,
    geometry::{Rect, Vec2},
    text::Text,
    widgets::cache::Cache,
};

use super::{widget::Widget, Element};

/// A [`Paragraph`] combines multiple widgets implementing the [`Text`] trait
/// into single widget, displaying them sequantially with configurable
/// separator.
///
/// Unlike layout-based widgets, [`Paragraph`] places spans directly next to
/// each other, allowing for more natural inline text composition.
///
/// # Example
/// ```
/// # use termint::{
/// #     term::Term,
/// #     paragraph,
/// #     text::Text,
/// #     enums::{Color, Modifier, Wrap},
/// #     widgets::{Paragraph, ToSpan, Widget},
/// # };
/// # fn example() -> Result<(), termint::Error> {
/// // Creates a Paragraph from a list of spans
/// let items: Vec<Box<dyn Text>> = vec![
///     Box::new("This is a text in".fg(Color::Yellow)),
///     Box::new("paragraph".modifier(Modifier::BOLD).fg(Color::Cyan)),
///     Box::new("and it adds".to_span()),
///     Box::new("separator".modifier(Modifier::ITALIC)),
/// ];
/// let mut p = Paragraph::new(items)
///     .wrap(Wrap::Letter)
///     .separator("-");
///
/// // Alternatively, use the `paragraph!` macro for convenience
/// let mut p = paragraph!(
///     "This is a text in".fg(Color::Yellow),
///     "paragraph".modifier(Modifier::BOLD).fg(Color::Cyan),
///     "and it adds".to_span(),
///     "separator".modifier(Modifier::ITALIC),
/// );
/// // Add more text later if needed
/// p.push("between each span");
///
/// // Print the Paragraph as a string
/// println!("{p}");
///
/// // Or you can render it using `Term` (or manually using `Buffer`)
/// let mut term = Term::new();
/// term.render(p)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Paragraph {
    children: Vec<Box<dyn Text>>,
    separator: String,
    wrap: Wrap,
}

impl Paragraph {
    /// Creates a new [`Paragraph`] with the given child elements.
    #[must_use]
    pub fn new(children: Vec<Box<dyn Text>>) -> Self {
        Self {
            children,
            ..Default::default()
        }
    }

    /// Creates an empty [`Paragraph`] with no children.
    #[must_use]
    pub fn empty() -> Self {
        Default::default()
    }

    /// Gets the rendered content of the [`Paragraph`] as a [`String`], joining
    /// all child elements with the configured separator.
    ///
    /// It ignores wrapping and any other position based settings.
    pub fn get(&self) -> String {
        let mut res = "".to_string();
        for child in self.children.iter() {
            if !res.is_empty() {
                res += &self.separator;
            }
            res += &child.get();
        }
        res
    }

    /// Sets the separator used between child elements when rendering the
    /// [`Paragraph`].
    #[must_use]
    pub fn separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }

    /// Sets text wrapping style of the [`Paragraph`].
    ///
    /// Default value is [`Wrap::Word`].
    #[must_use]
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Adds a child element to the [`Paragraph`]
    #[deprecated(
        since = "0.6.0",
        note = "Kept for compatibility purposes; use `push` function instead"
    )]
    pub fn add<T>(&mut self, child: T)
    where
        T: Into<Box<dyn Text>>,
    {
        self.children.push(child.into());
    }

    /// Adds a child element to the [`Paragraph`]
    pub fn push<T>(&mut self, child: T)
    where
        T: Into<Box<dyn Text>>,
    {
        self.children.push(child.into());
    }
}

impl Widget for Paragraph {
    fn render(&self, buffer: &mut Buffer, rect: Rect, _cache: &mut Cache) {
        let mut pos = Vec2::new(rect.x(), rect.y());
        let mut size = Vec2::new(rect.width(), rect.height());
        let mut offset = 0;

        for child in self.children.iter() {
            let crect = Rect::from_coords(pos, size);
            let end =
                child.render_offset(buffer, crect, offset, Some(self.wrap));

            size.y = size.y.saturating_sub(end.y - pos.y);
            pos.y = end.y;
            offset = end.x + self.separator.len();

            if end.y >= rect.y() + rect.height()
                && end.x >= rect.x() + rect.width()
            {
                break;
            }

            if offset + self.separator.len() <= rect.width() && offset != 0 {
                buffer.set_str(
                    &self.separator,
                    &Vec2::new(rect.x() + offset - 1, pos.y),
                );
            }
        }
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

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl Default for Paragraph {
    /// Creates [`Paragraph`] filled with default values
    fn default() -> Self {
        Self {
            children: Vec::new(),
            separator: " ".to_string(),
            wrap: Wrap::Word,
        }
    }
}

impl Paragraph {
    /// Gets [`Paragraph`] height when using word wrap
    fn height_word_wrap(&self, size: &Vec2) -> usize {
        let mut coords = Vec2::new(0, 0);

        for child in self.children.iter() {
            let words: Vec<&str> =
                child.get_text().split_whitespace().collect();
            for word in words {
                if (coords.x == 0 && coords.x + word.len() > size.x)
                    || (coords.x != 0 && coords.x + word.len() + 1 > size.x)
                {
                    coords.y += 1;
                    coords.x = 0;
                }

                if coords.x != 0 {
                    coords.x += 1;
                }
                coords.x += word.len();
            }
        }
        coords.y + 1
    }

    /// Gets width of [`Paragraph`] when using word wrap
    fn width_word_wrap(&self, size: &Vec2) -> usize {
        let mut guess = Vec2::new(self.size_letter_wrap(size.y), 0);

        while self.height_word_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets size of other side of [`Paragraph`] when using letter wrap
    /// When width given, gets height and other way around
    fn size_letter_wrap(&self, size: usize) -> usize {
        let mut len = 0;
        for child in self.children.iter() {
            len += child.get_text().len();
        }
        (len as f32 / size as f32).ceil() as usize
    }
}

// From implementations
impl From<Paragraph> for Box<dyn Widget> {
    fn from(value: Paragraph) -> Self {
        Box::new(value)
    }
}

impl From<Paragraph> for Element {
    fn from(value: Paragraph) -> Self {
        Element::new(value)
    }
}
