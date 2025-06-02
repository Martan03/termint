use std::{cell::Cell, rc::Rc};

use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
    style::Style,
};

use super::{Element, Widget};

/// A widget visualizing progress
pub struct ProgressBar {
    state: Rc<Cell<f64>>,
    thumb_chars: Vec<char>,
    thumb_style: Style,
    track_char: char,
    style: Style,
}

impl ProgressBar {
    /// Creates a new [`ProgressBar`] with given percentage state.
    ///
    /// # Example
    /// ```rust
    /// # use std::{cell::Cell, rc::Rc};
    /// # use termint::widgets::ProgressBar;
    /// let state = Rc::new(Cell::new(69.0));
    /// let pb = ProgressBar::new(state.clone());
    /// ```
    #[must_use]
    pub fn new(state: Rc<Cell<f64>>) -> Self {
        Self {
            state,
            thumb_style: Default::default(),
            thumb_chars: vec!['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'],
            track_char: ' ',
            style: Default::default(),
        }
    }

    /// Sets the thumb characters used for rendering the progress.
    ///
    /// It can contain any number of character, but the iterator should start
    /// with the least progress character and end with most progress character.
    ///
    /// # Example
    /// ```rust
    /// # use std::{cell::Cell, rc::Rc};
    /// # use termint::widgets::ProgressBar;
    /// # let state = Rc::new(Cell::new(69.0));
    /// let pb = ProgressBar::new(state.clone())
    ///     .thumb_chars(['▎', '▌', '▊',, '█']);
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
    pub fn thumb_style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.thumb_style = style.into();
        self
    }

    /// Sets the character used for the track of the [`ProgressBar`].§
    #[must_use]
    pub fn track_char(mut self, track: char) -> Self {
        self.track_char = track;
        self
    }

    /// Sets the base style of the [`ProgressBar`].
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
}

impl Widget for ProgressBar {
    fn render(&self, buffer: &mut Buffer, rect: Rect) {
        let chars_len = self.thumb_chars.len();
        if rect.is_empty() || chars_len == 0 {
            return;
        }

        let progress = (self.state.get() / 100.0).clamp(0.0, 1.0);
        let len = rect.width() as f64 * progress;
        let full_cells = len.floor() as usize;

        let fraction = len - full_cells as f64;
        let head_id = (fraction * (chars_len - 1) as f64).round() as usize;

        let mut rest_len = rect.width().saturating_sub(full_cells);
        if head_id > 0 {
            rest_len = rest_len.saturating_sub(1);
        }

        let thumb = self.thumb_chars[chars_len - 1];
        buffer.set_str_styled(
            thumb.to_string().repeat(full_cells),
            rect.pos(),
            self.thumb_style,
        );

        let mut track_pos = Vec2::new(rect.x() + full_cells, rect.y());
        if head_id > 0 {
            buffer.set_val(self.thumb_chars[head_id], &track_pos);
            track_pos.x += 1;
        }

        buffer.set_str_styled(
            self.track_char.to_string().repeat(rest_len),
            &track_pos,
            self.style,
        );
    }

    fn height(&self, _size: &Vec2) -> usize {
        1
    }

    fn width(&self, size: &Vec2) -> usize {
        size.x
    }
}

impl From<ProgressBar> for Element {
    fn from(value: ProgressBar) -> Self {
        Element::new(value)
    }
}

impl From<ProgressBar> for Box<dyn Widget> {
    fn from(value: ProgressBar) -> Self {
        Box::new(value)
    }
}
