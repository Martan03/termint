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
    thumb_style: Style,
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
    pub fn new(state: Rc<Cell<f64>>) -> Self {
        Self {
            state,
            thumb_style: Default::default(),
            style: Default::default(),
        }
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
        let progress = (self.state.get() / 100.).min(1.).max(0.);
        let thumb_len = (rect.width() as f64 * progress) as usize;
        let rest_len = rect.width().saturating_sub(thumb_len);
        buffer.set_str_styled(
            "█".repeat(thumb_len),
            rect.pos(),
            self.thumb_style,
        );
        let rrect =
            Rect::new(rect.x() + thumb_len, rect.y(), rest_len, rect.height());
        buffer.set_area_style(self.style, rrect);
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
