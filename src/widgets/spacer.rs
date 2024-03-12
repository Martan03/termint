use crate::geometry::coords::Coords;

use super::widget::Widget;

/// Spacer widget for better layouting
///
/// ## Example usage:
/// ```
/// # use termint::{
/// #     geometry::constrain::Constrain,
/// #     widgets::{layout::Layout, spacer::Spacer, span::StrSpanExtension},
/// # };
/// let mut layout = Layout::vertical();
/// layout.add_child("Example of Spacer".to_span(), Constrain::Min(0));
///
/// // Spacer creates one height space between spans
/// layout.add_child(Spacer::new(), Constrain::Length(1));
///
/// layout.add_child("One space above".to_span(), Constrain::Min(0));
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
    fn render(&self, _pos: &Coords, _size: &Coords) {}

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
