use crate::geometry::{Unit, Vec2};

#[derive(Debug)]
pub struct TableCache {
    pub size: Vec2,
    pub cols: Vec<Unit>,
    pub row_sizes: Vec<usize>,
    pub col_sizes: Vec<usize>,
    pub header_height: usize,
    pub scrollbar: bool,
}

impl TableCache {
    pub fn new(size: Vec2, cols: Vec<Unit>) -> Self {
        Self {
            size,
            cols,
            row_sizes: vec![],
            col_sizes: vec![],
            header_height: 0,
            scrollbar: false,
        }
    }

    pub fn sizes(mut self, cols: Vec<usize>, rows: Vec<usize>) -> Self {
        self.row_sizes = rows;
        self.col_sizes = cols;
        self
    }

    pub fn header_height(mut self, height: usize) -> Self {
        self.header_height = height;
        self
    }

    pub fn scrollbar(mut self, scrollbar: bool) -> Self {
        self.scrollbar = scrollbar;
        self
    }

    pub fn same_key(&self, size: &Vec2, cols: &Vec<Unit>) -> bool {
        size == &self.size && cols == &self.cols
    }
}
