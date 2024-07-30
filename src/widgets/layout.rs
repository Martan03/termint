use std::cmp::{max, min};

use crate::{
    buffer::Buffer,
    enums::Color,
    geometry::{Constraint, Coords, Direction, Padding, Rect},
};

use super::widget::Widget;

/// Contains layout child and constraint of its size
#[derive(Debug)]
struct LayoutChild {
    pub child: Box<dyn Widget>,
    pub constraint: Constraint,
}

/// Creates layout flexing in one direction
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     geometry::{Constraint, Rect},
/// #     widgets::{Block, Layout, StrSpanExtension, Widget},
/// # };
/// // Creates horizontal layout containing two blocks each covering 50%
/// let block1 = Block::vertical().title("Block 1");
/// let block2 = Block::vertical().title("Block 2");
///
/// let mut layout = Layout::horizontal();
/// layout.add_child(block1, Constraint::Percent(50));
/// layout.add_child(block2, Constraint::Percent(50));
///
/// // Renders layout using buffer
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 20, 5));
/// layout.render(&mut buffer);
/// buffer.render();
/// ```
#[derive(Debug)]
pub struct Layout {
    direction: Direction,
    children: Vec<LayoutChild>,
    bg: Option<Color>,
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

    /// Sets background color of the [`Layout`]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.bg = bg.into();
        self
    }

    /// Sets [`Padding`] of the [`Layout`]
    pub fn padding<T: Into<Padding>>(mut self, padding: T) -> Self {
        self.padding = padding.into();
        self
    }

    /// Makes [`Layout`] center its content in its direction
    pub fn center(mut self) -> Self {
        self.center = true;
        self
    }

    /// Adds child with its [`Constrain`] to [`Layout`]
    pub fn add_child<T>(&mut self, child: T, constraint: Constraint)
    where
        T: Into<Box<dyn Widget>>,
    {
        self.children.push(LayoutChild {
            child: child.into(),
            constraint,
        });
    }
}

impl Widget for Layout {
    /// Renders [`Layout`] and its children inside of it
    fn render(&self, buffer: &mut Buffer) {
        let mut pos = Coords::new(
            buffer.x() + self.padding.left,
            buffer.y() + self.padding.top,
        );
        let mut size = Coords::new(
            buffer.width().saturating_sub(self.padding.get_horizontal()),
            buffer.height().saturating_sub(self.padding.get_vertical()),
        );

        if size.x == 0 || size.y == 0 || self.children.is_empty() {
            return;
        }

        match self.direction {
            Direction::Vertical => {
                pos.transpone();
                size.transpone();
                self._render(
                    buffer,
                    &mut pos,
                    &mut size,
                    |child, constrain, size| {
                        self.ver_child_size(child, constrain, size)
                    },
                );
            }
            Direction::Horizontal => {
                self._render(
                    buffer,
                    &mut pos,
                    &mut size,
                    |child, constrain, size| {
                        self.hor_child_size(child, constrain, size)
                    },
                );
            }
        }
    }

    /// Reverted to old implementation for now, which should work worse,
    /// but somehow it ends up better (better for widgets, which don't have
    /// fixed one of its side sizes, such as text)
    fn height(&self, size: &Coords) -> usize {
        let mut height = 0;
        for LayoutChild { child, constraint } in self.children.iter() {
            match constraint {
                Constraint::Length(len) => height += len,
                Constraint::Min(m) => height += max(*m, child.height(size)),
                Constraint::MinMax(mn, mx) => {
                    height += min(*mx, max(*mn, child.height(size)))
                }
                _ => height += child.height(size),
            }
        }
        height + self.padding.get_vertical()
    }

    /// Reverted to old implementation for now, which should work worse,
    /// but somehow it ends up better (better for widgets, which don't have
    /// fixed one of its side sizes, such as text)
    fn width(&self, size: &Coords) -> usize {
        let mut width = 0;
        for LayoutChild { child, constraint } in self.children.iter() {
            match constraint {
                Constraint::Length(len) => width += len,
                Constraint::Min(m) => width += max(*m, child.height(size)),
                Constraint::MinMax(mn, mx) => {
                    width += min(*mx, max(*mn, child.height(size)))
                }
                _ => width += child.height(size),
            }
        }
        width + self.padding.get_horizontal()
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            children: Vec::new(),
            bg: None,
            padding: Default::default(),
            center: false,
        }
    }
}

impl Layout {
    /// Renders layout
    fn _render<F>(
        &self,
        buffer: &mut Buffer,
        pos: &mut Coords,
        size: &mut Coords,
        child_size: F,
    ) where
        F: Fn(&dyn Widget, &Constraint, &Coords) -> usize,
    {
        self.render_bg(buffer);

        let (sizes, fill) = self.get_sizes(size, pos, child_size);
        let mut coords = *pos;
        for (i, s) in sizes.iter().enumerate() {
            if coords.x - pos.x >= size.x {
                break;
            }

            let mut child_size = match self.children[i].constraint {
                Constraint::Fill => Coords::new(fill, size.y),
                _ => Coords::new(*s, size.y),
            };
            if child_size.x + coords.x - pos.x > size.x {
                child_size.x = size.x.saturating_sub(coords.x - pos.x);
            }

            let mut c = coords;
            coords.x += child_size.x;
            if self.direction == Direction::Vertical {
                child_size.transpone();
                c.transpone();
            }

            let mut cbuffer =
                buffer.get_subset(Rect::from_coords(c, child_size));
            self.children[i].child.render(&mut cbuffer);
            buffer.union(cbuffer);
        }
    }

    /// Gets given child size based on its constraints (when vertical layout)
    fn ver_child_size(
        &self,
        child: &dyn Widget,
        constrain: &Constraint,
        size: &Coords,
    ) -> usize {
        let size = size.inverse();
        match constrain {
            Constraint::Length(len) => *len,
            Constraint::Percent(p) => size.y * p / 100,
            Constraint::Min(val) => max(child.height(&size), *val),
            Constraint::Max(val) => min(child.height(&size), *val),
            Constraint::MinMax(l, h) => min(max(child.height(&size), *l), *h),
            Constraint::Fill => 0,
        }
    }

    /// Gets given child size based on its constraints (when horizontal layout)
    fn hor_child_size(
        &self,
        child: &dyn Widget,
        constrain: &Constraint,
        size: &Coords,
    ) -> usize {
        match constrain {
            Constraint::Length(len) => *len,
            Constraint::Percent(p) => size.x * p / 100,
            Constraint::Min(val) => max(child.width(size), *val),
            Constraint::Max(val) => min(child.width(size), *val),
            Constraint::MinMax(l, h) => min(max(child.width(size), *l), *h),
            Constraint::Fill => 0,
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
        F: Fn(&dyn Widget, &Constraint, &Coords) -> usize,
    {
        let mut sizes: Vec<usize> = Vec::new();
        let mut total = 0;
        let mut fill = 0;
        for LayoutChild { child, constraint } in self.children.iter() {
            if constraint == &Constraint::Fill {
                fill += 1;
            }
            let csize = child_size(&**child, constraint, size);
            sizes.push(csize);
            total += csize;
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

    /// Renders [`Layout`] background color
    fn render_bg(&self, buffer: &mut Buffer) {
        let Some(bg) = self.bg else {
            return;
        };

        for y in buffer.y()..buffer.y() + buffer.height() {
            for x in buffer.x()..buffer.x() + buffer.width() {
                buffer.set_bg(bg, &Coords::new(x, y));
            }
        }
    }
}

// From implementations
impl From<Layout> for Box<dyn Widget> {
    fn from(value: Layout) -> Self {
        Box::new(value)
    }
}
