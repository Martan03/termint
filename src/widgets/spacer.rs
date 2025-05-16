use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
};

use super::{widget::Widget, Element};

/// Spacer widget for better layouting
///
/// Can be used to add spaces to the layout, for example, between two widget.
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     geometry::Constraint,
/// #     widgets::{Layout, Spacer, ToSpan},
/// # };
/// let mut layout = Layout::vertical();
/// layout.push("Example of Spacer", Constraint::Min(0));
///
/// // Spacer creates one height space between spans
/// // Spacer size is set using [`Constrain`] when adding it to [`Layout`]
/// layout.push(Spacer::new(), Constraint::Length(1));
///
/// layout.push("One space above", Constraint::Min(0));
/// ```
#[derive(Debug, Default)]
pub struct Spacer {}

impl Spacer {
    /// Creates new spacer
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Spacer {
    fn render(&self, _buffer: &mut Buffer, _rect: Rect) {}

    fn height(&self, _size: &Vec2) -> usize {
        0
    }

    fn width(&self, _size: &Vec2) -> usize {
        0
    }
}

impl From<Spacer> for Box<dyn Widget> {
    fn from(value: Spacer) -> Self {
        Box::new(value)
    }
}

impl From<Spacer> for Element {
    fn from(value: Spacer) -> Self {
        Element::new(value)
    }
}
