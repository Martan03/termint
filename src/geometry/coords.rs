use std::ops::{Add, Sub};

/// Contains x and y coordinates
#[derive(Debug)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    /// Creates new [`Coords`] containing given x and y coordinates
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    /// Transpones [`Coords`]
    pub fn transpone(&mut self) {
        (self.x, self.y) = (self.y, self.x);
    }
}

impl Add for Coords {
    type Output = Coords;

    /// Performs the `+` operation on [`Coords`]
    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coords {
    type Output = Coords;

    /// Performs the `-` operation on [`Coords`]
    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
