use std::fmt;

use crate::geometry::coords::Coords;

/// Trait for widgets to implement
pub trait Widget {
    /// Renders [`Widget`] on given position with given size
    fn render(&self, pos: &Coords, size: &Coords);
}

impl fmt::Debug for dyn Widget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Converted widget")
    }
}
