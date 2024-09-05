/// Defines padding struct
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Padding {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

impl Padding {
    /// Creates a [`Padding`] by specifying every field
    pub const fn new(
        top: usize,
        right: usize,
        bottom: usize,
        left: usize,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Creates a [`Padding`] with same the value for all fields
    pub const fn uniform(value: usize) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Creates a [`Padding`] with `horizontal` value for `left` and `right`
    /// and `vertical` value for `top` and `bottom`
    pub const fn symmetric(horizontal: usize, vertical: usize) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Creates a [`Padding`] with the same value for `top` and `bottom` fields
    pub const fn vertical(value: usize) -> Self {
        Self {
            top: value,
            right: 0,
            bottom: value,
            left: 0,
        }
    }

    /// Creates a [`Padding`] with the same value for `left` and `right` fields
    pub const fn horizontal(value: usize) -> Self {
        Self {
            top: 0,
            right: value,
            bottom: 0,
            left: value,
        }
    }

    /// Creates a [`Padding`] that only sets the `top` padding
    pub const fn top(value: usize) -> Self {
        Self {
            top: value,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }

    /// Creates a [`Padding`] that only sets the `right` padding
    pub const fn right(value: usize) -> Self {
        Self {
            top: 0,
            right: value,
            bottom: 0,
            left: 0,
        }
    }

    /// Creates a [`Padding`] that only sets the `bottom` padding
    pub const fn bottom(value: usize) -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: value,
            left: 0,
        }
    }

    /// Creates a [`Padding`] that only sets the `left` padding
    pub const fn left(value: usize) -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: value,
        }
    }

    /// Gets total padding in vertical axis
    pub const fn get_vertical(&self) -> usize {
        self.top + self.bottom
    }

    /// Gets total padding in horizontal axis
    pub const fn get_horizontal(&self) -> usize {
        self.left + self.right
    }
}

impl Default for Padding {
    /// Creates new [`Padding`] with all paddding sides set to 0
    fn default() -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

impl From<usize> for Padding {
    /// Uses the value for all four sides
    fn from(value: usize) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

impl From<(usize, usize)> for Padding {
    /// Uses the first value for the top and bottom side,
    /// second for right and left
    fn from(value: (usize, usize)) -> Self {
        Self {
            top: value.0,
            right: value.1,
            bottom: value.0,
            left: value.1,
        }
    }
}

impl From<(usize, usize, usize, usize)> for Padding {
    /// Each value represent one side, starting from the top and continuing
    /// clockwise
    fn from(value: (usize, usize, usize, usize)) -> Self {
        Self {
            top: value.0,
            right: value.1,
            bottom: value.2,
            left: value.3,
        }
    }
}
