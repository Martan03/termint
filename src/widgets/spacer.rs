use crate::geometry::coords::Coords;

use super::widget::Widget;

/// None widget for better layouting
///
/// ## Example usage:
/// TODO
#[derive(Debug)]
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

impl Default for Spacer {
    fn default() -> Self {
        Self {}
    }
}

impl From<Spacer> for Box<dyn Widget> {
    fn from(value: Spacer) -> Self {
        Box::new(value)
    }
}
