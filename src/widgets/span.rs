use std::fmt;

use crate::{
    enums::{bg::Bg, cursor::Cursor, fg::Fg, modifier::Modifier, wrap::Wrap},
    geometry::coords::Coords,
};

use super::{text::Text, widget::Widget};

/// Widget for styling text
///
/// Available styles:
/// - foreground: can be set using [`Fg`]
/// - background: can be set using [`Bg`]
/// - modifications: can be set using [`Modifier`] (Bold, italic,...)
/// - wrap: how text should be wrapped, can be set using [`Wrap`]
/// - ellipsis: indication of overflown text, can be set to any string
///     (default: '...')
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     enums::{bg::Bg, fg::Fg, modifier::Modifier},
/// #     geometry::coords::Coords,
/// #     mods,
/// #     widgets::{
/// #         span::{Span, StrSpanExtension},
/// #         widget::Widget,
/// #     },
/// # };
///
/// // Creating span using new with red foreground:
/// let span = Span::new("Red text").fg(Fg::Red);
/// // Creating span using &str conversion with red text and white background
/// let span = "Red text on white".fg(Fg::Red).bg(Bg::White);
///
/// // Cyan bold and italic text on yellow background
/// // Using macro for getting modifiers
/// let span = "Cyan bold and italic on yellow"
///     .fg(Fg::Cyan)
///     .bg(Bg::Yellow)
///     .modifier(mods!(Bold, Italic));
///
/// // Span can be printed like this
/// println!("{span}");
///
/// // Or rendered on given coordinates and given size
/// // Text will be wrapping based on set value in wrap (Wrap::Word is default)
/// // Text will use ellipsis when can't fit ("..." is default)
/// span.render(&Coords::new(1, 1), &Coords::new(10, 3));
/// ```
#[derive(Debug)]
pub struct Span {
    text: String,
    fg: Fg,
    bg: Bg,
    modifier: Vec<Modifier>,
    wrap: Wrap,
    ellipsis: String,
}

