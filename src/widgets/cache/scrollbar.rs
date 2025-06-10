use crate::geometry::Direction;

pub struct ScrollbarCache {
    pub size: usize,
    pub direction: Direction,
    pub thumb_offset: usize,
    pub thumb_size: usize,
}

impl ScrollbarCache {
    pub fn new(size: usize, direction: Direction) -> Self {
        Self {
            size,
            direction,
            thumb_offset: 0,
            thumb_size: 0,
        }
    }

    pub fn thumb_offset(mut self, offset: usize) -> Self {
        self.thumb_offset = offset;
        self
    }

    pub fn thumb_size(mut self, size: usize) -> Self {
        self.thumb_size = size;
        self
    }

    pub fn same_key(&self, size: &usize, direction: &Direction) -> bool {
        size == &self.size && direction == &self.direction
    }
}
