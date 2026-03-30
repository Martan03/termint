use crate::prelude::Vec2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutBounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl LayoutBounds {
    pub const MIN: Vec2 = Vec2::new(0, 0);
    pub const MAX: Vec2 = Vec2::new(usize::MAX, usize::MAX);

    pub const NONE: LayoutBounds = LayoutBounds {
        min: Self::MIN,
        max: Self::MAX,
    };

    /// Creates new bounds based on the given ranges.
    pub fn new(min: impl Into<Vec2>, max: impl Into<Vec2>) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
        }
    }

    /// Creates bounds that are exactly the given size.
    pub fn exact(size: impl Into<Vec2>) -> Self {
        let size = size.into();
        Self::new(size, size)
    }

    /// Creates unbounded bounds.
    ///
    /// It uses `0` is minimum value and `usize::MAX` as the maximum value.
    pub fn unbounded() -> Self {
        Self::new(Self::MIN, Self::MAX)
    }

    /// Creates new bounds with the given upper bounds.
    pub fn at_most(max: impl Into<Vec2>) -> Self {
        Self::new(Self::MIN, max)
    }

    /// Creates new bounds with the given bottom bounds.
    pub fn at_least(min: impl Into<Vec2>) -> Self {
        Self::new(min, Self::MAX)
    }

    /// Clamps the given size to the current bounds.
    pub fn clamp(&self, size: impl Into<Vec2>) -> Vec2 {
        let size = size.into();
        Vec2::new(
            size.x.clamp(self.min.x, self.max.x),
            size.y.clamp(self.min.y, self.max.y),
        )
    }
}

impl<Size> From<Size> for LayoutBounds
where
    Size: Into<Vec2>,
{
    fn from(value: Size) -> Self {
        Self::exact(value.into())
    }
}

impl<Min, Max> From<(Min, Max)> for LayoutBounds
where
    Min: Into<Vec2>,
    Max: Into<Vec2>,
{
    fn from((min, max): (Min, Max)) -> Self {
        Self::new(min, max)
    }
}
