use crate::geometry::{constrain::Constrain, direction::Direction};

/// [`Layout`] helps creating layout for widgets
pub struct Layout {
    direction: Direction,
    constrain: Vec<Constrain>,
}

impl Layout {
    /// Creates new [`Layout`] that flexes in given direction
    pub fn new(direction: Direction, constrain: Vec<Constrain>) -> Self {
        Self {
            direction: direction,
            constrain: constrain,
        }
    }

    /// Create vertically flexed [`Layout`]
    pub fn vertical(constrain: Vec<Constrain>) -> Self {
        Self {
            direction: Direction::Vertical,
            constrain: constrain,
        }
    }

    /// Create horizontally flexed [`Layout`]
    pub fn horizontal(constrain: Vec<Constrain>) -> Self {
        Self {
            direction: Direction::Horizontal,
            constrain: constrain,
        }
    }
}
