use crate::geometry::{Unit, Vec2};

#[derive(Debug)]
pub struct GridCache {
    pub size: Vec2,
    pub rows: Vec<Unit>,
    pub cols: Vec<Unit>,
    pub row_sizes: Vec<usize>,
    pub col_sizes: Vec<usize>,
}

impl GridCache {
    pub fn new(size: Vec2, cols: Vec<Unit>, rows: Vec<Unit>) -> Self {
        Self {
            size,
            rows,
            cols,
            row_sizes: vec![],
            col_sizes: vec![],
        }
    }

    pub fn sizes(mut self, cols: Vec<usize>, rows: Vec<usize>) -> Self {
        self.row_sizes = rows;
        self.col_sizes = cols;
        self
    }

    pub fn same_key(
        &self,
        size: &Vec2,
        cols: &Vec<Unit>,
        rows: &Vec<Unit>,
    ) -> bool {
        size == &self.size && cols == &self.cols && rows == &self.rows
    }
}
