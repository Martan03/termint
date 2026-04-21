use std::{cell::Cell, rc::Rc};

use crate::{
    buffer::Buffer,
    enums::Color,
    prelude::{MouseButton, MouseEvent, Rect, Vec2},
    style::{Style, Styleable},
    term::backend::MouseEventKind,
    text::Text,
    widgets::{Element, EventResult, LayoutNode, Widget},
};

type ProgressBarHandler<M> = Box<dyn Fn(f64) -> M>;

/// A widget that displays a horizontal progress bar.
///
/// The [`ProgressBar`] visually represents a percentage value in the range
/// `0.0` to `1.0`. It can be styled and configured to use custom characters.
///
/// # Terminology
///
/// - Thumb = the part of the bar representing completed progress.
/// - Track = the empty part of the bar.
///
/// # Default settings
///
/// By default, the thumb chars are `'▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'`.
/// This ensures that the progress bar feels smooth. The track is set to `' '`
/// (space).
///
/// # Example
/// ```rust
/// use termint::prelude::*;
/// use std::{cell::Cell, rc::Rc};
///
/// let state = Rc::new(Cell::new(0.69));
/// let pb = ProgressBar::new(state.clone())
///     .thumb_chars(['▎', '▌', '▊', '█'])
///     .blue()
///     .track_char('=')
///     .track_style(Color::White)
///     .on_click(|p| format!("Clicked at {p}!"));
///
/// // Setting general style can be done also using `Stylize` trait
/// let pb = ProgressBar::<()>::new(state.clone())
///     .red()
///     .on_black()
///     .underline();
/// ```
pub struct ProgressBar<M> {
    state: Rc<Cell<f64>>,
    thumb_chars: Vec<char>,
    style: Style,
    track_char: char,
    track_style: Style,
    label: Option<ProgressLabel>,
    handlers: Vec<(MouseButton, ProgressBarHandler<M>)>,
}

/// Represents the [`ProgressBar`] label.
pub enum ProgressLabel {
    /// Static text widget.
    Static(Box<dyn Text>),
    /// Closure that generates a new text widget based on the progress.
    Dynamic(Box<dyn Fn(f64) -> Box<dyn Text>>),
}

impl<M> ProgressBar<M> {
    /// Creates a new [`ProgressBar`] with given percentage state.
    ///
    /// The `state` should contain a value between `0.0` and `1.0`. The value
    /// will get clamped if outside of this range.
    ///
    /// # Example
    /// ```rust
    /// use termint::prelude::*;
    /// use std::{cell::Cell, rc::Rc};
    ///
    /// let state = Rc::new(Cell::new(0.69));
    /// let pb = ProgressBar::<()>::new(state.clone());
    /// ```
    #[must_use]
    pub fn new(state: Rc<Cell<f64>>) -> Self {
        Self {
            state,
            style: Default::default(),
            thumb_chars: vec!['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'],
            track_char: ' ',
            track_style: Default::default(),
            label: None,
            handlers: vec![],
        }
    }

    /// Sets the thumb characters used for rendering the progress.
    ///
    /// You should provide a sequence of characters representing increasing
    /// levels of progress ("fullness"). The last character is used for the
    /// fully filled sections, while the others are used for the fractional
    /// part at the tip.
    ///
    /// The default is `vec!['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█']`.
    ///
    /// # Example
    /// ```rust
    /// use termint::prelude::*;
    /// use std::{cell::Cell, rc::Rc};
    ///
    /// // The progress bar bellow will look like this (incrementing progress):
    /// // `█`
    /// // `█▎`
    /// // `█▌`
    /// // `█▊`
    /// // `██`
    /// let pb = ProgressBar::<()>::new(Default::default())
    ///     .thumb_chars(['▎', '▌', '▊', '█']);
    /// ```
    #[must_use]
    pub fn thumb_chars<C>(mut self, chars: C) -> Self
    where
        C: IntoIterator<Item = char>,
    {
        self.thumb_chars = chars.into_iter().collect();
        self
    }

    /// Sets the style of the [`ProgressBar`]'s thumb.
    ///
    /// You can provide any type convertible to [`Style`].
    #[must_use]
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the character used for the track of the [`ProgressBar`].
    ///
    /// The default is a space (`' '`).
    #[must_use]
    pub fn track_char(mut self, track: char) -> Self {
        self.track_char = track;
        self
    }

    /// Sets the base style of the [`ProgressBar`].
    ///
    /// You can provide any type convertible to [`Style`].
    #[must_use]
    pub fn track_style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.track_style = style.into();
        self
    }

    /// Sets a [`ProgressBar`] label to the given text widget.
    ///
    /// `text` can be any type convertible into `Box<dyn Text>>`.
    pub fn label<T>(mut self, text: T) -> Self
    where
        T: Into<Box<dyn Text>>,
    {
        self.label = Some(ProgressLabel::Static(text.into()));
        self
    }

    /// Sets a [`ProgressBar`] label to the given dynamic label.
    ///
    /// `builder` is a closure, which accepts the current progress (`f64`) and
    /// returns the label text widget.
    pub fn dyn_label<F>(mut self, builder: F) -> Self
    where
        F: Fn(f64) -> Box<dyn Text> + 'static,
    {
        self.label = Some(ProgressLabel::Dynamic(Box::new(builder)));
        self
    }

    /// Sets the message to return when the left mouse button is clicked.
    ///
    /// The `response` is a closure that receives the progress at the click
    /// (in range 0.0-1.0) and returns the corresponding message.
    ///
    /// If a handler for the left mouse button already exists, it will be
    /// replaced.
    ///
    /// This is a convenience wrapper around [`ProgressBar::on_press`].
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn on_click<F>(self, response: F) -> Self
    where
        F: Fn(f64) -> M + 'static,
    {
        self.on_press(MouseButton::Left, response)
    }

    /// Sets the message to return when the given [`MouseButton`] is clicked.
    ///
    /// The `response` is a closure that receives the progress at the click
    /// (in range 0.0-1.0) and returns the corresponding message.
    ///
    /// If a handler for the given mouse button already exists, it will be
    /// replaced.
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    ///
    /// # Example
    ///
    /// ```rust
    /// use termint::prelude::*;
    ///
    /// let pb = ProgressBar::new(Default::default())
    ///     .on_press(MouseButton::Middle, |p| format!("Clicked at {p}!"));
    /// ```
    #[must_use]
    pub fn on_press<F>(mut self, button: MouseButton, response: F) -> Self
    where
        F: Fn(f64) -> M + 'static,
    {
        self.handlers.retain(|(b, _)| *b != button);
        self.handlers.push((button, Box::new(response)));
        self
    }
}

