use std::fmt;

use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
};

/// Trait for widgets to implement
pub trait Widget {
    /// Renders [`Widget`] on given position with given size
    fn render(&self, buffer: &mut Buffer, rect: Rect);

    /// Gets height of the [`Widget`]
    fn height(&self, size: &Vec2) -> usize;

    /// Gets width of the [`Widget`]
    fn width(&self, size: &Vec2) -> usize;
}

impl fmt::Debug for dyn Widget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Converted widget")
    }
}

#[derive(Debug)]
pub struct Element(Box<dyn Widget>);

impl Element {
    /// Creates new element
    pub fn new<W>(widget: W) -> Self
    where
        W: Widget + 'static,
    {
        Element(Box::new(widget))
    }
}

impl Widget for Element {
    fn render(&self, buffer: &mut Buffer, rect: Rect) {
        self.0.render(buffer, rect)
    }

    fn height(&self, size: &Vec2) -> usize {
        self.0.height(size)
    }

    fn width(&self, size: &Vec2) -> usize {
        self.0.width(size)
    }
}
