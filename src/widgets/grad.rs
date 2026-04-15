use core::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    enums::{Color, Modifier, RGB, Wrap},
    geometry::{Direction, TextAlign, Vec2},
    style::Style,
    text::{Line, StrStyle, Text, TextParser, get_step, text_render},
    widgets::layout::LayoutNode,
};

use super::{Element, widget::Widget};

/// A widget for rendering text with a gradient foreground color.
///
/// # Example
///
/// ```rust
/// use termint::{prelude::*, widgets::Grad};
///
/// // Text with blue-green foreground gradient
/// let grad = Grad::new("Hello Termint", (0, 0, 255), (0, 255, 0))
///     // Adds a white background
///     .bg(Color::White)
///     // Centers the text
///     .align(TextAlign::Center)
///     // Sets the wrapping to letter (new line after any character)
///     .wrap(Wrap::Letter)
///     // Adds `...` ellipsis (text shown when text overflows)
///     .ellipsis("...");
/// ```
///
/// [`Grad`] can also be used for printing the text directly to the terminal.
///
/// **Note**: text wrapping and ellipsis won't work in this mode, and the
/// gradient will be interpolated across the entire string length, rather than
/// per-line.
///
/// ```rust
/// use termint::widgets::Grad;
///
/// let grad = Grad::new(
///     "Printing gradient also works",
///     (0, 220, 255),
///     (200, 60, 255),
/// );
///
/// println!("{grad}");
/// ```
pub struct Grad {
    text: String,
    fg_start: RGB,
    fg_end: RGB,
    direction: Direction,
    bg: Option<Color>,
    modifier: Modifier,
    align: TextAlign,
    wrap: Wrap,
    ellipsis: String,
}

impl Grad {
    /// Creates a new [`Grad`] with the given text and start/end colors.
    ///
    /// The `start` and `end` colors can be any type convertible into [`RGB`],
    /// such as `u32`, `(u8 ,u8, u8)`. You can read more in the [`RGB`]
    /// documentation.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::Grad, enums::RGB};
    ///
    /// // You can use RGB constructors for the colors.
    /// let grad = Grad::new("Hello, World!",
    ///     RGB::new(0, 220, 255),
    ///     RGB::from_hex(0xC83CFF)
    /// );
    /// // Or any type convertible into `RGB`, such as tuple and `u32` (hex).
    /// let grad = Grad::new("Hello, Termint!", (0, 220, 255), 0xC83CFF);
    /// ```
    #[must_use]
    pub fn new<T, R, S>(text: T, start: R, end: S) -> Self
    where
        T: Into<String>,
        R: Into<RGB>,
        S: Into<RGB>,
    {
        Self {
            text: text.into(),
            fg_start: start.into(),
            fg_end: end.into(),
            direction: Direction::Horizontal,
            bg: None,
            modifier: Modifier::empty(),
            align: Default::default(),
            wrap: Default::default(),
            ellipsis: "...".to_string(),
        }
    }

