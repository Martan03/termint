use crate::geometry::coords::Coords;

pub trait Widget {
    /// Renders [`Widget`] on given position with given size
    fn render(&self, pos: &Coords, size: &Coords);
}
