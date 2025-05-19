use std::{cell::Cell, rc::Rc};

use crate::{
    buffer::Buffer,
    geometry::{Direction, Rect, Vec2, Vec2Range},
    style::Style,
};

use super::{Element, Widget};

/// A scrollbar widget that can be either vertical or horizontal.
///
/// A [`Scrollbar`] is typically used in conjuction with another widget, such
/// as [`Scrollable`], which determines the scroll state ([`ScrollbarState`])
/// during rendering. The reason is the state contains content length, which is
/// only known while rendering. The state is then used to compute the thumb
/// size and position.
///
/// # Example:
/// ```rust
/// # use std::{cell::Cell, rc::Rc};
/// # use termint::{
/// #     buffer::Buffer,
/// #     geometry::Rect,
/// #     widgets::{Scrollbar, ScrollbarState, Widget},
/// #     term::Term,
/// # };
/// # fn example() -> Result<(), &'static str> {
/// // Scrollbar state with fixed content length and offset
/// let state = Rc::new(Cell::new(ScrollbarState::new(3).content_len(30)));
///
/// // Creates new horizontal scrollbar with the shared state
/// let scrollbar = Scrollbar::horizontal(state.clone());
///
/// let mut term = Term::new();
/// term.render(scrollbar)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Scrollbar {
    track_char: char,
    track_style: Style,
    thumb_char: char,
    thumb_style: Style,
    direction: Direction,
    state: Rc<Cell<ScrollbarState>>,
}

/// Represents the scroll state shared by a [`Scrollbar`] and the app itself.
///
///
/// In the events handling of the app, you can handle key events and change the
/// scroll offset.
///
/// Contains the current offset (scroll position) and total content length,
/// which are used to calculate scrollbar thumb position and size.
///
/// # Example
/// ```rust
/// # use termint::widgets::ScrollbarState;
/// let mut state = ScrollbarState::new(0).content_len(50);
/// state.next();
/// assert_eq!(state.offset, 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollbarState {
    pub content_len: usize,
    pub offset: usize,
}

impl Scrollbar {
    /// Creates a vertical [`Scrollbar`] with the given state.
    ///
    /// Uses `│` character for track and `┃` for thumb by default.
    ///
    /// # Example
    /// ```rust
    /// # use std::{cell::Cell, rc::Rc};
    /// # use termint::{
    /// #     buffer::Buffer,
    /// #     geometry::Rect,
    /// #     widgets::{Scrollbar, ScrollbarState, Widget}
    /// # };
    /// # let state = Rc::new(Cell::new(ScrollbarState::new(3)
    /// #    .content_len(30)));
    /// let scrollbar = Scrollbar::vertical(state);
    /// ```
    #[must_use]
    pub fn vertical(state: Rc<Cell<ScrollbarState>>) -> Self {
        Self {
            state,
            ..Default::default()
        }
    }

    /// Creates a horizontal [`Scrollbar`] with the given state.
    ///
    /// Uses `─` character for track and `━` for thumb by default.
    ///
    /// # Example
    /// ```rust
    /// # use std::{cell::Cell, rc::Rc};
    /// # use termint::{
    /// #     buffer::Buffer,
    /// #     geometry::Rect,
    /// #     widgets::{Scrollbar, ScrollbarState, Widget}
    /// # };
    /// # let state = Rc::new(Cell::new(ScrollbarState::new(3)
    /// #    .content_len(30)));
    /// let scrollbar = Scrollbar::horizontal(state);
    /// ```
    #[must_use]
    pub fn horizontal(state: Rc<Cell<ScrollbarState>>) -> Self {
        Self {
            direction: Direction::Horizontal,
            track_char: '─',
            thumb_char: '━',
            state,
            ..Default::default()
        }
    }

    /// Sets the character used to draw the [`Scrollbar`] track.
    #[must_use]
    pub fn track_char(mut self, track_char: char) -> Self {
        self.track_char = track_char;
        self
    }

    /// Sets the style of the [`Scrollbar`] track.
    #[must_use]
    pub fn track_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.track_style = style.into();
        self
    }

    /// Sets the character used to draw the [`Scrollbar`] thumb
    /// (the moving part).
    #[must_use]
    pub fn thumb_char(mut self, thumb_char: char) -> Self {
        self.thumb_char = thumb_char;
        self
    }

    /// Sets the style of the [`Scrollbar`] thumb.
    #[must_use]
    pub fn thumb_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.thumb_style = style.into();
        self
    }

    /// Sets the [`Direction`] of the [`Scrollbar`].
    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the scroll offset in the [`ScrollbarState`].
    pub fn offset(&self, offset: usize) {
        self.state.set(self.state.get().offset(offset));
    }

    /// Sets the total content length in the [`ScrollbarState`].
    pub fn content_len(&self, content_len: usize) {
        self.state.set(self.state.get().content_len(content_len));
    }

    /// Returns a copy of the current [`ScrollbarState`].
    pub fn get_state(&self) -> ScrollbarState {
        self.state.get()
    }
}

