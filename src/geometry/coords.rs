/// Contains x and y coordinates
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    /// Creates new [`Coords`] containing given x and y coordinates
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }
}
