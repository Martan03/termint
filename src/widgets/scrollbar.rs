use std::{cell::Cell, rc::Rc};

use crate::{
    buffer::Buffer,
    geometry::{Direction, Vec2, Vec2Range},
    style::Style,
};

use super::{Element, Widget};

/// Scrollbar widget that can be either in vertical or horizontal direction
///
/// In general, the scrollbar should be used by another widget, since it needs
/// the `content_len` to calculate the sizes by. But the `content_len` is known
/// only while rendering. For example, `Scrollable` widget uses the scrollbar
/// and sets the `content_len` before rendering the scrollbar.
///
/// ## Example usage:
/// ```rust
/// # use std::{cell::Cell, rc::Rc};
/// # use termint::{
/// #     buffer::Buffer,
/// #     geometry::Rect,
/// #     widgets::{Scrollbar, ScrollbarState, Widget}
/// # };
/// // Scrollbar state, with content_len set to fixed value to demonstrate
/// let state = Rc::new(Cell::new(ScrollbarState::new(3).content_len(30)));
///
/// // Creates new horizontal scrollbar
/// let scrollbar = Scrollbar::horizontal(state.clone());
///
/// // Renders using the buffer
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 10, 1));
/// scrollbar.render(&mut buffer);
/// buffer.render();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollbarState {
    pub content_len: usize,
    pub offset: usize,
}

impl Scrollbar {
    /// Creates new vertical [`Scrollbar`]
    pub fn vertical(state: Rc<Cell<ScrollbarState>>) -> Self {
        Self {
            state,
            ..Default::default()
        }
    }

    /// Creates new horizontal [`Scrollbar`]
    pub fn horizontal(state: Rc<Cell<ScrollbarState>>) -> Self {
        Self {
            direction: Direction::Horizontal,
            track_char: '─',
            thumb_char: '━',
            state,
            ..Default::default()
        }
    }

    /// Sets the track character of the [`Scrollbar`]
    pub fn track_char(mut self, track_char: char) -> Self {
        self.track_char = track_char;
        self
    }

    /// Sets [`Scrollbar`] track style to given value
    pub fn track_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.track_style = style.into();
        self
    }

    /// Sets the thumb character of the [`Scrollbar`]
    pub fn thumb_char(mut self, thumb_char: char) -> Self {
        self.thumb_char = thumb_char;
        self
    }

    /// Sets [`Scrollbar`] thumb style to given value
    pub fn thumb_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.thumb_style = style.into();
        self
    }

    /// Sets the direction of the [`Scrollbar`]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets [`ScrollbarState`] offset to given value
    pub fn offset(&self, offset: usize) {
        self.state.set(self.state.get().offset(offset));
    }

    /// Sets [`ScrollbarState`] content length to given value
    pub fn content_len(&self, content_len: usize) {
        self.state.set(self.state.get().content_len(content_len));
    }

    /// Gets a copy of the [`ScrollbarState`]
    pub fn get_state(&self) -> ScrollbarState {
        self.state.get()
    }
}

impl ScrollbarState {
    /// Creates a new [`ScrollbarState`]. The `content_len` is most often set
    /// by the widget using the scrollbar
    pub fn new(offset: usize) -> Self {
        Self {
            content_len: 0,
            offset: offset,
        }
    }

    /// Sets the offset of the [`ScrollbarState`]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the content length of the [`ScrollbarState`]
    pub fn content_len(mut self, content_len: usize) -> Self {
        self.content_len = content_len;
        self
    }

    /// Sets the scroll offset to the next position
    pub fn next(&mut self) {
        self.offset =
            (self.offset + 1).min(self.content_len.saturating_sub(1));
    }

    /// Sets the scroll offset to the previous position
    pub fn prev(&mut self) {
        self.offset = self.offset.saturating_sub(1);
    }

    /// Sets the scroll offset to the first position
    pub fn first(&mut self) {
        self.offset = 0;
    }

    /// Sets the scroll offset to the last position
    pub fn last(&mut self) {
        self.offset = self.content_len.saturating_sub(1);
    }
}

impl Widget for Scrollbar {
    fn render(&self, buffer: &mut Buffer) {
        match self.direction {
            Direction::Vertical => self.ver_render(buffer),
            Direction::Horizontal => self.hor_render(buffer),
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
    fn ver_render(&self, buffer: &mut Buffer) {
        let Some((size, pos)) = self.calc_thumb(buffer.height()) else {
            return;
        };

        self.render_track(
            buffer,
            buffer
                .pos()
                .to(Vec2::new(buffer.x() + 1, buffer.bottom() + 1)),
        );

        let start = Vec2::new(buffer.x(), buffer.y() + pos);
        let end = Vec2::new(buffer.x() + 1, buffer.y() + pos + size);
        self.render_thumb(buffer, start.to(end));
    }

    /// Renders the horizontal scrollbar
    fn hor_render(&self, buffer: &mut Buffer) {
        let Some((size, pos)) = self.calc_thumb(buffer.width()) else {
            return;
        };

        self.render_track(
            buffer,
            buffer
                .pos()
                .to(Vec2::new(buffer.right() + 1, buffer.y() + 1)),
        );

        let start = Vec2::new(buffer.x() + pos, buffer.y());
        let end = Vec2::new(buffer.x() + pos + size, buffer.y() + 1);
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
