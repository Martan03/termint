use std::fmt;

use crate::{
    enums::{bg::Bg, cursor::Cursor, fg::Fg, modifier::Modifier, wrap::Wrap},
    geometry::{coords::Coords, text_align::TextAlign},
};

use super::{text::Text, widget::Widget};

/// Widget for styling text
///
/// Available styles:
/// - foreground: can be set using [`Fg`]
/// - background: can be set using [`Bg`]
/// - modifications: can be set using [`Modifier`] (Bold, italic,...)
/// - align: can be set using [`TextAlign`]
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
    bg: Option<Bg>,
    modifier: Vec<Modifier>,
    align: TextAlign,
    wrap: Wrap,
    ellipsis: String,
}

impl Span {
    /// Creates new [`Span`] with given text
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Sets foreground of [`Span`] to given color
    pub fn fg(mut self, fg: Fg) -> Self {
        self.fg = fg;
        self
    }

    /// Sets background of [`Span`] to given color
    pub fn bg<T: Into<Option<Bg>>>(mut self, bg: T) -> Self {
        self.bg = bg.into();
        self
    }

    /// Sets modifiers of [`Span`] to given modifiers
    pub fn modifier(mut self, mods: Vec<Modifier>) -> Self {
        self.modifier = mods;
        self
    }

    /// Sets [`Span`] text alignment
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets [`Wrap`] of [`Span`] to given value
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets [`Span`] ellipsis to given string
    pub fn ellipsis<T: Into<String>>(mut self, ellipsis: T) -> Self {
        self.ellipsis = ellipsis.into();
        self
    }
}

impl Widget for Span {
    fn render(&self, pos: &Coords, size: &Coords) {
        print!("{}", self.get_mods());

        match self.wrap {
            Wrap::Letter => _ = self.render_letter(pos, size, 0),
            Wrap::Word => _ = self.render_word(pos, size, 0),
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
        let wrap = wrap.unwrap_or(&self.wrap);
        match wrap {
            Wrap::Letter => self.render_letter(pos, size, offset),
            Wrap::Word => self.render_word(pos, size, offset),
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
        format!(
            "{}{}{}",
            self.fg,
            self.bg.map_or_else(|| "".to_string(), |bg| bg.to_string()),
            m
        )
    }
}

impl Default for Span {
    fn default() -> Self {
        Self {
            text: Default::default(),
            fg: Default::default(),
            bg: Default::default(),
            modifier: Default::default(),
            align: Default::default(),
            wrap: Wrap::Word,
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
    /// Renders [`Span`] with word wrapping with given offset
    /// Returns [`Coords`] where rendered text ends
    fn render_word(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
    ) -> Coords {
        let mut res: Vec<&str> = vec![];
        let mut coords = Coords::new(offset, pos.y);

        print!("{}", Cursor::Pos(pos.x + offset, pos.y));
        for word in self.text.split_whitespace() {
            if coords.x + word.len() + !res.is_empty() as usize > size.x {
                if coords.y + 1 >= pos.y + size.y || word.len() > size.x {
                    self.render_line(size, res.join(" "));
                    self.render_ellipsis(&coords, size);
                    return coords;
                }

                (coords.x, coords.y) = (0, coords.y + 1);
                self.render_line(size, res.join(" "));
                print!("{}", Cursor::Pos(pos.x, coords.y));
                res = vec![];
            }
            coords.x += word.len() + !res.is_empty() as usize;
            res.push(word);
        }

        if !res.is_empty() {
            self.render_line(size, res.join(" "));
        }
        coords
    }

    /// Renders [`Span`] with letter wrapping with given offset
    /// Returns [`Coords`] where rendered text ended
    fn render_letter(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
    ) -> Coords {
        let mut coords = Coords::new(offset, pos.y);
        print!("{}", Cursor::Pos(pos.x + offset, pos.y));

        let fits = self.text.len() <= size.x * size.y;
        for chunk in self.text.chars().collect::<Vec<char>>().chunks(size.x) {
            let chunk_str: String = chunk.iter().collect();
            coords.x = chunk_str.len();
            print!("{chunk_str}");
            if !fits && coords.y + 1 == size.y + pos.y {
                self.render_ellipsis(&coords, size);
                return coords;
            }

            coords.y = coords.y + 1;
            print!("{}", Cursor::Pos(pos.x, coords.y));
        }
        coords
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

    /// Renders one line of text and aligns it based on set alignment
    fn render_line(&self, size: &Coords, line: String) {
        match self.align {
            TextAlign::Left => (),
            TextAlign::Center => {
                let offset = size.x.saturating_sub(line.len()) >> 1;
                if offset > 0 {
                    print!("{}", Cursor::Right(offset))
                }
            }
            TextAlign::Right => {
                let offset = size.x.saturating_sub(line.len());
                if offset > 0 {
                    print!("{}", Cursor::Right(offset));
                }
            }
        }
        print!("{}", line);
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

/// Enables creating [`Span`] by calling one of the functions on string
pub trait StrSpanExtension {
    /// Creates [`Span`] from string and sets its fg to given color
    fn fg(self, fg: Fg) -> Span;

    /// Creates [`Span`] from string and sets its bg to given color
    fn bg(self, bg: Bg) -> Span;

    /// Creates [`Span`] from string and sets its modifier to given values
    fn modifier(self, mods: Vec<Modifier>) -> Span;

    /// Creates [`Span`] from string and sets its alignment to given value
    fn align(self, align: TextAlign) -> Span;

    /// Creates [`Span`] from string and sets its wrapping to given value
    fn wrap(self, wrap: Wrap) -> Span;

    /// Creates [`Span`] from string and sets its ellipsis to given value
    fn ellipsis<T: AsRef<str>>(self, ellipsis: T) -> Span;

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

    fn align(self, align: TextAlign) -> Span {
        Span::new(self).align(align)
    }

    fn wrap(self, wrap: Wrap) -> Span {
        Span::new(self).wrap(wrap)
    }

    fn ellipsis<T: AsRef<str>>(self, ellipsis: T) -> Span {
        Span::new(self).ellipsis(ellipsis.as_ref())
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

impl<T> From<T> for Box<dyn Widget>
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Box::new(Span::new(value.as_ref().to_string()))
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