impl ScrollbarState {
    /// Creates a new [`ScrollbarState`] with the given offset.
    ///
    /// The content length defaults to zero.
    #[must_use]
    pub fn new(offset: usize) -> Self {
        Self {
            content_len: 0,
            offset,
        }
    }

    /// Sets the scroll offset.
    #[must_use]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the total content length.
    #[must_use]
    pub fn content_len(mut self, content_len: usize) -> Self {
        self.content_len = content_len;
        self
    }

    /// Increments the scroll offset by one, up to the end of the content.
    pub fn next(&mut self) {
        self.offset =
            (self.offset + 1).min(self.content_len.saturating_sub(1));
    }

    /// Decrements the scroll offset by one, down to zero.
    pub fn prev(&mut self) {
        self.offset = self.offset.saturating_sub(1);
    }

    /// Resets the scroll offset to the start (zero).
    pub fn first(&mut self) {
        self.offset = 0;
    }

    /// Sets the scroll offset to the last valid position.
    pub fn last(&mut self) {
        self.offset = self.content_len.saturating_sub(1);
    }
}

impl Widget for Scrollbar {
    fn render(&self, buffer: &mut Buffer, rect: Rect) {
        match self.direction {
            Direction::Vertical => self.ver_render(buffer, &rect),
            Direction::Horizontal => self.hor_render(buffer, &rect),
        }
    }

    fn height(&self, size: &Vec2) -> usize {
        match self.direction {
            Direction::Vertical => size.y,
            Direction::Horizontal => 1,
        }
    }

    fn width(&self, size: &Vec2) -> usize {
        match self.direction {
            Direction::Vertical => 1,
            Direction::Horizontal => size.x,
        }
    }
}

impl Scrollbar {
    /// Renders the vertical scrollbar
    fn ver_render(&self, buffer: &mut Buffer, rect: &Rect) {
        let Some((size, pos)) = self.calc_thumb(rect.height()) else {
            return;
        };

        self.render_track(
            buffer,
            rect.pos().to(Vec2::new(rect.x() + 1, rect.bottom() + 1)),
        );

        let start = Vec2::new(rect.x(), rect.y() + pos);
        let end = Vec2::new(rect.x() + 1, rect.y() + pos + size);
        self.render_thumb(buffer, start.to(end));
    }

    /// Renders the horizontal scrollbar
    fn hor_render(&self, buffer: &mut Buffer, rect: &Rect) {
        let Some((size, pos)) = self.calc_thumb(rect.width()) else {
            return;
        };

        self.render_track(
            buffer,
            rect.pos().to(Vec2::new(rect.right() + 1, rect.y() + 1)),
        );

        let start = Vec2::new(rect.x() + pos, rect.y());
        let end = Vec2::new(rect.x() + pos + size, rect.y() + 1);
        self.render_thumb(buffer, start.to(end));
    }

    /// Gets size of the thumb and its position
    fn calc_thumb(&self, visible: usize) -> Option<(usize, usize)> {
        let total = self.state.get().content_len;
        if total <= visible {
            return None;
        }

        let thumb_size =
            ((visible * visible) as f64 / total as f64).round() as usize;
        let max_offset = total.saturating_sub(visible);

        let mut state = self.state.get();
        if state.offset > max_offset {
            state = state.offset(max_offset);
            self.state.set(state);
        }

        let pos = (state.offset as f64 / max_offset as f64
            * (visible - thumb_size) as f64)
            .round() as usize;

        Some((thumb_size, pos))
    }

    /// Renders the scrollbar track
    fn render_track(&self, buffer: &mut Buffer, pos_range: Vec2Range) {
        for pos in pos_range {
            buffer[pos] =
                buffer[pos].val(self.track_char).style(self.track_style);
        }
    }

    /// Renders the scrollbar thumb
    fn render_thumb(&self, buffer: &mut Buffer, pos_range: Vec2Range) {
        for pos in pos_range {
            buffer[pos] =
                buffer[pos].val(self.thumb_char).style(self.thumb_style);
        }
    }
}

impl Default for Scrollbar {
    fn default() -> Self {
        Self {
            track_char: '│',
            track_style: Default::default(),
            thumb_char: '┃',
            thumb_style: Default::default(),
            direction: Default::default(),
            state: Default::default(),
        }
    }
}

impl From<Scrollbar> for Box<dyn Widget> {
    fn from(value: Scrollbar) -> Self {
        Box::new(value)
    }
}

impl From<Scrollbar> for Element {
    fn from(value: Scrollbar) -> Self {
        Element::new(value)
    }
}
