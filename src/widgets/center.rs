use crate::geometry::coords::Coords;

use super::widget::Widget;

pub struct Center {
    child: Box<dyn Widget>,
    vertical: bool,
    horizontal: bool,
}

impl Center {
    /// Creates new [`Center`] with centering in both directions
    pub fn new<T>(child: T) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        Self {
            child: child.into(),
            vertical: true,
            horizontal: true,
        }
    }

    /// Creates new [`Center`] with horizontal centering
    pub fn horizontal<T>(child: T) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        Self {
            child: child.into(),
            vertical: false,
            horizontal: true,
        }
    }

    /// Creates new [`Center`] with vertical centering
    pub fn vertical<T>(child: T) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        Self {
            child: child.into(),
            vertical: true,
            horizontal: false,
        }
    }
}

impl Widget for Center {
    fn render(&self, pos: &Coords, size: &Coords) {
        todo!()
    }

    fn height(&self, size: &Coords) -> usize {
        todo!()
    }

    fn width(&self, size: &Coords) -> usize {
        todo!()
    }
}
