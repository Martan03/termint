use crate::geometry::{
    constrain::Constrain, coords::Coords, direction::Direction,
};

use super::widget::Widget;

/// [`Layout`] helps creating layout for widgets
#[derive(Debug)]
pub struct Layout {
    direction: Direction,
    constrain: Vec<Constrain>,
    children: Vec<Box<dyn Widget>>,
}

impl Layout {
    /// Creates new [`Layout`] that flexes in given [`Direction`]
    pub fn new(direction: Direction) -> Self {
        Self {
            direction: direction,
            constrain: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Creates [`Layout`] with vertical [`Direction`]
    pub fn vertical() -> Self {
        Self {
            direction: Direction::Vertical,
            constrain: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Creates [`Layout`] with horizontal [`Direction`]
    pub fn horizontal() -> Self {
        Self {
            direction: Direction::Horizontal,
            constrain: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Sets [`Direction`] of the [`Layout`]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Adds child with its [`Constrain`] to [`Layout`]
    pub fn add_child(&mut self, child: Box<dyn Widget>, constrain: Constrain) {
        self.children.push(child);
        self.constrain.push(constrain);
    }

    /// Renders [`Layout`] in vertical [`Direction`]
    fn render_vertical(&self, pos: &Coords, size: &Coords) {
        let mut coords = Coords::new(pos.x, pos.y);

        for i in 0..self.children.len() {
            let mut child_size = self.child_size(&self.constrain[i], size);
            if child_size.y + coords.y - pos.y > size.y {
                child_size.y = size.y.saturating_sub(coords.y);
            }
            self.children[i].render(&coords, &child_size);

            coords.y += child_size.y;
        }
    }

    /// Renders [`Layout`] in horizontal [`Direction`]
    fn render_horizontal(&self, pos: &Coords, size: &Coords) {
        let mut coords = Coords::new(pos.x, pos.y);
        let size = Coords::new(size.y, size.x);

        for i in 0..self.children.len() {
            let mut child_size = self.child_size(&self.constrain[i], &size);
            child_size.transpone();
            if child_size.x + coords.x - pos.x > size.y {
                child_size.x = size.y - coords.x;
            }

            self.children[i].render(&coords, &child_size);

            coords.x += child_size.x;
        }
    }

    /// Gets given child size in vertical layout
    fn child_size(&self, constrain: &Constrain, size: &Coords) -> Coords {
        match constrain {
            Constrain::Length(len) => Coords::new(size.x, *len),
            Constrain::Percent(p) => {
                let percent = (*p as f32 / 100.0 * size.y as f32) as usize;
                Coords::new(size.x, percent)
            }
        }
    }
}

impl Widget for Layout {
    /// Renders [`Layout`] and its children inside of it
    fn render(&self, pos: &Coords, size: &Coords) {
        if size.x == 0 || size.y == 0 {
            return;
        }

        match self.direction {
            Direction::Vertical => self.render_vertical(pos, size),
            Direction::Horizontal => self.render_horizontal(pos, size),
        }
    }
}
