use std::{
    cmp::max,
    fmt,
    io::{stdout, Write},
};

use crate::{
    buffer::buffer::Buffer,
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
    fn render(&self, buffer: &mut Buffer) {
        print!("{}", self.get_string(&buffer.pos(), &buffer.size()));
        _ = stdout().flush();
    }

    fn get_string(&self, pos: &Coords, size: &Coords) -> String {
        let (res, _) = self.get_offset(pos, size, 0, None);
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

impl Text for Span {
    fn render_offset(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
        wrap: Option<&Wrap>,
    ) -> Coords {
        let (res, coords) = self.get_offset(pos, size, offset, wrap);
        print!("{res}");
        coords
    }

    fn get_offset(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
        wrap: Option<&Wrap>,
    ) -> (String, Coords) {
        let wrap = wrap.unwrap_or(&self.wrap);
        let (res, coords) = match wrap {
            Wrap::Letter => {
                self.render_lines(pos, size, offset, |t, r, p, s, o| {
                    self.render_letter(t, r, p, s, o)
                })
            }
            Wrap::Word => {
                self.render_lines(pos, size, offset, |t, r, p, s, o| {
                    self.render_word(t, r, p, s, o)
                })
            }
        };
        (format!("{}{res}\x1b[0m", self.get_mods()), coords)
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
    /// Renders each line of the [`Span`]
    fn render_lines<F>(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
        text_render: F,
    ) -> (String, Coords)
    where
        F: Fn(&str, &mut String, &Coords, &Coords, usize) -> Coords,
    {
        let mut res = String::new();
        let mut fin_coords = Coords::new(0, pos.y);
        let mut coords = Coords::new(pos.x, pos.y);
        let mut lsize = *size;

        let mut offset = offset;
        for line in self.text.lines() {
            if lsize.y == 0 {
                break;
            }

            fin_coords = text_render(line, &mut res, &coords, &lsize, offset);
            (coords.x, coords.y) = (pos.x, fin_coords.y + 1);
            lsize.y = size.y.saturating_sub(coords.y - pos.y);
            offset = 0;
        }
        (res, fin_coords)
    }

    /// Renders [`Span`] with word wrapping with given offset
    /// Returns [`Coords`] where rendered text ends
    fn render_word(
        &self,
        text: &str,
        res: &mut String,
        pos: &Coords,
        size: &Coords,
        offset: usize,
    ) -> Coords {
        res.push_str(&Cursor::Pos(pos.x + offset, pos.y).to_string());
        let mut line: Vec<&str> = vec![];
        let mut coords = Coords::new(offset, pos.y);

        for word in text.split_whitespace() {
            if coords.x + word.len() + !line.is_empty() as usize > size.x {
                if coords.y + 1 >= pos.y + size.y || word.len() > size.x {
                    let mut line_str = line.join(" ");
                    let sum = coords.x + self.ellipsis.len();
                    if sum >= size.x {
                        let end = size.x.saturating_sub(self.ellipsis.len());
                        line_str = line_str[..end].to_string();
                    }

                    line_str.push_str(&self.ellipsis);
                    coords.x = line.len();
                    self.render_line(res, size, line_str);
                    return coords;
                }

                (coords.x, coords.y) = (0, coords.y + 1);
                self.render_line(res, size, line.join(" "));
                res.push_str(&Cursor::Pos(pos.x, coords.y).to_string());
                line = vec![];
            }
            coords.x += word.len() + !line.is_empty() as usize;
            line.push(word);
        }

        if !line.is_empty() {
            self.render_line(res, size, line.join(" "));
        }
        coords
    }

    /// Renders [`Span`] with letter wrapping with given offset
    /// Returns [`Coords`] where rendered text ended
    fn render_letter(
        &self,
        text: &str,
        res: &mut String,
        pos: &Coords,
        size: &Coords,
        offset: usize,
    ) -> Coords {
        let mut coords = Coords::new(offset, pos.y);
        res.push_str(&Cursor::Pos(pos.x + offset, pos.y).to_string());

        let fits = text.len() <= size.x * size.y;
        for chunk in text.chars().collect::<Vec<char>>().chunks(size.x) {
            let mut chunk_str: String = chunk.iter().collect();
            coords.x = chunk_str.len();
            if !fits && coords.y + 1 == size.y + pos.y {
                let sum = coords.x + self.ellipsis.len();
                if sum >= size.x {
                    let end = size.x.saturating_sub(self.ellipsis.len());
                    chunk_str = chunk_str[..end].to_string();
                }

                chunk_str.push_str(&self.ellipsis);
                coords.x = chunk_str.len();
                res.push_str(&chunk_str);
                return coords;
            }

            coords.y += 1;
            res.push_str(&chunk_str);
            res.push_str(&Cursor::Pos(pos.x, coords.y).to_string());
        }
        Coords::new(coords.x, max(coords.y - 1, pos.y))
    }

    /// Renders one line of text and aligns it based on set alignment
    fn render_line(&self, res: &mut String, size: &Coords, line: String) {
        match self.align {
            TextAlign::Left => (),
            TextAlign::Center => {
                let offset = size.x.saturating_sub(line.len()) >> 1;
                if offset > 0 {
                    res.push_str(&Cursor::Right(offset).to_string());
                }
            }
            TextAlign::Right => {
                let offset = size.x.saturating_sub(line.len());
                if offset > 0 {
                    res.push_str(&Cursor::Right(offset).to_string());
                }
            }
        }
        res.push_str(&line);
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
