use std::cmp::{max, min};

use crate::{
    buffer::Buffer,
    enums::Color,
    geometry::{Constraint, Direction, Padding, Rect, Vec2},
    style::Style,
};

use super::{widget::Widget, Element};

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
    style: Style,
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

    /// Sets the base style of the [`Layout`]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets base background color of the [`Layout`]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.bg(bg);
        self
    }

    /// Sets base foreground color of the [`Layout`]
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.fg(fg);
        self
    }

    /// Sets [`Padding`] of the [`Layout`]
    pub fn padding<T>(mut self, padding: T) -> Self
    where
        T: Into<Padding>,
    {
        self.padding = padding.into();
        self
    }

    /// Makes [`Layout`] center its content in its direction
    pub fn center(mut self) -> Self {
        self.center = true;
        self
    }

    /// Adds child with its [`Constraint`] to [`Layout`]
    #[deprecated(
        since = "0.6.0",
        note = "Kept for compatibility purposes; use `push` function instead"
    )]
    pub fn add_child<T, C>(&mut self, child: T, constraint: C)
    where
        T: Into<Box<dyn Widget>>,
        C: Into<Constraint>,
    {
        self.children.push(LayoutChild {
            child: child.into(),
            constraint: constraint.into(),
        });
    }

    /// Pushes child with its [`Constraint`] to the [`Layout`]
    pub fn push<T, C>(&mut self, child: T, constraint: C)
    where
        T: Into<Box<dyn Widget>>,
        C: Into<Constraint>,
    {
        self.children.push(LayoutChild {
            child: child.into(),
            constraint: constraint.into(),
        });
    }
}

impl Widget for Layout {
    /// Renders [`Layout`] and its children inside of it
    fn render(&self, buffer: &mut Buffer) {
        let rect = buffer.rect().inner(self.padding);
        if rect.width() == 0 || rect.height() == 0 {
            return;
        }

        self.render_base_style(buffer);
        if self.children.is_empty() {
            return;
        }

        let mut cbuffer = buffer.subset(rect);
        match self.direction {
            Direction::Vertical => self.ver_render(&mut cbuffer, rect),
            Direction::Horizontal => self.hor_render(&mut cbuffer, rect),
        }
        buffer.merge(cbuffer);
    }

    /// Reverted to old implementation for now, which should work worse,
    /// but somehow it ends up better (better for widgets, which don't have
    /// fixed one of its side sizes, such as text)
    fn height(&self, size: &Vec2) -> usize {
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
    fn width(&self, size: &Vec2) -> usize {
        let mut width = 0;
        for LayoutChild { child, constraint } in self.children.iter() {
            match constraint {
                Constraint::Length(len) => width += len,
                Constraint::Min(m) => width += max(*m, child.width(size)),
                Constraint::MinMax(mn, mx) => {
                    width += min(*mx, max(*mn, child.width(size)))
                }
                _ => width += child.width(size),
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
            style: Style::new(),
            padding: Default::default(),
            center: false,
        }
    }
}

impl Layout {
    /// Renders layout
    fn ver_render(&self, buffer: &mut Buffer, rect: Rect) {
        let (sizes, mut rect) = self.ver_sizes(rect);
        for (i, s) in sizes.iter().enumerate() {
            let mut csize = *s;
            if csize > rect.height() {
                csize = rect.height().saturating_sub(rect.y() - buffer.y())
            }

            let mut cbuffer = buffer.subset(Rect::from_coords(
                *rect.pos(),
                Vec2::new(rect.width(), csize),
            ));
            self.children[i].child.render(&mut cbuffer);
            buffer.merge(cbuffer);
            rect = rect.inner(Padding::top(csize));
        }
    }

    /// Renders layout
    fn hor_render(&self, buffer: &mut Buffer, rect: Rect) {
        let (sizes, mut rect) = self.hor_sizes(rect);
        for (i, s) in sizes.iter().enumerate() {
            let mut csize = *s;
            if csize > rect.width() {
                csize -= csize - rect.width();
            }

            let mut cbuffer = buffer.subset(Rect::from_coords(
                *rect.pos(),
                Vec2::new(csize, rect.height()),
            ));
            self.children[i].child.render(&mut cbuffer);
            buffer.merge(cbuffer);
            rect = rect.inner(Padding::left(csize));
        }
    }

    /// Gets child sizes of vertical layout
    fn ver_sizes(&self, rect: Rect) -> (Vec<usize>, Rect) {
        self.child_sizes(
            rect,
            rect.height(),
            |c, s| c.height(s),
            |s, v| s.y = s.y.saturating_sub(v),
            |s| s.y,
            |r, s| r.inner((s, 0)),
        )
    }

    /// Gets child sizes of horizontal layout
    fn hor_sizes(&self, rect: Rect) -> (Vec<usize>, Rect) {
        self.child_sizes(
            rect,
            rect.width(),
            |c, s| c.width(s),
            |s, v| s.x = s.x.saturating_sub(v),
            |s| s.x,
            |r, s| r.inner((0, s)),
        )
    }

    /// Gets sizes of all the children
    fn child_sizes<F1, F2, F3, F4>(
        &self,
        rect: Rect,
        percent: usize,
        csize: F1,
        shrink: F2,
        left: F3,
        inner: F4,
    ) -> (Vec<usize>, Rect)
    where
        F1: Fn(&Box<dyn Widget>, &Vec2) -> usize,
        F2: Fn(&mut Vec2, usize),
        F3: Fn(Vec2) -> usize,
        F4: Fn(Rect, usize) -> Rect,
    {
        let mut fill_ids = Vec::new();
        let mut fills = 0;
        let mut sizes = Vec::new();
        let mut size = *rect.size();

        for LayoutChild { child, constraint } in self.children.iter() {
            let csize = match constraint {
                Constraint::Length(len) => *len,
                Constraint::Percent(p) => percent * p / 100,
                Constraint::Min(l) => max(csize(child, &size), *l),
                Constraint::Max(h) => min(csize(child, &size), *h),
                Constraint::MinMax(l, h) => {
                    min(max(csize(child, &size), *l), *h)
                }
                Constraint::Fill(val) => {
                    fill_ids.push(sizes.len());
                    sizes.push(*val);
                    fills += val;
                    continue;
                }
            };
            sizes.push(csize);
            shrink(&mut size, csize);
        }

        let mut left = left(size);
        if fills == 0 && self.center {
            return (sizes, inner(rect, left / 2));
        }

        for f in fill_ids {
            sizes[f] = left / fills * sizes[f];
            left -= sizes[f];
        }
        (sizes, rect)
    }

    /// Renders [`Layout`] base style
    fn render_base_style(&self, buffer: &mut Buffer) {
        for y in buffer.y()..buffer.y() + buffer.height() {
            for x in buffer.x()..buffer.x() + buffer.width() {
                buffer.set_style(self.style, &Vec2::new(x, y));
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

impl From<Layout> for Element {
    fn from(value: Layout) -> Self {
        Element::new(value)
    }
}
