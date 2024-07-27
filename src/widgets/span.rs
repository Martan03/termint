use std::fmt;

use crate::{
    buffer::buffer::Buffer,
    enums::{modifier::Modifier, wrap::Wrap, Color},
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
/// #     buffer::buffer::Buffer,
/// #     enums::{Color, modifier::Modifier},
/// #     geometry::{coords::Coords, rect::Rect},
/// #     mods,
/// #     widgets::{
/// #         span::{Span, StrSpanExtension},
/// #         widget::Widget,
/// #     },
/// # };
///
/// // Creating span using new with red foreground:
/// let span = Span::new("Red text").fg(Color::Red);
/// // Creating span using &str conversion with red text and white background
/// let span = "Red text on white".fg(Color::Red).bg(Color::White);
///
/// // Cyan bold and italic text on yellow background
/// // Using macro for getting modifiers
/// let span = "Cyan bold and italic on yellow"
///     .fg(Color::Cyan)
///     .bg(Color::Yellow)
///     .modifiers(mods!(Bold, Italic));
///
/// // Span can be printed like this
/// println!("{span}");
///
/// // Or rendered using the buffer
/// // Text will be wrapping based on set value in wrap (Wrap::Word is default)
/// // Text will use ellipsis when can't fit ("..." is default)
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 10, 3));
/// span.render(&mut buffer);
/// buffer.render();
/// ```
#[derive(Debug)]
pub struct Span {
    text: String,
    fg: Option<Color>,
    bg: Option<Color>,
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
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.fg = fg.into();
        self
    }

    /// Sets background of [`Span`] to given color
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
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
        let _ = self.render_offset(buffer, 0, None);
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
        wrap: Option<Wrap>,
    ) -> Coords {
        let wrap = wrap.unwrap_or(self.wrap);
        match wrap {
            Wrap::Letter => {
                self.render_lines(buffer, offset, |t, b, ox, oy| {
                    self.render_letter(t, b, ox, oy)
                })
            }
            Wrap::Word => self.render_lines(buffer, offset, |t, b, ox, oy| {
                self.render_word(t, b, ox, oy)
            }),
        }
    }

    fn get(&self) -> String {
        format!("{}{}\x1b[0m", self.get_mods(), self.text)
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_mods(&self) -> String {
        let mut res = self
            .modifier
            .iter()
            .map(|m| m.to_ansi())
            .collect::<Vec<&str>>()
            .join("");
        if let Some(fg) = self.fg {
            res.push_str(&fg.to_fg());
        }
        if let Some(bg) = self.bg {
            res.push_str(&bg.to_bg());
        }
        res
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
    ) -> Coords
    where
        F: Fn(&str, &mut Buffer, usize, usize) -> Coords,
    {
        let mut fin_coords = Coords::new(0, buffer.y());
        let mut coords = Coords::new(buffer.x(), buffer.y());
        let mut lsize = buffer.size();

        let mut offset = offset;
        for line in self.text.lines() {
            if lsize.y == 0 {
                break;
            }

            fin_coords = text_render(line, buffer, offset, coords.y);
            (coords.x, coords.y) = (buffer.x(), fin_coords.y + 1);
            lsize.y = buffer.height().saturating_sub(coords.y - buffer.y());
            offset = 0;
        }
        fin_coords
    }

    /// Renders [`Span`] with word wrapping with given offset
    /// Returns [`Coords`] where rendered text ends
    fn render_word(
        &self,
        text: &str,
        buffer: &mut Buffer,
        offset_x: usize,
        offset_y: usize,
    ) -> Coords {
        let mut line = Vec::<&str>::new();
        let mut coords = Coords::new(offset_x, offset_y);

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
        offset_x: usize,
        offset_y: usize,
    ) -> Coords {
        let stext: String = text.chars().take(buffer.area()).collect();
        buffer.set_str_styled(
            &stext,
            &Coords::new(buffer.x() + offset_x, offset_y),
            self.fg,
            self.bg,
        );

        if stext.len() != text.len() && !self.ellipsis.is_empty() {
            let coords = Coords::new(
                (buffer.x() + buffer.width())
                    .saturating_sub(self.ellipsis.len()),
                (buffer.y() + buffer.height()).saturating_sub(1),
            );
            buffer.set_str_styled(&self.ellipsis, &coords, self.fg, self.bg)
        }

        buffer.coords_of(stext.len() + offset_x)
    }

    /// Renders one line of text and aligns it based on set alignment
    fn render_line(&self, buffer: &mut Buffer, line: String, pos: &Coords) {
        let x = match self.align {
            TextAlign::Left => pos.x,
            TextAlign::Center => {
                buffer.width().saturating_sub(line.len()) >> 1
            }
            TextAlign::Right => buffer.width().saturating_sub(line.len()),
        };
        buffer.set_str_styled(line, &Coords::new(x, pos.y), self.fg, self.bg);
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
    fn fg<T>(self, fg: T) -> Span
    where
        T: Into<Option<Color>>;

    /// Creates [`Span`] from string and sets its bg to given color
    fn bg<T>(self, bg: T) -> Span
    where
        T: Into<Option<Color>>;

    /// Creates [`Span`] from string and sets its modifier to given value
    fn modifier(self, modifier: Modifier) -> Span;

    /// Creates [`Span`] from string and sets its modifier to given values
    fn modifiers(self, mods: Vec<Modifier>) -> Span;

    /// Creates [`Span`] from string and sets its alignment to given value
    fn align(self, align: TextAlign) -> Span;

    /// Creates [`Span`] from string and sets its wrapping to given value
    fn wrap(self, wrap: Wrap) -> Span;

    /// Creates [`Span`] from string and sets its ellipsis to given value
    fn ellipsis<T>(self, ellipsis: T) -> Span
    where
        T: AsRef<str>;

    /// Converts &str to [`Span`]
    fn to_span(self) -> Span;
}

impl StrSpanExtension for &str {
    fn fg<T>(self, fg: T) -> Span
    where
        T: Into<Option<Color>>,
    {
        Span::new(self).fg(fg)
    }

    fn bg<T>(self, bg: T) -> Span
    where
        T: Into<Option<Color>>,
    {
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
        Box::new(Span::new(value.as_ref()))
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