    /// Sets the direction of the color gradient.
    ///
    /// The default direction is [`Direction::Horizontal`].
    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the background color of the [`Grad`].
    ///
    /// The `bg` can be any type convertible into `Option<Color>`. You can
    /// supply `None` for transparent background.
    #[must_use]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.bg = bg.into();
        self
    }

    /// Replaces the current text modifiers with the given modifers.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::Grad, modifiers};
    ///
    /// // Italic and Bold modifiers using the bitwise or for chaining.
    /// let grad = Grad::new("modifier", (0, 220, 255), 0xC83CFF)
    ///     .modifier(Modifier::ITALIC | Modifier::BOLD);
    /// // Or shorther using `modifiers!` macro
    /// let grad = Grad::new("modifier", (0, 220, 255), 0xC83CFF)
    ///     .modifier(modifiers!(BOLD, ITALIC));
    /// ```
    #[must_use]
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = Modifier::empty();
        self.modifier.insert(modifier);
        self
    }

    /// Adds a modifier to the existing set of modifiers.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::Grad};
    ///
    /// let grad = Grad::new("add_modifier", (0, 220, 255), 0xC83CFF)
    ///     // Sets modifiers to bold.
    ///     .modifier(Modifier::BOLD)
    ///     // Adds italic to the modifiers, resulting in italic bold text.
    ///     .add_modifier(Modifier::ITALIC);
    /// ```
    #[must_use]
    pub fn add_modifier(mut self, flag: Modifier) -> Self {
        self.modifier.insert(flag);
        self
    }

    /// Removes a specific from the current set of modifiers.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::Grad};
    ///
    /// let grad = Grad::new("remove_modifier", (0, 220, 255), 0xC83CFF)
    ///     // Makes text italic and bold.
    ///     .modifier(Modifier::ITALIC | Modifier::BOLD)
    ///     // Removes the italic modifier, resulting in only bold text.
    ///     .remove_modifier(Modifier::ITALIC);
    /// ```
    #[must_use]
    pub fn remove_modifier(mut self, flag: Modifier) -> Self {
        self.modifier.remove(flag);
        self
    }

    /// Sets the text alignment of the [`Grad`].
    ///
    /// The default alignment is [`TextAlign::Left`].
    #[must_use]
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the wrapping strategy of the [`Grad`].
    ///
    /// The default wrapping is [`Wrap::Word`], which wraps text only after
    /// a word. You can also use [`Wrap::Letter`], which wraps after any
    /// character.
    #[must_use]
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets the ellipsis string to use when text overflows.
    ///
    /// The default value is `"..."`.
    #[must_use]
    pub fn ellipsis(mut self, ellipsis: &str) -> Self {
        self.ellipsis = ellipsis.to_string();
        self
    }
}

impl<M: Clone + 'static> Widget<M> for Grad {
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        text_render(self, buffer, layout.area, &self.ellipsis, self.align);
    }

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.text.hash(&mut hasher);
        self.wrap.hash(&mut hasher);

        hasher.finish()
    }

    fn height(&self, size: &Vec2) -> usize {
        self.inner_height(size)
    }

    fn width(&self, size: &Vec2) -> usize {
        self.inner_width(size)
    }
}

impl Text for Grad {
    fn append_lines<'a>(
        &'a self,
        lines: &mut Vec<Line<'a>>,
        size: &Vec2,
        wrap: Option<Wrap>,
    ) -> bool {
        let wrap = wrap.unwrap_or(self.wrap);
        let mut parser = TextParser::new(&self.text).wrap(wrap);
        let frags = self.get_frags(&mut parser, lines, size);
        if frags.is_empty() {
            return true;
        }

        let fit = parser.is_end();
        match self.direction {
            Direction::Vertical => {
                self.get_lines_vert(lines, frags, parser, size)
            }
            Direction::Horizontal => self.get_lines_hor(lines, frags, fit),
        }
        fit
    }

    fn get(&self) -> String {
        let len = self.text.len().saturating_sub(1);
        let ((mut r, mut g, mut b), (rs, gs, bs)) =
            get_step(&self.fg_start, &self.fg_end, len);

        let mut res = format!(
            "{}{}",
            self.modifier,
            self.bg.map_or_else(|| "".to_string(), |bg| bg.to_bg())
        );

        for grapheme in self.text.graphemes(true) {
            let gw = grapheme.width();
            if gw == 0 {
                continue;
            }

            let color = Color::Rgb(r as u8, g as u8, b as u8);
            res += &color.to_fg();
            res += grapheme;

            let ssize = gw as f32;
            (r, g, b) = (r + rs * ssize, g + gs * ssize, b + bs * ssize);
        }
        res += "\x1b[0m";
        res
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_align(&self) -> TextAlign {
        self.align
    }
}

impl fmt::Display for Grad {
    /// Automatically converts [`Grad`] to String when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl Grad {
    fn inner_height(&self, size: &Vec2) -> usize {
        match self.wrap {
            Wrap::Letter => self.height_letter_wrap(size),
            Wrap::Word => self.height_word_wrap(size),
        }
    }

    fn inner_width(&self, size: &Vec2) -> usize {
        match self.wrap {
            Wrap::Letter => self.width_letter_wrap(size),
            Wrap::Word => self.width_word_wrap(size),
        }
    }

