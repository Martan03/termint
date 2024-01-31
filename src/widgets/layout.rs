use std::cmp::max;

use crate::geometry::{
    constrain::Constrain, coords::Coords, direction::Direction,
};

use super::widget::Widget;

/// Creates layout for widgets
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     geometry::{constrain::Constrain, coords::Coords},
/// #     widgets::{
/// #         block::Block, layout::Layout, span::StrSpanExtension,
/// #         widget::Widget,
/// #     },
/// # };
/// // Creates horizontal layout containing two blocks each covering 50%
/// let block1 = Block::new().title("Block 1".to_span());
/// let block2 = Block::new().title("Block 2".to_span());
///
/// let mut layout = Layout::horizontal();
/// layout.add_child(block1, Constrain::Percent(50));
/// layout.add_child(block2, Constrain::Percent(50));
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
    pub fn add_child<T>(&mut self, child: T, constrain: Constrain)
    where
        T: Widget + 'static,
    {
        self.children.push(Box::new(child));
        self.constrain.push(constrain);
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

impl Layout {
    /// Renders [`Layout`] in vertical [`Direction`]
    fn render_vertical(&self, pos: &Coords, size: &Coords) {
        let (sizes, total, fill) = self
            .get_sizes(size, |child, c, s| self.child_size_ver(child, c, s));

        let mut coords = Coords::new(pos.x, pos.y);

        for i in 0..self.children.len() {
            if coords.y - pos.y >= size.y {
                break;
            }

            let mut child_size = match self.constrain[i] {
                Constrain::Fill => {
                    Coords::new(size.x, (size.y.saturating_sub(total)) / fill)
                }
                _ => Coords::new(size.x, sizes[i]),
            };
            if child_size.y + coords.y - pos.y > size.y {
                child_size.y = size.y.saturating_sub(coords.y);
            }
            self.children[i].render(&coords, &child_size);

            coords.y += child_size.y;
        }
    }

    /// Renders [`Layout`] in horizontal [`Direction`]
    fn render_horizontal(&self, pos: &Coords, size: &Coords) {
        let (sizes, total, fill) = self
            .get_sizes(size, |child, c, s| self.child_size_hor(child, c, s));

        let mut coords = Coords::new(pos.x, pos.y);

        for i in 0..self.children.len() {
            if coords.x - pos.x >= size.x {
                break;
            }

            let mut child_size = match self.constrain[i] {
                Constrain::Fill => {
                    Coords::new((size.x.saturating_sub(total)) / fill, size.y)
                }
                _ => Coords::new(sizes[i], size.y),
            };
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
        size: &Coords,
    ) -> usize {
        match constrain {
            Constrain::Length(len) => *len,
            Constrain::Percent(p) => {
                let percent = (*p as f32 / 100.0 * size.y as f32) as usize;
                percent
            }
            Constrain::Min(val) => max(child.height(size), *val),
            Constrain::Fill => 0,
        }
    }

    /// Gets given child size in horizontal layout
    fn child_size_hor(
        &self,
        child: &Box<dyn Widget>,
        constrain: &Constrain,
        size: &Coords,
    ) -> usize {
        match constrain {
            Constrain::Length(len) => *len,
            Constrain::Percent(p) => {
                let percent = (*p as f32 / 100.0 * size.x as f32) as usize;
                percent
            }
            Constrain::Min(val) => max(child.width(size), *val),
            Constrain::Fill => 0,
        }
    }

    /// Gets children sizes
    fn get_sizes<F>(
        &self,
        size: &Coords,
        child_size: F,
    ) -> (Vec<usize>, usize, usize)
    where
        F: Fn(&Box<dyn Widget>, &Constrain, &Coords) -> usize,
    {
        let mut sizes: Vec<usize> = Vec::new();
        let mut total = 0;
        let mut fill = 0;
        for i in 0..self.children.len() {
            if self.constrain[i] == Constrain::Fill {
                fill += 1;
            }
            sizes.push(child_size(
                &self.children[i],
                &self.constrain[i],
                size,
            ));
            total += sizes[i];
        }

        (sizes, total, fill)
    }
}
