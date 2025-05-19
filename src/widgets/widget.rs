use std::fmt;

use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
};

/// Trait implemented by all the widgets.
///
/// A widget is a visual component that can render itself to a [`Buffer`] and
/// report its size requirements for layout purposes.
///
/// Use [`Element`] to store and manipulate widgets in a uniform way.
///
/// Users will use [`Widget`] trait directly only when implementing custom
/// widget, otherwise they will use built-in widgets like [`Span`], [`List`]
/// and so on.
pub trait Widget {
    /// Renders the widget into the given [`Buffer`] within the provided
    /// [`Rect`] bounds.
    fn render(&self, buffer: &mut Buffer, rect: Rect);

    /// Return the height of the [`Widget`] based on the width of the given
    /// size.
    fn height(&self, size: &Vec2) -> usize;

    /// Returns the width of the [`Widget`] based on the height of the given
    /// size.
    fn width(&self, size: &Vec2) -> usize;
}

impl fmt::Debug for dyn Widget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Converted widget")
    }
}

/// A container for any widget implementing the [`Widget`] trait.
///
/// This is the primary type used to store and manipulate widgets in layout
/// trees. `Element` wraps a widget in a dynamic trait object.
///
/// Use [`Element::new`] to convert a widget into an `Element`.
#[derive(Debug)]
pub struct Element(Box<dyn Widget>);

impl Element {
    /// Creates a new [`Element`] from a given widget.
    ///
    /// This is commonly used to wrap widgets when composing layouts.
    ///
    /// # Example
    /// ```
    /// # use termint::widgets::{Span, Element};
    /// let span = Span::new("Hello");
    /// let element = Element::new(label);
    /// ```
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