impl<M: Clone + 'static> Widget<M> for ProgressBar<M> {
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        let rect = layout.area;
        if rect.is_empty() || self.thumb_chars.is_empty() {
            return;
        }

        let (full_cells, head_id) = self.calc_size(&rect);
        let mut rest_len = rect.width().saturating_sub(full_cells);

        let mut track_pos = Vec2::new(rect.x() + full_cells, rect.y());
        if head_id > 0 {
            rest_len = rest_len.saturating_sub(1);
            buffer[track_pos]
                .char(self.thumb_chars[head_id])
                .style(self.style);
            track_pos.x += 1;
        }

        let thumb = self.thumb_chars[self.thumb_chars.len() - 1];
        buffer.set_str_styled(
            thumb.to_string().repeat(full_cells),
            rect.pos(),
            self.style,
        );

        buffer.set_str_styled(
            self.track_char.to_string().repeat(rest_len),
            &track_pos,
            self.track_style,
        );

        self.render_label(buffer, rect, full_cells);
    }

    fn height(&self, _size: &Vec2) -> usize {
        1
    }

    fn width(&self, size: &Vec2) -> usize {
        size.x
    }

    fn on_event(&self, node: &LayoutNode, e: &MouseEvent) -> EventResult<M> {
        let area = node.area;
        if !area.contains_pos(&e.pos) {
            return EventResult::None;
        }

        match &e.kind {
            MouseEventKind::Down(button) => {
                let rx = e.pos.x.saturating_sub(area.x());
                let progress = (rx as f64 / area.width() as f64).clamp(0., 1.);
                self.handlers
                    .iter()
                    .find(|(b, _)| b == button)
                    .map(|(_, m)| EventResult::Response(m(progress)))
                    .unwrap_or(EventResult::None)
            }
            _ => EventResult::None,
        }
    }
}

impl<M> ProgressBar<M> {
    /// Calculates the size of full cells and head ID to get corresponding
    /// progress character with.
    fn calc_size(&self, rect: &Rect) -> (usize, usize) {
        let progress = self.state.get().clamp(0.0, 1.0);
        let len = rect.width() as f64 * progress;
        let full_cells = len.floor() as usize;

        let frac = len - full_cells as f64;
        let head_id = (frac * (self.thumb_chars.len() - 1) as f64).round();
        (full_cells, head_id as usize)
    }

    fn render_label(&self, buffer: &mut Buffer, rect: Rect, full: usize) {
        let mut render_text = |text: &Box<dyn Text>| {
            let mut lines = vec![];
            text.append_lines(&mut lines, rect.size(), None);
            let Some(line) = lines.first() else {
                return;
            };

            let align = text.get_align();
            line.render(buffer, rect, align);
            let offset = line.align_offset(&rect, align);
            self.recolor_label(buffer, &rect, line.width, offset, full);
        };

        match &self.label {
            Some(ProgressLabel::Static(text)) => {
                render_text(text);
            }
            Some(ProgressLabel::Dynamic(builder)) => {
                let progress = self.state.get().clamp(0.0, 1.0);
                render_text(&builder(progress));
            }
            _ => {}
        }
    }

    fn recolor_label(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        width: usize,
        offset: usize,
        full: usize,
    ) {
        let (thumb_color, track_color) = self.label_colors();
        let mut set_cols = |x, bg, fg| {
            let pos = Vec2::new(x, rect.y());
            if let Some(c) = bg {
                buffer[pos].bg = c;
            }
            if let Some(c) = fg {
                buffer[pos].fg = c;
            }
        };

        let base_x = rect.x() + offset;
        for x in base_x..base_x + width {
            if x < rect.x() + full {
                let fg = track_color.unwrap_or(Color::Black);
                set_cols(x, thumb_color, Some(fg));
            } else {
                set_cols(x, track_color, thumb_color);
            }
        }
    }

    fn label_colors(&self) -> (Option<Color>, Option<Color>) {
        let full_thumb_char = self.thumb_chars.last().copied().unwrap_or(' ');
        let thumb_color = if full_thumb_char.is_whitespace() {
            self.style.bg.or(self.style.fg)
        } else {
            self.style.fg.or(self.style.bg)
        };

        let track_color = if self.track_char.is_whitespace() {
            self.track_style.bg.or(self.track_style.fg)
        } else {
            self.track_style.fg.or(self.track_style.bg)
        };

        (thumb_color, track_color)
    }
}

impl<M> Styleable for ProgressBar<M> {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl<M: Clone + 'static> From<ProgressBar<M>> for Element<M> {
    fn from(value: ProgressBar<M>) -> Self {
        Element::new(value)
    }
}

impl<M: Clone + 'static> From<ProgressBar<M>> for Box<dyn Widget<M>> {
    fn from(value: ProgressBar<M>) -> Self {
        Box::new(value)
    }
}
