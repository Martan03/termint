use core::fmt;

use crate::{
    buffer::Buffer,
    enums::Wrap,
    geometry::{Coords, Rect},
    widgets::text::Text,
};

use super::widget::Widget;

/// [`Paragraph`] allow to use multiple widgets implementing [`Text`] trait
/// in one Widget, separating them with set separator. Spans are placed after
/// each other, which you can't really achieve with Layout
///
/// ## Example usage:
/// ```ignore
/// # use termint::{
/// #     paragraph,
/// #     enums::{fg::Fg, modifier::Modifier},
/// #     geometry::coords::Coords,
/// #     widgets::{
/// #         paragraph::Paragraph, span::StrSpanExtension, widget::Widget,
/// #     },
/// # };
/// // Creates new Paragraph filled with spans
/// let mut p = Paragraph::new(vec![
///     Box::new("This is a text in".fg(Fg::Yellow)),
///     Box::new("paragraph".modifier(vec![Modifier::Bold]).fg(Fg::Cyan)),
///     Box::new("and it adds".to_span()),
///     Box::new("separator".modifier(vec![Modifier::Italic])),
/// ]);
///
/// // Creates new Paragraph filled with spans using macro
/// let mut p = paragraph!(
///     "This is a text in".fg(Fg::Yellow),
///     "paragraph".modifier(vec![Modifier::Bold]).fg(Fg::Cyan),
///     "and it adds".to_span(),
///     "separator".modifier(vec![Modifier::Italic]),
/// );
/// // You can also add child later
/// p.add("between each span".to_span());
///
/// // Paragraph can be printed like this
/// println!("{p}");
///
/// // Or you can render it on given position and with given size
/// p.render(&Coords::new(1, 1), &Coords::new(20, 10));
/// ```
#[derive(Debug)]
pub struct Paragraph {
    children: Vec<Box<dyn Text>>,
    separator: String,
    wrap: Wrap,
}

impl Paragraph {
    /// Creates new [`Paragraph`]
    pub fn new(children: Vec<Box<dyn Text>>) -> Self {
        Self {
            children,
            ..Default::default()
        }
    }

    /// Creates empty [`Paragraph`]
    pub fn empty() -> Self {
        Default::default()
    }

    /// Gets [`Paragraph`] as string
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

    /// Sets [`Paragraph`] separator to given string
    pub fn separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }

    /// Sets [`Paragraph`] wrapping to given option
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Adds child to [`Paragraph`]
    pub fn add<T>(&mut self, child: T)
    where
        T: Into<Box<dyn Text>>,
    {
        self.children.push(child.into());
    }
}

impl Widget for Paragraph {
    fn render(&self, buffer: &mut Buffer) {
        let mut pos = Coords::new(buffer.x(), buffer.y());
        let mut size = Coords::new(buffer.width(), buffer.height());
        let mut offset = 0;

        for child in self.children.iter() {
            let mut cbuffer = buffer.get_subset(Rect::from_coords(pos, size));
            let end =
                child.render_offset(&mut cbuffer, offset, Some(self.wrap));
            buffer.union(cbuffer);

            size.y = size.y.saturating_sub(end.y - pos.y);
            pos.y = end.y;
            offset = end.x + self.separator.len();

            if end.y >= buffer.y() + buffer.height()
                && end.x >= buffer.x() + buffer.width()
            {
                break;
            }

            if offset + self.separator.len() <= buffer.width() && offset != 0 {
                buffer.set_str(
                    &self.separator,
                    &Coords::new(buffer.x() + offset - 1, pos.y),
                );
            }
        }
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
            separator: "-".to_string(),
            wrap: Wrap::Word,
        }
    }
}

impl Paragraph {
    /// Gets [`Paragraph`] height when using word wrap
    fn height_word_wrap(&self, size: &Coords) -> usize {
        let mut coords = Coords::new(0, 0);

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
    fn width_word_wrap(&self, size: &Coords) -> usize {
        let mut guess = Coords::new(self.size_letter_wrap(size.y), 0);

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
