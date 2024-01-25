use crate::geometry::direction::Direction;

/// [`Layout`] helps creating layout for widgets
struct Layout {
    direction: Direction,
}

impl Layout {
    /// Creates new [`Layout`] that flexes in given direction
    pub fn new(direction: Direction) -> Self {
        Self {
            direction: direction,
        }
    }
}
