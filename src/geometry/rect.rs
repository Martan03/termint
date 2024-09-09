use std::cmp::{max, min};

use super::{vec2::Vec2, Padding, Vec2Range};

/// A rectangular area containing its position and size
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pos: Vec2,
    size: Vec2,
}

impl Rect {
    /// Creates new [`Rect`]
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            pos: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }

    /// Creates new [`Rect`] from [`Vec2`] position and size
    pub fn from_coords<T1, T2>(pos: T1, size: T2) -> Self
    where
        T1: Into<Vec2>,
        T2: Into<Vec2>,
    {
        Self {
            pos: pos.into(),
            size: size.into(),
        }
    }

    /// Gets a new [`Rect`] after applying padding to the current one
    pub fn inner<T>(self, padding: T) -> Self
    where
        T: Into<Padding>,
    {
        let padding: Padding = padding.into();
        Self {
            pos: Vec2::new(self.x() + padding.left, self.y() + padding.top),
            size: Vec2::new(
                self.width().saturating_sub(padding.get_horizontal()),
                self.height().saturating_sub(padding.get_vertical()),
            ),
        }
    }

    /// Creates a new [`Rect`] that contains both current and given
    pub fn union(self, other: &Self) -> Self {
        let min_x = min(self.x(), other.x());
        let min_y = min(self.y(), other.y());
        let max_x = max(self.right() + 1, other.right() + 1);
        let max_y = max(self.bottom() + 1, other.bottom() + 1);

        Self {
            pos: Vec2::new(min_x, min_y),
            size: Vec2::new(
                max_x.saturating_sub(min_x),
                max_y.saturating_sub(min_y),
            ),
        }
    }

    /// Creates a new [`Rect`] that is the intersection of current and given
    pub fn intersection(self, other: &Self) -> Self {
        let max_x = max(self.x(), other.x());
        let max_y = max(self.y(), other.y());
        let min_x = min(self.right() + 1, other.right() + 1);
        let min_y = min(self.bottom() + 1, other.bottom() + 1);

        Self {
            pos: Vec2::new(max_x, max_y),
            size: Vec2::new(
                min_x.saturating_sub(max_x),
                min_y.saturating_sub(max_y),
            ),
        }
    }

    /// Moves [`Rect`] to given position
    pub fn move_to(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    /// Gets reference to position of the [`Rect`]
    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }

    /// Gets x coordinate of the [`Rect`]
    pub fn x(&self) -> usize {
        self.pos.x
    }

    /// Gets x coordinate of the [`Rect`]
    pub fn left(&self) -> usize {
        self.pos.x
    }

    /// Gets x coordinate of the most right cell of the [`Rect`]
    pub fn right(&self) -> usize {
        (self.pos.x + self.size.x).saturating_sub(1)
    }

    /// Gets y coordinate of the [`Rect`]
    pub fn y(&self) -> usize {
        self.pos.y
    }

    /// Gets y coordinate of the [`Rect`]
    pub fn top(&self) -> usize {
        self.pos.y
    }

    /// Gets y coordinate of the most bottom cell of the [`Rect`]
    pub fn bottom(&self) -> usize {
        (self.pos.y + self.size.y).saturating_sub(1)
    }

    /// Gets top left corner position
    pub fn top_left(&self) -> Vec2 {
        *self.pos()
    }

    /// Gets top right corner position
    pub fn top_right(&self) -> Vec2 {
        Vec2::new(self.right(), self.top())
    }

    /// Gets bottom left corner position
    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.x(), self.bottom())
    }

    /// Gets bottom right corner position
    pub fn bottom_right(&self) -> Vec2 {
        Vec2::new(self.right(), self.bottom())
    }

    /// Gets reference to size of the [`Rect`]
    pub fn size(&self) -> &Vec2 {
        &self.size
    }

    /// Gets width of the [`Rect`]
    pub fn width(&self) -> usize {
        self.size.x
    }

    /// Gets height of the [`Rect`]
    pub fn height(&self) -> usize {
        self.size.y
    }

    /// Returns true if current [`Rect`] contains the given one
    pub fn contains(&self, other: &Self) -> bool {
        !other.is_empty()
            && self.x() <= other.x()
            && self.y() <= other.y()
            && self.right() >= other.right()
            && self.bottom() >= other.bottom()
    }

    /// Returns true if current [`Rect`] contains the given position
    pub fn contains_pos(&self, pos: &Vec2) -> bool {
        self.x() <= pos.x
            && pos.x <= self.right()
            && self.y() <= pos.y
            && pos.y <= self.bottom()
    }

    /// Returns true if current [`Rect`] intersects the given one
    pub fn intersects(&self, other: &Self) -> bool {
        (self.x() < other.right() && self.right() > other.x())
            || (self.y() < other.bottom() && self.bottom() > other.x())
    }

    /// Gets area of the [`Rect`]
    pub const fn area(&self) -> usize {
        self.size.x * self.size.y
    }

    /// Returns true if the [`Rect`] is empty (no area)
    pub const fn is_empty(&self) -> bool {
        self.size.x == 0 || self.size.y == 0
    }
}

impl From<(Vec2, Vec2)> for Rect {
    fn from((pos, size): (Vec2, Vec2)) -> Self {
        Self { pos, size }
    }
}

impl From<(usize, usize, Vec2)> for Rect {
    fn from((x, y, size): (usize, usize, Vec2)) -> Self {
        Self {
            pos: Vec2::new(x, y),
            size,
        }
    }
}

impl From<(Vec2, usize, usize)> for Rect {
    fn from((pos, width, height): (Vec2, usize, usize)) -> Self {
        Self {
            pos,
            size: Vec2::new(width, height),
        }
    }
}

impl From<(usize, usize, usize, usize)> for Rect {
    fn from((x, y, width, height): (usize, usize, usize, usize)) -> Self {
        Self::new(x, y, width, height)
    }
}

impl IntoIterator for Rect {
    type Item = Vec2;
    type IntoIter = Vec2Range;

    fn into_iter(self) -> Self::IntoIter {
        Vec2Range::new(
            self.pos,
            Vec2::new(self.x() + self.width(), self.y() + self.height()),
        )
    }
}
