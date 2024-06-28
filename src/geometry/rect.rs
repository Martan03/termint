use super::coords::Coords;

/// Represents rectangle containing its position and size
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pos: Coords,
    size: Coords,
}

impl Rect {
    /// Creates new [`Rect`]
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            pos: Coords::new(x, y),
            size: Coords::new(width, height),
        }
    }

    /// Creates new [`Rect`] from [`Coords`]
    pub fn from_coords<T1, T2>(pos: T1, size: T2) -> Self
    where
        T1: Into<Coords>,
        T2: Into<Coords>,
    {
        Self {
            pos: pos.into(),
            size: size.into(),
        }
    }

    /// Gets position of the [`Rect`]
    pub fn pos(&self) -> Coords {
        self.pos.clone()
    }

    /// Gets position of the [`Rect`] as reference
    pub fn pos_ref(&self) -> &Coords {
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

    /// Gets size of the [`Rect`]
    pub fn size(&self) -> Coords {
        self.size.clone()
    }

    /// Gets size of the [`Rect`] as reference
    pub fn size_ref(&self) -> &Coords {
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

    /// Gets area of the [`Rect`]
    pub fn area(&self) -> usize {
        self.size.x * self.size.y
    }
}
