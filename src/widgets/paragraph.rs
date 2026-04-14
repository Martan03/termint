use core::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{
    buffer::Buffer,
    enums::Wrap,
    prelude::{Rect, Vec2},
    text::Text,
    widgets::{Element, LayoutNode, Widget},
};

/// A widget that combines multiple text elements into a single flow.
///
/// A [`Paragraph`] displays a list of widgets implementing the [`Text`] trait
/// sequentially. Unlike the layout-based widgets, [`Paragraph`] places spans
/// directly next to each other (inline), wrapping them based on the
/// configuration.
///
/// It also supports a configurable separator which is inserted between
/// elements.
///
/// # Example
/// ```
/// use termint::{prelude::*, text::Text, paragraph};
///
/// // Using the constructor (requires homogeneous types)
/// let mut p = Paragraph::new(vec![
///     "This is a text in".fg(Color::Yellow),
///     "paragraph".modifier(Modifier::BOLD),
///     ". Cool, right?".to_span()
/// ])
/// .wrap(Wrap::Letter)
/// .separator("-");
///
/// // Or using the `paragraph!` macro for convenience
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
/// ```
#[derive(Debug)]
pub struct Paragraph<'a> {
    children: Vec<Box<dyn Text<'a>>>,
    separator: String,
    wrap: Wrap,
}

impl<'a> Paragraph<'a> {
    /// Creates a new [`Paragraph`] with the given child elements.
    #[must_use]
    pub fn new<I>(children: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Box<dyn Text<'a>>>,
    {
        Self {
            children: children.into_iter().map(|i| i.into()).collect(),
            ..Default::default()
        }
    }

    /// Creates an empty [`Paragraph`].
    #[must_use]
    pub fn empty() -> Self {
        Default::default()
    }

    /// Returns the raw text content as a [`String`].
    ///
    /// This joins all child elements using the configured separator, but
    /// ignores the text wrapping.
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

    /// Sets the separator string inserted child elements.
    ///
    /// ```rust
    /// use termint::prelude::*;
    ///
    /// // If the text fits on one line, it would be "Separator-showcase"
    /// let mut p = Paragraph::new(vec!["Separator", "showcase"])
    ///     .separator("-");
    /// ```
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

    /// Appends a child element to the end of the [`Paragraph`].
    pub fn push<T>(&mut self, child: T)
    where
        T: Into<Box<dyn Text<'a>>>,
    {
        self.children.push(child.into());
    }
}

impl<M: Clone + 'static> Widget<M> for Paragraph<'static> {
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        let rect = layout.area;
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

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.children
            .iter()
            .for_each(|c| c.get_text().hash(&mut hasher));
        self.separator.hash(&mut hasher);
        self.wrap.hash(&mut hasher);

        hasher.finish()
    }
}

impl<'a> fmt::Display for Paragraph<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl<'a> Default for Paragraph<'a> {
    /// Creates [`Paragraph`] filled with default values
    fn default() -> Self {
        Self {
            children: Vec::new(),
            separator: " ".to_string(),
            wrap: Wrap::Word,
        }
    }
}

impl<'a> Paragraph<'a> {
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
impl<M: Clone + 'static> From<Paragraph<'static>> for Box<dyn Widget<M>> {
    fn from(value: Paragraph<'static>) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Paragraph<'static>> for Element<M> {
    fn from(value: Paragraph<'static>) -> Self {
        Element::new(value)
    }
}
