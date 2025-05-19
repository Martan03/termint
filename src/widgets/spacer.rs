use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
};

use super::{widget::Widget, Element};

/// A spacer widget used for layout spacing.
///
/// [`Spacer`] is useful for adding space between widgets in a [`Layout`] or
/// any other widget. [`Spacer`] can also be used for widgets to be empty, such
/// as in the [`BgGrad`] widget, where by default it contains [`Spacer`].
///
/// # Sizing
///
/// [`Spacer`] widget tries to be as small as possible. If you don't specify
/// the minimum size in the [`Layout`] (or other widget you use it in), its
/// size will be zero.
///
/// # Example
/// ```rust
/// # use termint::{
/// #     geometry::Constraint,
/// #     widgets::{Layout, Spacer, ToSpan},
/// # };
/// let mut layout = Layout::vertical();
/// layout.push("Top Widget", Constraint::Min(0));
///
/// // Insert a spacer with fixed height of 1
/// layout.push(Spacer::new(), Constraint::Length(1));
///
/// layout.push("Bottom Widget", Constraint::Min(0));
/// ```
///
/// In this example, there will be one line of space between the two texts.
#[derive(Debug, Default)]
pub struct Spacer {}

impl Spacer {
    /// Creates a new spacer widget.
    #[must_use]
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
