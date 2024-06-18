use super::coords::Coords;

/// Represents rectangle containing its position and size
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pos: Coords,
    size: Coords,
}

impl Rect {
    /// Creates new [`Rect`]
    pub fn new<T1, T2, T3, T4>(x: T1, y: T2, width: T3, height: T4) -> Self
    where
        T1: Into<usize>,
        T2: Into<usize>,
        T3: Into<usize>,
        T4: Into<usize>,
    {
        Self {
            pos: Coords::new(x.into(), y.into()),
            size: Coords::new(width.into(), height.into()),
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
        self.pos.x + self.size.x
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
    pub fn down(&self) -> usize {
        self.pos.y + self.size.y
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
