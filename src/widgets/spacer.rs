use crate::{buffer::buffer::Buffer, geometry::coords::Coords};

use super::widget::Widget;

/// Spacer widget for better layouting
///
/// Can be used to add spaces to the layout, for example, between two widget.
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     geometry::constraint::Constraint,
/// #     widgets::{layout::Layout, spacer::Spacer, span::StrSpanExtension},
/// # };
/// let mut layout = Layout::vertical();
/// layout.add_child("Example of Spacer", Constraint::Min(0));
///
/// // Spacer creates one height space between spans
/// // Spacer size is set using [`Constrain`] when adding it to [`Layout`]
/// layout.add_child(Spacer::new(), Constraint::Length(1));
///
/// layout.add_child("One space above", Constraint::Min(0));
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
    fn render(&self, _buffer: &mut Buffer) {}

    fn height(&self, _size: &Coords) -> usize {
        0
    }

    fn width(&self, _size: &Coords) -> usize {
        0
    }
}

impl From<Spacer> for Box<dyn Widget> {
    fn from(value: Spacer) -> Self {
        Box::new(value)
    }
}
