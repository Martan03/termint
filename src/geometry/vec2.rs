use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub,
        SubAssign,
    },
};

use super::Vec2Range;

/// A 2D vector implementing basic operations
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vec2<T = usize> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    /// Creates new [`Vec2`] containing given x and y coordinates
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vec2<usize> {
    /// Saturating [`Vec2`] subtraction. Computes `self - rhs`, saturating at
    /// the numeric bounds instead of overlowing
    pub fn saturating_sub<T>(&self, rhs: T) -> Self
    where
        T: Into<Self>,
    {
        let rhs = rhs.into();
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }

    /// Checked [`Vec2`] subtraction. Computes `self - rhs`, returning `None`
    /// if overflow occured.
    pub fn checked_sub<T>(&self, rhs: T) -> Option<Self>
    where
        T: Into<Self>,
    {
        let rhs = rhs.into();
        Some(Self {
            x: self.x.checked_sub(rhs.x)?,
            y: self.y.checked_sub(rhs.y)?,
        })
    }
}

impl<T> Vec2<T>
where
    T: Copy + Into<f64>,
{
    /// Calculates magnituted of the 2D vector
    pub fn magnitude(&self) -> f64 {
        let x = self.x.into();
        let y = self.y.into();
        (x * x + y * y).sqrt()
    }

    /// Converts 2D vector to its normalized form (length equal to 1)
    pub fn normalize(&self) -> Vec2<f64> {
        let mag = self.magnitude();
        Vec2::new(self.x.into() / mag, self.y.into() / mag)
    }
}

impl<T> Vec2<T>
where
    T: Copy,
{
    /// Transpones [`Vec2`]
    pub fn transpone(&mut self) {
        (self.x, self.y) = (self.y, self.x);
    }

    /// Transpones [`Vec2`] and returns new [`Vec2`]
    pub fn inverse(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

impl<T> Vec2<T>
where
    T: Copy + PartialOrd,
{
    /// Creates new 2D vector range with first value inclusive and second
    /// exclusive
    pub fn to(self, other: Vec2<T>) -> Vec2Range<T> {
        Vec2Range::new(self, other)
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index {index} is out of bounds for Coords"),
        }
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index {index} is out of bounds for Coords"),
        }
    }
}

impl<T> PartialOrd for Vec2<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x < other.x && self.y < other.y {
            Some(std::cmp::Ordering::Less)
        } else if self.x > other.x && self.y > other.y {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl<T> Ord for Vec2<T>
where
    T: PartialOrd + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (
            self.x < other.x && self.y < other.y,
            self.x > other.x && self.y > other.y,
        ) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl<L, R> Add<Vec2<R>> for Vec2<L>
where
    L: Add<R>,
{
    type Output = Vec2<L::Output>;

    fn add(self, rhs: Vec2<R>) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<L, R> AddAssign<Vec2<R>> for Vec2<L>
where
    L: AddAssign<R>,
{
    fn add_assign(&mut self, rhs: Vec2<R>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<L, R> Sub<Vec2<R>> for Vec2<L>
where
    L: Add<R>,
{
    type Output = Vec2<L::Output>;

    fn sub(self, rhs: Vec2<R>) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<L, R> SubAssign<Vec2<R>> for Vec2<L>
where
    L: SubAssign<R>,
{
    fn sub_assign(&mut self, rhs: Vec2<R>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<L, R> Mul<Vec2<R>> for Vec2<L>
where
    L: Mul<R>,
{
    type Output = Vec2<L::Output>;

    fn mul(self, rhs: Vec2<R>) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<L, R> MulAssign<Vec2<R>> for Vec2<L>
where
    L: MulAssign<R>,
{
    fn mul_assign(&mut self, rhs: Vec2<R>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<L, R> Div<Vec2<R>> for Vec2<L>
where
    L: Div<R>,
{
    type Output = Vec2<L::Output>;

    fn div(self, rhs: Vec2<R>) -> Self::Output {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<L, R> DivAssign<Vec2<R>> for Vec2<L>
where
    L: DivAssign<R>,
{
    fn div_assign(&mut self, rhs: Vec2<R>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<T> Display for Vec2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
