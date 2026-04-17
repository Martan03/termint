use core::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};

use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    enums::Wrap,
    prelude::{TextAlign, Vec2},
    style::Style,
    text::{Line, Text, text_render},
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
pub struct Paragraph {
    children: Vec<Box<dyn Text>>,
    separator: String,
    wrap: Wrap,
    align: TextAlign,
    ellipsis: String,
}

impl Paragraph {
    /// Creates a new [`Paragraph`] with the given child elements.
    #[must_use]
    pub fn new<I>(children: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Box<dyn Text>>,
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

    /// Sets the [`Paragraph`] text alignment.
    ///
    /// Default value is [`TextAlign::Left`].
    #[must_use]
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
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

    /// Appends a child element to the end of the [`Paragraph`].
    pub fn push<T>(&mut self, child: T)
    where
        T: Into<Box<dyn Text>>,
    {
        self.children.push(child.into());
    }
}

impl<M: Clone + 'static> Widget<M> for Paragraph {
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        text_render(self, buffer, layout.area, &self.ellipsis, self.align);
    }

    fn height(&self, size: &Vec2) -> usize {
        if size.x == 0 || size.y == 0 {
            return 0;
        }

        let mut lines = vec![];
        self.append_lines(&mut lines, &Vec2::new(size.x, usize::MAX), None);
        lines.len()
    }

    fn width(&self, size: &Vec2) -> usize {
        if size.x == 0 || size.y == 0 {
            return 0;
        }

        let mut lines = vec![];
        let inf_size = Vec2::new(usize::MAX, usize::MAX);
        self.append_lines(&mut lines, &inf_size, None);
        let max_width = lines.iter().map(|l| l.width).max().unwrap_or(0);

        if size.y == 1 || max_width == 0 {
            return max_width;
        }

        let mut low = (max_width + size.y - 1) / size.y;
        let mut high = max_width;
        while low < high {
            let mid = low + (high - low) / 2;

            let mut lines = vec![];
            self.append_lines(&mut lines, &Vec2::new(mid, usize::MAX), None);

            if lines.len() <= size.y {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.children
            .iter()
            .for_each(|c| c.layout_hash().hash(&mut hasher));
        self.separator.hash(&mut hasher);
        self.wrap.hash(&mut hasher);

        hasher.finish()
    }
}

impl Text for Paragraph {
    fn append_lines<'a>(
        &'a self,
        lines: &mut Vec<Line<'a>>,
        size: &Vec2,
        wrap: Option<Wrap>,
    ) -> bool {
        let wrap = wrap.unwrap_or(self.wrap);

        let mut fit = true;
        let mut sep: Option<(usize, usize)> = None;
        for (i, child) in self.children.iter().enumerate() {
            fit = child.append_lines(lines, &size, Some(wrap));

            if let Some((Some(line), w)) =
                sep.take().map(|(i, w)| (lines.get_mut(i), w))
                && line.width == w
            {
                line.pop();
            }

            if !fit || lines.len() > size.y {
                break;
            }

            if i < self.children.len() - 1 && !self.separator.is_empty() {
                let last_id = lines.len().saturating_sub(1);
                if let Some(line) = lines.last_mut() {
                    let sep_w = self.separator.width();

                    if line.width + sep_w <= size.x {
                        // TODO: Add style?
                        line.push(&self.separator, sep_w, Style::default());
                        sep = Some((last_id, line.width));
                    } else {
                        lines.push(Line::empty());
                    }
                }
            }
        }
        fit
    }

    fn get(&self) -> String {
        let texts: Vec<String> =
            self.children.iter().map(|c| c.get()).collect();
        texts.join(&self.separator)
    }

    fn get_align(&self) -> TextAlign {
        self.align
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
            align: TextAlign::Left,
            ellipsis: "...".to_string(),
        }
    }
}

// From implementations
impl<M: Clone + 'static> From<Paragraph> for Box<dyn Widget<M>> {
    fn from(value: Paragraph) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Paragraph> for Element<M> {
    fn from(value: Paragraph) -> Self {
        Element::new(value)
    }
}
