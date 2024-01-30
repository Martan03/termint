use core::fmt;

use crate::{
    enums::{cursor::Cursor, wrap::Wrap},
    geometry::coords::Coords,
};

use super::{span::Span, widget::Widget};

/// [`Paragraph`] allow to use multiple [`Span`] in one Widget,
/// separating them with set separator. Spans are placed after each
/// other, which you can't really achieve with Layout
///
/// ## Example usage:
/// ```
/// # use termint::{
/// #     enums::{fg::Fg, modifier::Modifier},
/// #     geometry::coords::Coords,
/// #     widgets::{
/// #         paragraph::Paragraph, span::StrSpanExtension, widget::Widget,
/// #     },
/// # };
/// // Creates new Paragraph filled with spans
/// let mut p = Paragraph::new(vec![
///     "This is a text in".fg(Fg::Yellow),
///     "paragraph".modifier(vec![Modifier::Bold]).fg(Fg::Cyan),
///     "and it adds".to_span(),
///     "separator".modifier(vec![Modifier::Italic]),
/// ]);
/// // You can also add child later
/// p.add("between each span".to_span());
///
/// // Paragraph can be printed like this
/// println!("{p}");
///
/// // Or you can render it on given position and with given size
/// p.render(&Coords::new(1, 1), &Coords::new(20, 10));
/// ```
pub struct Paragraph {
    children: Vec<Span>,
    separator: String,
    wrap: Wrap,
}

impl Paragraph {
    /// Creates new [`Paragraph`]
    pub fn new(children: Vec<Span>) -> Self {
        Self {
            children: children,
            ..Default::default()
        }
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

    /// Adds [`Span`] to [`Paragraph`]
    pub fn add(&mut self, span: Span) {
        self.children.push(span);
    }

    /// Gets length of the [`Paragraph`]
    pub fn len(&self) -> usize {
        let mut len = 0;
        for child in self.children.iter() {
            len += child.width(&Coords::new(0, 1));
        }
        len + (self.children.len() - 1) * self.separator.len()
    }
}

impl Widget for Paragraph {
    fn render(&self, pos: &Coords, size: &Coords) {
        match self.wrap {
            Wrap::Letter => self.render_letter_wrap(pos, size),
            Wrap::Word => self.render_word_wrap(pos, size),
        }
        println!("\x1b[0m");
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
    /// Renders [`Paragraph`] with word wrap
    fn render_word_wrap(&self, pos: &Coords, size: &Coords) {
        let mut coords = Coords::new(0, pos.y);
        print!("{}", Cursor::Pos(pos.x, pos.y));

        for child in self.children.iter() {
            print!("{}", child.get_ansi());

            let words: Vec<&str> =
                child.get_text().split_whitespace().collect();
            for word in words {
                let mut print_str = if coords.x == 0 {
                    format!("{word}")
                } else {
                    format!(" {word}")
                };

                if coords.x + print_str.len() > size.x {
                    coords.y += 1;
                    if coords.y >= pos.y + size.y || word.len() > size.x {
                        break;
                    }

                    coords.x = 0;
                    print_str = word.to_string();
                    print!("{}", Cursor::Pos(pos.x, coords.y));
                }

                print!("{print_str}");
                coords.x += print_str.len();
            }
            print!("\x1b[0m");
        }
    }

    /// Renders [`Paragraph`] with letter wrap
    fn render_letter_wrap(&self, pos: &Coords, size: &Coords) {
        let chars = size.x * size.y;

        let mut x = 0;
        let mut y = 0;
        print!("{}", Cursor::Pos(pos.x, pos.y));
        for child in self.children.iter() {
            print!("{}", child.get_ansi());
            for c in child.get_text().chars() {
                if x + y * size.x >= chars {
                    break;
                }

                if x >= size.x {
                    y += 1;
                    x = 0;
                    print!("{}", Cursor::Pos(pos.x, pos.y + y));
                }

                print!("{c}");
                x += 1;
            }

            if x + self.separator.len() <= size.x {
                print!("{}", self.separator);
                x += self.separator.len();
            }
            print!("\x1b[0m");
        }
    }

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
