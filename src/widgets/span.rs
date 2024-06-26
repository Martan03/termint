use std::fmt;

use crate::{
    buffer::buffer::Buffer,
    enums::{bg::Bg, fg::Fg, modifier::Modifier, wrap::Wrap},
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
    pub fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            text: text.as_ref().to_string(),
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

    /// Adds [`Span`] modifier to current modifiers
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier.push(modifier);
        self
    }

    /// Sets modifiers of [`Span`] to given modifiers
    pub fn modifiers(mut self, mods: Vec<Modifier>) -> Self {
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
        let (_, _) = self.get_offset(buffer, 0, None);
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
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<&Wrap>,
    ) -> Coords {
        let (res, coords) = self.get_offset(buffer, offset, wrap);
        print!("{res}");
        coords
    }

    fn get_offset(
        &self,
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<&Wrap>,
    ) -> (String, Coords) {
        let wrap = wrap.unwrap_or(&self.wrap);
        let (res, coords) = match wrap {
            Wrap::Letter => self.render_lines(buffer, offset, |t, b, o| {
                self.render_letter(t, b, o)
            }),
            Wrap::Word => self.render_lines(buffer, offset, |t, b, o| {
                self.render_word(t, b, o)
            }),
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
        buffer: &mut Buffer,
        offset: usize,
        text_render: F,
    ) -> (String, Coords)
    where
        F: Fn(&str, &mut Buffer, usize) -> Coords,
    {
        let mut res = String::new();
        let mut fin_coords = Coords::new(0, buffer.y());
        let mut coords = Coords::new(buffer.x(), buffer.y());
        let mut lsize = buffer.size();

        let mut offset = offset;
        for line in self.text.lines() {
            if lsize.y == 0 {
                break;
            }

            fin_coords = text_render(line, buffer, offset);
            (coords.x, coords.y) = (buffer.x(), fin_coords.y + 1);
            lsize.y = buffer.height().saturating_sub(coords.y - buffer.y());
            offset = 0;
        }
        (res, fin_coords)
    }

    /// Renders [`Span`] with word wrapping with given offset
    /// Returns [`Coords`] where rendered text ends
    fn render_word(
        &self,
        text: &str,
        buffer: &mut Buffer,
        offset: usize,
    ) -> Coords {
        // res.push_str(&Cursor::Pos(pos.x + offset, pos.y).to_string());
        let mut coords = Coords::new(offset, buffer.y());

        let mut line = Vec::<&str>::new();
        for word in text.split_whitespace() {
            if coords.x + word.len() + !line.is_empty() as usize
                > buffer.width()
            {
                if coords.y + 1 >= buffer.y() + buffer.height()
                    || word.len() > buffer.width()
                {
                    let mut line_str = line.join(" ");
                    let sum = coords.x + self.ellipsis.len();
                    if sum >= buffer.width() {
                        let end =
                            buffer.width().saturating_sub(self.ellipsis.len());
                        line_str = line_str[..end].to_string();
                    }

                    line_str.push_str(&self.ellipsis);
                    coords.x = line.len();
                    self.render_line(
                        buffer,
                        line_str,
                        &Coords::new(buffer.x(), coords.y),
                    );
                    return coords;
                }

                self.render_line(
                    buffer,
                    line.join(" "),
                    &Coords::new(buffer.x(), coords.y),
                );
                (coords.x, coords.y) = (0, coords.y + 1);
                line.clear();
            }
            coords.x += word.len() + !line.is_empty() as usize;
            line.push(word);
        }

        if !line.is_empty() {
            self.render_line(
                buffer,
                line.join(" "),
                &Coords::new(buffer.x(), coords.y),
            );
        }

        coords
    }

    /// Renders [`Span`] with letter wrapping with given offset
    /// Returns [`Coords`] where rendered text ended
    fn render_letter(
        &self,
        text: &str,
        buffer: &mut Buffer,
        offset: usize,
    ) -> Coords {
        let stext: String = text.chars().take(buffer.area()).collect();
        buffer.set_str(&stext, &Coords::new(buffer.x() + offset, buffer.y()));

        if stext.len() != text.len() && self.ellipsis.len() != 0 {
            let coords = Coords::new(
                (buffer.x() + buffer.width())
                    .saturating_sub(self.ellipsis.len()),
                (buffer.y() + buffer.height()).saturating_sub(1),
            );
            buffer.set_str(&self.ellipsis, &coords)
        }

        buffer.coords_of(stext.len() + offset)
    }

    /// Renders one line of text and aligns it based on set alignment
    fn render_line(&self, buffer: &mut Buffer, line: String, pos: &Coords) {
        match self.align {
            TextAlign::Left => buffer.set_str(line, pos),
            TextAlign::Center => {
                let offset = buffer.width().saturating_sub(line.len()) >> 1;
                buffer.set_str(line, &Coords::new(pos.x + offset, pos.y));
            }
            TextAlign::Right => {
                let offset = buffer.width().saturating_sub(line.len());
                buffer.set_str(line, &Coords::new(pos.x + offset, pos.y));
            }
        }
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

    /// Creates [`Span`] from string and sets its modifier to given value
    fn modifier(self, modifier: Modifier) -> Span;

    /// Creates [`Span`] from string and sets its modifier to given values
    fn modifiers(self, mods: Vec<Modifier>) -> Span;

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

    fn modifier(self, modifier: Modifier) -> Span {
        Span::new(self).modifier(modifier)
    }

    fn modifiers(self, mods: Vec<Modifier>) -> Span {
        Span::new(self).modifiers(mods)
    }

    fn align(self, align: TextAlign) -> Span {
        Span::new(self).align(align)
    }

    fn wrap(self, wrap: Wrap) -> Span {
        Span::new(self).wrap(wrap)
    }

    fn ellipsis<R>(self, ellipsis: R) -> Span
    where
        R: AsRef<str>,
    {
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