impl Span {
    /// Creates new [`Span`] with given text
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            fg: Fg::Default,
            bg: Bg::Default,
            modifier: Vec::new(),
            wrap: Wrap::Word,
            ellipsis: "...".to_string(),
        }
    }

    /// Sets foreground of [`Span`] to given color
    pub fn fg(mut self, fg: Fg) -> Self {
        self.fg = fg;
        self
    }

    /// Sets background of [`Span`] to given color
    pub fn bg(mut self, bg: Bg) -> Self {
        self.bg = bg;
        self
    }

    /// Sets modifiers of [`Span`] to given modifiers
    pub fn modifier(mut self, mods: Vec<Modifier>) -> Self {
        self.modifier = mods;
        self
    }

    /// Sets [`Wrap`] of [`Span`] to given value
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets [`Span`] ellipsis to given string
    pub fn ellipsis(mut self, ellipsis: &str) -> Self {
        self.ellipsis = ellipsis.to_string();
        self
    }

    /// Gets [`Span`] length
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Checks if [`Span`] is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Widget for Span {
    fn render(&self, pos: &Coords, size: &Coords) {
        print!("{}", self.get_mods());

        match self.wrap {
            Wrap::Letter => _ = self.render_letter_wrap(pos, size, 0),
            Wrap::Word => _ = self.render_word_wrap(pos, size, 0),
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

impl Text for Span {
    fn render_offset(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
        wrap: Option<&Wrap>,
    ) -> Coords {
        let wrap = if let Some(wrap) = wrap {
            wrap
        } else {
            &self.wrap
        };

        match wrap {
            Wrap::Letter => self.render_letter_wrap(pos, size, offset),
            Wrap::Word => self.render_word_wrap(pos, size, offset),
        }
    }

    fn get(&self) -> String {
        format!("{}{}\x1b[0m", self.get_mods(), self.text)
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_mods(&self) -> String {
        let m = self
            .modifier
            .iter()
            .map(|m| m.to_ansi())
            .collect::<Vec<&str>>()
            .join("");
        format!("{}{}{}", self.fg, self.bg, m)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl Span {
    /// Renders [`Span`] with word wrapping with given offset
    /// Returns [`Coords`] where rendered text ends
    fn render_word_wrap(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
    ) -> Coords {
        let mut coords = Coords::new(offset, pos.y);
        print!("{}", Cursor::Pos(pos.x + offset, pos.y));

        let words: Vec<&str> = self.text.split_whitespace().collect();
        for word in words {
            let mut print_str = if coords.x == 0 {
                word.to_string()
            } else {
                format!(" {word}")
            };

            if coords.x + print_str.len() > size.x {
                coords.y += 1;
                if coords.y >= pos.y + size.y || word.len() > size.x {
                    self.render_ellipsis(&coords, size);
                    break;
                }

                coords.x = 0;
                print_str = word.to_string();
                print!("{}", Cursor::Pos(pos.x, coords.y));
            }

            print!("{print_str}");
            coords.x += print_str.len();
        }
        Coords::new(coords.x, coords.y)
    }

    /// Renders [`Span`] with letter wrapping with given offset
    /// Returns [`Coords`] where rendered text ended
    fn render_letter_wrap(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
    ) -> Coords {
        let mut coords = Coords::new(offset, pos.y);
        print!("{}", Cursor::Pos(pos.x + offset, pos.y));

        for (i, c) in self.text.chars().enumerate() {
            if coords.x >= size.x {
                coords.x = 0;
                coords.y += 1;
                print!("{}", Cursor::Pos(pos.x, coords.y));
            }

            // TODO think about this condition (too complicated)
            if coords.y + 1 == size.y + pos.y
                && coords.x + self.text.len() - i > size.x
                && coords.x + self.ellipsis.len() >= size.x
            {
                print!("{}", self.ellipsis);
                coords.x += self.ellipsis.len();
                break;
            }

            print!("{c}");
            coords.x += 1;
        }
        Coords::new(coords.x + 1, coords.y)
    }

    /// Renders ellipsis when rendering [`Span`] with word wrap
    fn render_ellipsis(&self, coords: &Coords, size: &Coords) {
        let sum = coords.x + self.ellipsis.len();
        if sum > size.x {
            if size.x < self.ellipsis.len() {
                return;
            }

            print!("{}", Cursor::Left(sum - size.x));
        }
        print!("{}", self.ellipsis);
    }

    /// Gets height of the [`Span`] when using word wrap
    fn height_word_wrap(&self, size: &Coords) -> usize {
        let mut coords = Coords::new(0, 0);

        let words: Vec<&str> = self.text.split_whitespace().collect();
        for word in words {
            let len = word.len();
            if (coords.x == 0 && coords.x + len > size.x)
                || (coords.x != 0 && coords.x + len + 1 > size.x)
            {
                coords.y += 1;
                coords.x = 0;
            }

            if coords.x != 0 {
                coords.x += 1;
            }
            coords.x += len;
        }
        coords.y + 1
    }

    /// Gets width of the [`Span`] when using word wrap
    fn width_word_wrap(&self, size: &Coords) -> usize {
        let mut guess = Coords::new(self.size_letter_wrap(size.y), 0);

        while self.height_word_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets size of the [`Span`] when using letter wrap
    fn size_letter_wrap(&self, size: usize) -> usize {
        (self.text.len() as f32 / size as f32).ceil() as usize
    }
}

/// Enables creating [`Span`] by calling one of the functions on &str
pub trait StrSpanExtension {
    /// Creates [`Span`] from &str and sets its fg to given color
    fn fg(self, fg: Fg) -> Span;

    /// Creates [`Span`] from &str and sets its bg to given color
    fn bg(self, bg: Bg) -> Span;

    /// Creates [`Span`] from &str and sets its modifier to given values
    fn modifier(self, mods: Vec<Modifier>) -> Span;

    /// Converts &str to [`Span`]
    fn to_span(self) -> Span;
}

impl StrSpanExtension for &str {
    fn fg(self, fg: Fg) -> Span {
        Span::new(self).fg(fg)
    }

    fn bg(self, bg: Bg) -> Span {
        Span::new(self).bg(bg)
    }

    fn modifier(self, mods: Vec<Modifier>) -> Span {
        Span::new(self).modifier(mods)
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

impl From<Span> for Box<dyn Text> {
    fn from(value: Span) -> Self {
        Box::new(value)
    }
}

impl From<&str> for Box<dyn Text> {
    fn from(value: &str) -> Self {
        Box::new(Span::new(value))
    }
}
