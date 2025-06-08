use crate::geometry::{Constraint, Direction, Vec2};

#[derive(Debug)]
pub struct LayoutCache {
    pub size: Vec2,
    pub direction: Direction,
    pub constraints: Vec<Constraint>,
    pub sizes: Vec<usize>,
}

impl LayoutCache {
    pub fn new(
        size: Vec2,
        direction: Direction,
        constraints: Vec<Constraint>,
    ) -> Self {
        Self {
            size,
            direction,
            constraints,
            sizes: vec![],
        }
    }

    pub fn sizes(mut self, sizes: Vec<usize>) -> Self {
        self.sizes = sizes;
        self
    }

    pub fn same_key(
        &self,
        size: &Vec2,
        direction: &Direction,
        constraints: &Vec<Constraint>,
    ) -> bool {
        &self.size == size
            && &self.direction == direction
            && &self.constraints == constraints
    }
}
