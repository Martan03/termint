use std::cmp::max;

use crate::geometry::{
    constrain::Constrain, coords::Coords, direction::Direction,
    padding::Padding,
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
    padding: Padding,
    center: bool,
}

impl Layout {
    /// Creates new [`Layout`] that flexes in given [`Direction`]
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            ..Default::default()
        }
    }

    /// Creates [`Layout`] with vertical [`Direction`]
    pub fn vertical() -> Self {
        Default::default()
    }

    /// Creates [`Layout`] with horizontal [`Direction`]
    pub fn horizontal() -> Self {
        Self {
            direction: Direction::Horizontal,
            ..Default::default()
        }
    }

    /// Sets [`Direction`] of the [`Layout`]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets [`Padding`] of the [`Layout`]
    pub fn padding<T: Into<Padding>>(mut self, padding: T) -> Self {
        self.padding = padding.into();
        self
    }

    /// Makes [`Layout`] center its content
    pub fn center(mut self) -> Self {
        self.center = true;
        self
    }

    /// Adds child with its [`Constrain`] to [`Layout`]
    pub fn add_child<T>(&mut self, child: T, constrain: Constrain)
    where
        T: Into<Box<dyn Widget>>,
    {
        self.children.push(child.into());
        self.constrain.push(constrain);
    }
}

impl Widget for Layout {
    /// Renders [`Layout`] and its children inside of it
    fn render(&self, pos: &Coords, size: &Coords) {
        let mut pos =
            Coords::new(pos.x + self.padding.left, pos.y + self.padding.top);
        let mut size = Coords::new(
            size.x.saturating_sub(self.padding.get_horizontal()),
            size.y.saturating_sub(self.padding.get_vertical()),
        );

        if size.x == 0 || size.y == 0 {
            return;
        }

        match self.direction {
            Direction::Vertical => {
                pos.transpone();
                size.transpone();
                self._render(&mut pos, &mut size, |child, size| {
                    child.height(size)
                })
            }
            Direction::Horizontal => {
                self._render(&mut pos, &mut size, |child, size| {
                    child.width(size)
                })
            }
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

impl Default for Layout {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            constrain: Vec::new(),
            children: Vec::new(),
            padding: Default::default(),
            center: false,
        }
    }
}

impl Layout {
    fn _render<F>(&self, pos: &mut Coords, size: &mut Coords, chsize: F)
    where
        F: Fn(&dyn Widget, &Coords) -> usize,
    {
        let (sizes, fill) = self.get_sizes(size, pos, chsize);

        let mut coords = Coords::new(pos.x, pos.y);
        for (i, s) in sizes.iter().enumerate() {
            if coords.x - pos.x >= size.x {
                break;
            }

            let mut child_size = match self.constrain[i] {
                Constrain::Fill => Coords::new(fill, size.y),
                _ => Coords::new(*s, size.y),
            };
            if child_size.x + coords.x - pos.x > size.x {
                child_size.x = size.x.saturating_sub(coords.x);
            }

            let mut c = coords.clone();
            coords.x += child_size.x;
            if self.direction == Direction::Vertical {
                child_size.transpone();
                c.transpone();
            }
            self.children[i].render(&c, &child_size);
        }
    }

    fn child_size<F>(
        &self,
        child: &dyn Widget,
        chsize: F,
        constrain: &Constrain,
        size: &Coords,
    ) -> usize
    where
        F: Fn(&dyn Widget, &Coords) -> usize,
    {
        match constrain {
            Constrain::Length(len) => *len,
            Constrain::Percent(p) => {
                (*p as f32 / 100.0 * size.x as f32) as usize
            }
            Constrain::Min(val) => max(chsize(child, size), *val),
            Constrain::Fill => 0,
        }
    }

    /// Gets children sizes
    fn get_sizes<F>(
        &self,
        size: &mut Coords,
        pos: &mut Coords,
        child_size: F,
    ) -> (Vec<usize>, usize)
    where
        F: Fn(&dyn Widget, &Coords) -> usize,
    {
        let mut sizes: Vec<usize> = Vec::new();
        let mut total = 0;
        let mut fill = 0;
        for i in 0..self.children.len() {
            if self.constrain[i] == Constrain::Fill {
                fill += 1;
            }
            sizes.push(self.child_size(
                &*self.children[i],
                &child_size,
                &self.constrain[i],
                size,
            ));
            total += sizes[i];
        }

        let fills =
            size.x.saturating_sub(total).checked_div(fill).unwrap_or(0);
        if fills == 0 && self.center {
            let space_size = size.x.saturating_sub(total);
            let move_size = space_size / 2;

            size.x -= space_size;
            pos.x += move_size;
        }

        (sizes, fills)
    }
}

// From implementations
impl From<Layout> for Box<dyn Widget> {
    fn from(value: Layout) -> Self {
        Box::new(value)
    }
}
