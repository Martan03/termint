use core::fmt;
use std::io::{stdout, Write};

use crate::{
    enums::wrap::Wrap, geometry::coords::Coords, widgets::text::Text,
};

use super::widget::Widget;

/// [`Paragraph`] allow to use multiple widgets implementing [`Text`] trait
/// in one Widget, separating them with set separator. Spans are placed after
/// each other, which you can't really achieve with Layout
///
/// ## Example usage:
/// ```
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
    fn render(&self, pos: &Coords, size: &Coords) {
        let mut text_pos = Coords::new(pos.x, pos.y);
        let mut text_size = Coords::new(size.x, size.y);
        let mut offset = 0;

        for child in self.children.iter() {
            print!("{}", child.get_mods());

            let end = child.render_offset(
                &text_pos,
                &text_size,
                offset,
                Some(&self.wrap),
            );
            text_pos.y = end.y;
            offset = end.x + self.separator.len();

            if pos.y + size.y <= end.y {
                break;
            }
            text_size.y = pos.y + size.y - end.y;

            print!("\x1b[0m");
            if offset < size.x {
                print!("{}", self.separator);
            }
        }
        _ = stdout().flush();
    }

    fn get_string(&self, pos: &Coords, size: &Coords) -> String {
        let mut res = String::new();
        let mut text_pos = Coords::new(pos.x, pos.y);
        let mut text_size = Coords::new(size.x, size.y);
        let mut offset = 0;

        for child in self.children.iter() {
            res.push_str(&child.get_mods());

            let (child_res, end) = child.get_offset(
                &text_pos,
                &text_size,
                offset,
                Some(&self.wrap),
            );
            res.push_str(&child_res);
            text_pos.y = end.y;
            offset = end.x + self.separator.len();

            if pos.y + size.y <= end.y {
                break;
            }
            text_size.y = pos.y + size.y - end.y;

            res.push_str("\x1b[0m");
            if offset < size.x {
                res.push_str(&self.separator);
            }
        }
        res
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
            separator: " ".to_string(),
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
