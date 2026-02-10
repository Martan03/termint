use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
    widgets::cache::Cache,
};

use super::{widget::Widget, Element};

/// A spacer widget used for layout spacing.
///
/// [`Spacer`] is useful for adding space between widgets in a
/// [`Layout`](crate::widgets::Layout) or any other widget. [`Spacer`] can also
/// be used for widgets to be empty, such as in the
/// [`BgGrad`](crate::widgets::BgGrad) widget, where by default it contains
/// [`Spacer`].
///
/// # Sizing
///
/// [`Spacer`] widget tries to be as small as possible. If you don't specify
/// the minimum size in the [`Layout`](crate::widgets::Layout) (or other widget
/// you use it in), its size will be zero.
///
/// # Example
///
/// In this example, there will be one line of space between the two texts.
///
/// ```rust
/// use termint::prelude::*;
///
/// let mut layout = Layout::<()>::vertical();
/// layout.push("Top Widget", Constraint::Min(0));
///
/// // Insert a spacer with fixed height of 1
/// layout.push(Spacer::new(), Constraint::Length(1));
///
/// layout.push("Bottom Widget", Constraint::Min(0));
/// ```
#[derive(Debug, Default)]
pub struct Spacer {}

impl Spacer {
    /// Creates a new spacer widget.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl<M: Clone + 'static> Widget<M> for Spacer {
    fn render(&self, _buffer: &mut Buffer, _rect: Rect, _cache: &mut Cache) {}

    fn height(&self, _size: &Vec2) -> usize {
        0
    }

    fn width(&self, _size: &Vec2) -> usize {
        0
    }
}

impl<M: Clone + 'static> From<Spacer> for Box<dyn Widget<M>> {
    fn from(value: Spacer) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Spacer> for Element<M> {
    fn from(value: Spacer) -> Self {
        Element::new(value)
    }
}
