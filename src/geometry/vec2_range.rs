use super::Vec2;

/// A range bounded by Vec2 inclusively below and exclusively above
/// (start <= x < end). It is empty if start >= end
pub struct Vec2Range<T = usize> {
    start: Vec2<T>,
    end: Vec2<T>,
    cur: Vec2<T>,
}

impl<T> Vec2Range<T>
where
    T: Copy + PartialOrd,
{
    /// Creates new [`Vec2`] range
    pub fn new(start: Vec2<T>, end: Vec2<T>) -> Self {
        Self {
            start,
            end,
            cur: start,
        }
    }

    /// Returns true if item is in the [`Vec2`] range
    pub fn contains(&self, item: &Vec2<T>) -> bool {
        &self.start <= item && item.x < self.end.x && item.y < self.end.y
    }
}

impl Iterator for Vec2Range<usize> {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.x >= self.end.x {
            self.cur.x = self.start.x;
            self.cur.y += 1;
        }

        if self.cur.y >= self.end.y {
            return None;
        }

        self.cur.x += 1;
        Some(Vec2::new(self.cur.x - 1, self.cur.y))
    }
}
