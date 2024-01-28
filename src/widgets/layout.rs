use std::cmp::max;

use crate::geometry::{
    constrain::Constrain, coords::Coords, direction::Direction,
};

use super::widget::Widget;

/// Creates layout for widgets
///
/// ## Example usage:
/// ```rust
/// use termint::{
///     geometry::{constrain::Constrain, coords::Coords},
///     widgets::{
///         block::Block, layout::Layout, span::StrSpanExtension,
///         widget::Widget,
///     },
/// };
///
/// // Creates horizontal layout containing two blocks each covering 50%
/// let block1 = Block::new().title("Block 1".to_span());
/// let block2 = Block::new().title("Block 2".to_span());
///
/// let mut layout = Layout::horizontal();
/// layout.add_child(Box::new(block1), Constrain::Percent(50));
/// layout.add_child(Box::new(block2), Constrain::Percent(50));
///
/// // Renders layout on coordinates 1, 1 with width 20 and height 5
/// layout.render(&Coords::new(1, 1), &Coords::new(20, 5));
/// ```
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
            if coords.y - pos.y >= size.y {
                break;
            }

            let mut child_size = self.child_size_ver(
                &self.children[i],
                &self.constrain[i],
                coords.y - pos.y,
                size,
            );
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

        for i in 0..self.children.len() {
            if coords.x - pos.x >= size.x {
                break;
            }

            let mut child_size = self.child_size_hor(
                &self.children[i],
                &self.constrain[i],
                coords.x - pos.x,
                size,
            );
            if child_size.x + coords.x - pos.x > size.x {
                child_size.x = size.x.saturating_sub(coords.x);
            }

            self.children[i].render(&coords, &child_size);

            coords.x += child_size.x;
        }
    }

    /// Gets given child size in vertical layout
    fn child_size_ver(
        &self,
        child: &Box<dyn Widget>,
        constrain: &Constrain,
        used: usize,
        size: &Coords,
    ) -> Coords {
        match constrain {
            Constrain::Length(len) => Coords::new(size.x, *len),
            Constrain::Percent(p) => {
                let percent = (*p as f32 / 100.0 * size.y as f32) as usize;
                Coords::new(size.x, percent)
            },
            Constrain::Min(val) => {
                Coords::new(size.x, max(child.height(size), *val))
            },
            Constrain::Fill => Coords::new(size.x, size.y - used),
        }
    }

    /// Gets given child size in horizontal layout
    fn child_size_hor(
        &self,
        child: &Box<dyn Widget>,
        constrain: &Constrain,
        used: usize,
        size: &Coords,
    ) -> Coords {
        match constrain {
            Constrain::Length(len) => Coords::new(*len, size.y),
            Constrain::Percent(p) => {
                let percent = (*p as f32 / 100.0 * size.y as f32) as usize;
                Coords::new(percent, size.y)
            },
            Constrain::Min(val) => {
                Coords::new(max(child.width(size), *val), size.y)
            },
            Constrain::Fill => Coords::new(size.x - used, size.y),
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

    fn height(&self, size: &Coords) -> usize {
        let mut height = 0;
        for child in self.children.iter() {
            height += child.height(size);
        }
        height
    }

    fn width(&self, size: &Coords) -> usize {
        let mut width = 0;
        for child in self.children.iter() {
            width += child.width(size);
        }
        width
    }
}
