use std::fmt;

use crate::{
    enums::{bg::Bg, cursor::Cursor, fg::Fg, modifier::Modifier, wrap::Wrap},
    geometry::coords::Coords,
};

use super::widget::Widget;

/// [`Span`] makes easier text modifications such as foreground, background,...
///
/// ## Example usage:
/// ```rust
/// use termint::{
///     enums::{bg::Bg, fg::Fg, modifier::Modifier},
///     geometry::coords::Coords,
///     modifiers,
///     widgets::{
///         span::{Span, StrSpanExtension},
///         widget::Widget,
///     },
/// };
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
///     .modifier(modifiers!(Bold, Italic));
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

    /// Gets [`Span`] as string
    pub fn get(&self) -> String {
        format!("{}{}\x1b[0m", self.get_ansi(), self.text)
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

    /// Gets ANSI codes to set fg, bg and other [`Span`] properties
    fn get_ansi(&self) -> String {
        let m = self
            .modifier
            .iter()
            .map(|m| m.to_ansi())
            .collect::<Vec<&str>>()
            .join("");
        format!("{}{}{}", self.fg, self.bg, m)
    }

    /// Renders [`Span`] with word wrapping
    fn render_word_wrap(&self, pos: &Coords, size: &Coords) {
        let mut coords = Coords::new(0, pos.y);
        print!("{}", Cursor::Pos(pos.x, pos.y));

        let words: Vec<&str> = self.text.split_whitespace().collect();
        for word in words {
            let len = word.len();
            if coords.x + len + 1 > size.x {
                coords.y += 1;

                if coords.y >= pos.y + size.y || len > size.x {
                    if coords.x + self.ellipsis.len() >= size.x {
                        let sum = coords.x + self.ellipsis.len();
                        print!("{}", Cursor::Left(sum - size.x));
                    }
                    print!("{}", self.ellipsis);
                    break;
                }

                coords.x = 0;
                print!("{}", Cursor::Pos(pos.x, coords.y));
            }

            if coords.x == 0 {
                print!("{word}");
                coords.x += len;
            } else {
                print!(" {word}");
                coords.x += len + 1;
            }
        }
    }

    /// Renders [`Span`] with letter wrapping
    fn render_letter_wrap(&self, pos: &Coords, size: &Coords) {
        let chars = size.x * size.y;
        let ellipsis_len = self.ellipsis.len();

        for (i, c) in self.text.chars().enumerate() {
            if i + ellipsis_len >= chars {
                print!("{}", self.ellipsis);
                break;
            }

            if i % size.x == 0 {
                print!("{}", Cursor::Pos(pos.x, pos.y + i / size.x));
            }

            print!("{c}");
        }
    }
}

impl Widget for Span {
    fn render(&self, pos: &Coords, size: &Coords) {
        print!("{}", self.get_ansi());

        match self.wrap {
            Wrap::Letter => self.render_letter_wrap(pos, size),
            Wrap::Word => self.render_word_wrap(pos, size),
        }

        println!("\x1b[0m");
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
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
