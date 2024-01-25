use crate::geometry::coords::Coords;

use super::border::Border;

pub struct Block {
    title: String,
    borders: u8,
}

impl Block {
    /// Creates new [`Block`] with given title and all borders
    pub fn new(title: String) -> Self {
        Self {
            title: title,
            borders: Border::ALL,
        }
    }

    /// Sets on which sides border of the [`Block`] should be rendered
    pub fn borders(mut self, borders: u8) -> Self {
        self.borders = borders;
        self
    }

    pub fn render(&self, size: Coords, pos: Coords) {
        for i in 1..size.x {

        }
    }
}
