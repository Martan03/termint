/// Struct containing Padding on all four sides
#[derive(Debug)]
pub struct Padding {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

impl Padding {
    /// Creates new [`Padding`]
    pub fn new<T: Into<Padding>>(padding: T) -> Self {
        padding.into()
    }

    /// Gets total padding in vertical axis
    pub fn get_vertical(&self) -> usize {
        self.top + self.bottom
    }

    /// Gets total padding in horizontal axis
    pub fn get_horizontal(&self) -> usize {
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
    fn from(value: (usize, usize, usize, usize)) -> Self {
        Self {
            top: value.0,
            right: value.1,
            bottom: value.2,
            left: value.3,
        }
    }
}
