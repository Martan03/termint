use crate::geometry::{
    constrain::Constrain, coords::Coords, direction::Direction,
};

use super::widget::Widget;

/// [`Layout`] helps creating layout for widgets
pub struct Layout {
    direction: Direction,
    constrain: Vec<Constrain>,
    children: Vec<Box<dyn Widget>>,
}

impl Layout {
    /// Creates new [`Layout`] that flexes in given [`Direction`]
    pub fn new(direction: Direction, constrain: Vec<Constrain>) -> Self {
        Self {
            direction: direction,
            constrain: constrain,
            children: Vec::new(),
        }
    }

    /// Creates [`Layout`] with vertical [`Direction`]
    pub fn vertical(constrain: Vec<Constrain>) -> Self {
        Self {
            direction: Direction::Vertical,
            constrain: constrain,
            children: Vec::new(),
        }
    }

    /// Creates [`Layout`] with horizontal [`Direction`]
    pub fn horizontal(constrain: Vec<Constrain>) -> Self {
        Self {
            direction: Direction::Horizontal,
            constrain: constrain,
            children: Vec::new(),
        }
    }

    /// Adds child with its [`Constrain`] to [`Layout`]
    pub fn child<T: Into<Box<dyn Widget>>>(
        &mut self,
        child: T,
        constrain: Constrain,
    ) {
        self.children.push(child.into());
        self.constrain.push(constrain);
    }

    /// Renders [`Layout`] in vertical [`Direction`]
    fn render_vertical(&self, pos: &Coords, size: &Coords) {
        let mut coords = Coords::new(pos.x, pos.y);

        for i in 0..self.children.len() {
            let child_size =
                self.child_size_vertical(&self.constrain[i], size);
            self.children[i].render(&coords, &child_size);
            coords.y += child_size.y;
        }
    }

    /// Gets given child size
    fn child_size_vertical(
        &self,
        constrain: &Constrain,
        size: &Coords,
    ) -> Coords {
        match constrain {
            Constrain::Length(len) => Coords::new(size.x, *len),
            Constrain::Percent(_) => todo!(),
        }
    }
}

impl Widget for Layout {
    /// Renders [`Layout`] and its children inside of it
    fn render(&self, pos: &Coords, size: &Coords) {
        match self.direction {
            Direction::Vertical => self.render_vertical(pos, size),
            Direction::Horizontal => todo!(),
        }
    }
}
