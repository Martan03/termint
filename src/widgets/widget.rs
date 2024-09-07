use std::fmt;

use crate::{buffer::Buffer, geometry::Vec2};

/// Trait for widgets to implement
pub trait Widget {
    /// Renders [`Widget`] on given position with given size
    fn render(&self, buffer: &mut Buffer);

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

pub struct Element(pub Box<dyn Widget>);

impl Widget for Element {
    fn render(&self, buffer: &mut Buffer) {
        self.0.render(buffer)
    }

    fn height(&self, size: &Vec2) -> usize {
        self.0.height(size)
    }

    fn width(&self, size: &Vec2) -> usize {
        self.0.width(size)
    }
}
