use std::fmt;

use crate::geometry::coords::Coords;

/// Trait for widgets to implement
pub trait Widget {
    /// Renders [`Widget`] on given position with given size
    fn render(&self, pos: &Coords, size: &Coords);

    /// Gets string representation of the [`Widget`]
    fn get_string(&self, pos: &Coords, size: &Coords) -> String;

    /// Gets height of the [`Widget`]
    fn height(&self, size: &Coords) -> usize;

    /// Gets width of the [`Widget`]
    fn width(&self, size: &Coords) -> usize;
}

impl fmt::Debug for dyn Widget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Converted widget")
    }
}
