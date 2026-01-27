use crate::geometry::Rect;

#[derive(Debug)]
pub struct Frame {
    size: Rect,
}

impl Frame {
    /// Creates new frame with given size
    pub fn new(size: Rect) -> Self {
        Self { size }
    }

    /// Gets the size of the frame
    pub fn size(&self) -> Rect {
        self.size
    }
}