    fn get_frags<'a>(
        &self,
        parser: &mut TextParser<'a>,
        lines: &mut Vec<Line<'a>>,
        size: &Vec2,
    ) -> Vec<(&'a str, usize)> {
        let height = lines.len().saturating_sub(1);
        if size.x == 0 || height >= size.y || parser.is_end() {
            return vec![];
        }

        let mut frags = Vec::new();
        let last_width = lines.last().map(|l| l.width).unwrap_or_default();
        let mut fwidth = size.x.saturating_sub(last_width);

        for _ in height..size.y {
            let Some(line) = parser.next_line(fwidth) else {
                break;
            };
            frags.push(line);
            fwidth = size.x;
        }
        frags
    }

    /// Assumes frags is not empty, otherwise it will not work.
    fn get_lines_vert<'a>(
        &self,
        lines: &mut Vec<Line<'a>>,
        frags: Vec<(&'a str, usize)>,
        mut parser: TextParser<'a>,
        size: &Vec2,
    ) {
        let mut height = frags.len().saturating_sub(1);
        while let Some(_) = parser.next_line(size.x) {
            height += 1;
        }

        let ((mut r, mut g, mut b), (rs, gs, bs)) =
            get_step(&self.fg_start, &self.fg_end, height);
        let base_style = Style::new().bg(self.bg);

        let mut line = lines.pop().unwrap_or_else(Line::empty);
        for (text, len) in frags {
            let col = Color::Rgb(r as u8, g as u8, b as u8);
            let style = StrStyle::Static(base_style.fg(col));
            line.push(text, len, style);
            lines.push(line);

            line = Line::empty();
            (r, g, b) = (r + rs, g + gs, b + bs);
        }
    }

    /// Assumes frags is not empty, otherwise it will not work.
    fn get_lines_hor<'a>(
        &self,
        lines: &mut Vec<Line<'a>>,
        frags: Vec<(&'a str, usize)>,
        fits: bool,
    ) {
        let style = if frags.len() <= 1 && fits {
            StrStyle::LocalGrad(self.fg_start, self.fg_end)
        } else {
            StrStyle::GlobalGrad(self.fg_start, self.fg_end)
        };

        let mut line = lines.pop().unwrap_or_else(Line::empty);
        for (text, len) in frags {
            line.push(text, len, style.clone());
            lines.push(line);
            line = Line::empty();
        }
    }

    /// Gets height of the [`Grad`] when using word wrap
    fn height_word_wrap(&self, size: &Vec2) -> usize {
        let mut parser = TextParser::new(&self.text);

        let mut pos = Vec2::new(0, 0);
        loop {
            if parser.next_line(size.x).is_none() {
                break;
            }
            pos.y += 1;
        }
        pos.y
    }

    /// Gets width of the [`Grad`] when using word wrap
    fn width_word_wrap(&self, size: &Vec2) -> usize {
        let mut guess =
            Vec2::new(self.size_letter_wrap(size.y).saturating_sub(1), 0);

        while self.height_word_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets height of the [`Grad`] when using letter wrap
    fn height_letter_wrap(&self, size: &Vec2) -> usize {
        self.text
            .lines()
            .map(|l| {
                (l.chars().count() as f32 / size.x as f32).ceil() as usize
            })
            .sum()
    }

    /// Gets width of the [`Grad`] when using letter wrap
    fn width_letter_wrap(&self, size: &Vec2) -> usize {
        let mut guess = Vec2::new(self.size_letter_wrap(size.y), 0);
        while self.height_letter_wrap(&guess) > size.y {
            guess.x += 1;
        }
        guess.x
    }

    /// Gets size of the [`Grad`] when using letter wrap
    fn size_letter_wrap(&self, size: usize) -> usize {
        (self.text.chars().count() as f32 / size as f32).ceil() as usize
    }
}

// From implementations
impl<M: Clone + 'static> From<Grad> for Box<dyn Widget<M>> {
    fn from(value: Grad) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Grad> for Element<M> {
    fn from(value: Grad) -> Self {
        Element::new(value)
    }
}

impl<'a> From<Grad> for Box<dyn Text> {
    fn from(value: Grad) -> Self {
        Box::new(value)
    }
}
