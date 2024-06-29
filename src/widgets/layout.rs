use std::cmp::{max, min};

use crate::{
    buffer::buffer::Buffer,
    enums::Color,
    geometry::{
        constraint::Constraint, coords::Coords, direction::Direction,
        padding::Padding, rect::Rect,
    },
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
    constraint: Vec<Constraint>,
    children: Vec<Box<dyn Widget>>,
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
        self.children.push(child.into());
        self.constraint.push(constraint);
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

    fn height(&self, size: &Coords) -> usize {
        match self.direction {
            Direction::Vertical => {
                let mut size = Coords::new(
                    size.x.saturating_sub(self.padding.get_horizontal()),
                    size.y,
                );
                size.transpone();
                self.get_size(&size, |child, constrain, size| {
                    self.ver_child_size(child, constrain, size)
                }) + self.padding.get_vertical()
            }
            Direction::Horizontal => self.height_horizontal(size),
        }
    }

    fn width(&self, size: &Coords) -> usize {
        match self.direction {
            Direction::Vertical => self.width_vertical(size),
            Direction::Horizontal => {
                let size = Coords::new(
                    size.x,
                    size.y.saturating_sub(self.padding.get_vertical()),
                );
                self.get_size(&size, |child, constrain, size| {
                    self.hor_child_size(child, constrain, size)
                }) + self.padding.get_horizontal()
            }
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            constraint: Vec::new(),
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
        println!(
            "sizes: {}",
            sizes
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

        let mut coords = *pos;
        for (i, s) in sizes.iter().enumerate() {
            if coords.x - pos.x >= size.x {
                break;
            }

            let mut child_size = match self.constraint[i] {
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
            println!("child size: [{} {}]", child_size.x, child_size.y);

            let mut cbuffer =
                buffer.get_subset(Rect::from_coords(c, child_size));
            self.children[i].render(&mut cbuffer);
            buffer.union(cbuffer);
        }
    }

    /// Gets total layout size
    fn get_size<F>(&self, size: &Coords, child_size: F) -> usize
    where
        F: Fn(&dyn Widget, &Constraint, &Coords) -> usize,
    {
        let mut total = 0;
        let mut fill = 0;
        for i in 0..self.children.len() {
            if self.constraint[i] == Constraint::Fill {
                fill += 1;
            }
            total += child_size(&*self.children[i], &self.constraint[i], size);
        }

        if fill > 0 {
            max(total, size.x)
        } else {
            total
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
            Constraint::Percent(p) => {
                (*p as f32 / 100.0 * size.y as f32) as usize
            }
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
        let test = match constrain {
            Constraint::Length(len) => *len,
            Constraint::Percent(p) => {
                (*p as f32 / 100.0 * size.x as f32) as usize
            }
            Constraint::Min(val) => max(child.width(size), *val),
            Constraint::Max(val) => min(child.width(size), *val),
            Constraint::MinMax(l, h) => min(max(child.width(size), *l), *h),
            Constraint::Fill => 0,
        };
        test
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
        for i in 0..self.children.len() {
            if self.constraint[i] == Constraint::Fill {
                fill += 1;
            }
            sizes.push(child_size(
                &*self.children[i],
                &self.constraint[i],
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

    fn height_horizontal(&self, size: &Coords) -> usize {
        let mut size = *size;
        let (sizes, fill) = self.get_sizes(
            &mut size,
            &mut Coords::new(1, 1),
            |child, constrain, size| {
                self.hor_child_size(child, constrain, size)
            },
        );

        let mut max = 2;
        for (i, s) in sizes.iter().enumerate() {
            max = std::cmp::max(
                match self.constraint[i] {
                    Constraint::Fill => fill,
                    _ => *s,
                },
                max,
            );
        }
        max
    }

    fn width_vertical(&self, size: &Coords) -> usize {
        let mut size = *size;
        let (sizes, fill) = self.get_sizes(
            &mut size,
            &mut Coords::new(1, 1),
            |child, constrain, size| {
                self.ver_child_size(child, constrain, size)
            },
        );

        let mut max = 2;
        for (i, s) in sizes.iter().enumerate() {
            max = std::cmp::max(
                match self.constraint[i] {
                    Constraint::Fill => fill,
                    _ => *s,
                },
                max,
            );
        }
        max
    }
}

// From implementations
impl From<Layout> for Box<dyn Widget> {
    fn from(value: Layout) -> Self {
        Box::new(value)
    }
}
