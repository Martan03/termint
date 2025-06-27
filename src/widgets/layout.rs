use std::cmp::{max, min};

use crate::{
    buffer::Buffer,
    enums::Color,
    geometry::{Constraint, Direction, Padding, Rect, Vec2},
    style::Style,
    widgets::cache::{Cache, LayoutCache},
};

use super::{widget::Widget, Element};

/// A container widget that arranges child widgets in a single direction
/// (horizontal or vertical), flexing their sizes based on given constraints.
///
/// # Direction
///
/// The layout can be horizontal (left-to-right) or vertical (top-to-bottom).
///
/// # Constraints
///
/// Each child widget's size is controlled by a [`Constraint`] determining how
/// space is allocated. You can learn more about [`Constraint`] in its
/// documentation.
///
/// # Example
/// ```rust
/// # use termint::{
/// #     term::Term,
/// #     geometry::{Constraint, Rect},
/// #     widgets::{Block, Layout, ToSpan, Widget},
/// # };
/// # fn example() -> Result<(), &'static str> {
/// // Creates new horizontal layout containing two blocks each covering 50%
/// let block1 = Block::vertical().title("Block 1");
/// let block2 = Block::vertical().title("Block 2");
///
/// let mut layout = Layout::horizontal();
/// layout.push(block1, Constraint::Percent(50));
/// layout.push(block2, Constraint::Percent(50));
///
/// let mut term = Term::new();
/// term.render(layout)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Layout {
    direction: Direction,
    children: Vec<Element>,
    constraints: Vec<Constraint>,
    style: Style,
    padding: Padding,
    center: bool,
}

impl Layout {
    /// Creates new [`Layout`] that flexes in given [`Direction`].
    #[must_use]
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            ..Default::default()
        }
    }

    /// Creates a [`Layout`] that flexes vertically.
    #[must_use]
    pub fn vertical() -> Self {
        Default::default()
    }

    /// Creates a [`Layout`] that flexes horizontally.
    #[must_use]
    pub fn horizontal() -> Self {
        Self {
            direction: Direction::Horizontal,
            ..Default::default()
        }
    }

    /// Sets flexing [`Direction`] of the [`Layout`].
    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the base style of the [`Layout`].
    #[must_use]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets base background color of the [`Layout`].
    #[must_use]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.bg(bg);
        self
    }

    /// Sets base foreground color of the [`Layout`].
    #[must_use]
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.fg(fg);
        self
    }

    /// Sets the [`Padding`] of the [`Layout`].
    #[must_use]
    pub fn padding<T>(mut self, padding: T) -> Self
    where
        T: Into<Padding>,
    {
        self.padding = padding.into();
        self
    }

    /// Makes [`Layout`] center its content in the direction it flexes.
    ///
    /// If the layout is flexing its children horizontally, the content will
    /// be centered horizontally. Otherwise it will be centered vertically.
    #[must_use]
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
        T: Into<Element>,
        C: Into<Constraint>,
    {
        self.children.push(child.into());
        self.constraints.push(constraint.into());
    }

    /// Adds a child widget with its contraint
    ///
    /// # Parameters
    /// - `child`: The widget to add (any type convertible to [`Element`])
    /// - `contraint`: Widget's contraint (any type convertible to
    ///     [`Constraint`])
    pub fn push<T, C>(&mut self, child: T, constraint: C)
    where
        T: Into<Element>,
        C: Into<Constraint>,
    {
        self.children.push(child.into());
        self.constraints.push(constraint.into());
    }
}

impl Widget for Layout {
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        self.render_base_style(buffer, &rect);

        let rect = rect.inner(self.padding);
        if rect.is_empty() || self.children.is_empty() {
            return;
        }

        match self.direction {
            Direction::Vertical => self.ver_render(buffer, rect, cache),
            Direction::Horizontal => self.hor_render(buffer, rect, cache),
        }
    }

    fn height(&self, size: &Vec2) -> usize {
        let size = Vec2::new(
            size.x.saturating_sub(self.padding.get_horizontal()),
            size.y.saturating_sub(self.padding.get_vertical()),
        );
        let height = match self.direction {
            Direction::Vertical => {
                self.size_sd(&size, size.y, |c, s| c.height(s))
            }
            Direction::Horizontal => self.hor_height(&size),
        };
        height + self.padding.get_vertical()
    }

    fn width(&self, size: &Vec2) -> usize {
        let size = Vec2::new(
            size.x.saturating_sub(self.padding.get_horizontal()),
            size.y.saturating_sub(self.padding.get_vertical()),
        );
        let width = match self.direction {
            Direction::Vertical => self.ver_width(&size),
            Direction::Horizontal => {
                self.size_sd(&size, size.x, |c, s| c.width(s))
            }
        };
        width + self.padding.get_horizontal()
    }

    fn children(&self) -> Vec<&Element> {
        self.children.iter().collect()
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            children: Vec::new(),
            constraints: Vec::new(),
            style: Style::new(),
            padding: Default::default(),
            center: false,
        }
    }
}

impl Layout {
    /// Renders layout
    fn ver_render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        let (sizes, mut rect) = match self.get_cache(&rect, cache) {
            Some(sizes) => {
                let rect =
                    self.content_rect(rect, &sizes, |r, v| r.inner((v, 0)));
                (sizes, rect)
            }
            None => {
                let (sizes, crect) = self.ver_sizes(rect.clone());
                self.create_cache(rect, cache, &sizes);
                (sizes, crect)
            }
        };

        for (i, s) in sizes.iter().enumerate() {
            let csize = min(*s, rect.height());
            let crect =
                Rect::from_coords(*rect.pos(), Vec2::new(rect.width(), csize));
            self.children[i].render(buffer, crect, &mut cache.children[i]);
            rect = rect.inner(Padding::top(csize));
        }
    }

    /// Renders layout
    fn hor_render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        let (sizes, mut rect) = match self.get_cache(&rect, cache) {
            Some(sizes) => {
                let rect =
                    self.content_rect(rect, &sizes, |r, v| r.inner((0, v)));
                (sizes, rect)
            }
            None => {
                let (sizes, crect) = self.hor_sizes(rect.clone());
                self.create_cache(rect, cache, &sizes);
                (sizes, crect)
            }
        };

        for (i, s) in sizes.iter().enumerate() {
            let csize = min(*s, rect.width());
            let crect = Rect::from_coords(
                *rect.pos(),
                Vec2::new(csize, rect.height()),
            );
            self.children[i].render(buffer, crect, &mut cache.children[i]);
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
            |r, s| r.inner(Padding::vertical(s)),
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
            |r, s| r.inner(Padding::horizontal(s)),
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
        F1: Fn(&Element, &Vec2) -> usize,
        F2: Fn(&mut Vec2, usize),
        F3: Fn(Vec2) -> usize,
        F4: Fn(Rect, usize) -> Rect,
    {
        let mut fill_ids = Vec::new();
        let mut fills = 0;
        let mut sizes = Vec::new();
        let mut size = *rect.size();

        for (i, constraint) in self.constraints.iter().enumerate() {
            let csize = match constraint {
                Constraint::Length(len) => *len,
                Constraint::Percent(p) => percent * p / 100,
                Constraint::Min(l) => max(csize(&self.children[i], &size), *l),
                Constraint::Max(h) => min(csize(&self.children[i], &size), *h),
                Constraint::MinMax(l, h) => {
                    min(max(csize(&self.children[i], &size), *l), *h)
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
            let fill = sizes[f];
            sizes[f] = left / fills * fill;
            fills -= fill;
            left -= sizes[f];
        }
        (sizes, rect)
    }

    /// Renders [`Layout`] base style
    fn render_base_style(&self, buffer: &mut Buffer, rect: &Rect) {
        for pos in rect.into_iter() {
            buffer.set_style(self.style, &pos);
            if self.style.bg.is_some() {
                buffer.set_val(' ', &pos);
            }
        }
    }

    fn size_sd<F>(&self, size: &Vec2, prim: usize, csize: F) -> usize
    where
        F: Fn(&Element, &Vec2) -> usize,
    {
        let mut total = 0;
        let mut fill = false;
        for (i, constraint) in self.constraints.iter().enumerate() {
            match constraint {
                Constraint::Length(len) => total += len,
                Constraint::Percent(p) => total += prim * p / 100,
                Constraint::Min(l) => {
                    total += max(*l, csize(&self.children[i], size))
                }
                Constraint::Max(h) => {
                    total += min(*h, csize(&self.children[i], size))
                }
                Constraint::MinMax(l, h) => {
                    total += min(*h, max(*l, csize(&self.children[i], size)))
                }
                Constraint::Fill(_) => fill = true,
            }
        }
        if fill {
            return max(prim, total);
        }
        total
    }

    fn ver_width(&self, size: &Vec2) -> usize {
        let mut width = 0;
        let mut total = 0;
        let mut total_fills = 0;
        let mut fills = Vec::new();
        for (i, constraint) in self.constraints.iter().enumerate() {
            let csize = match constraint {
                Constraint::Length(len) => *len,
                Constraint::Percent(p) => size.y * p / 100,
                Constraint::Min(l) => max(*l, self.children[i].height(size)),
                Constraint::Max(h) => min(*h, self.children[i].height(size)),
                Constraint::MinMax(l, h) => {
                    min(*h, max(*l, self.children[i].height(size)))
                }
                Constraint::Fill(f) => {
                    total_fills += f;
                    fills.push((&self.children[i], f));
                    continue;
                }
            };
            total += csize;
            width =
                width.max(self.children[i].width(&Vec2::new(size.x, csize)));
        }

        let mut left = Vec2::new(size.x, size.y.saturating_sub(total));
        for (child, f) in fills {
            let h = left.y / total_fills * f;
            width = width.max(child.width(&left));
            left.y -= h;
            total_fills -= f;
        }
        width
    }

    fn hor_height(&self, size: &Vec2) -> usize {
        let mut height = 0;
        let mut total = 0;
        let mut total_fills = 0;
        let mut fills = Vec::new();
        for (i, constraint) in self.constraints.iter().enumerate() {
            let csize = match constraint {
                Constraint::Length(len) => *len,
                Constraint::Percent(p) => size.y * p / 100,
                Constraint::Min(l) => max(*l, self.children[i].width(size)),
                Constraint::Max(h) => min(*h, self.children[i].width(size)),
                Constraint::MinMax(l, h) => {
                    min(*h, max(*l, self.children[i].width(size)))
                }
                Constraint::Fill(f) => {
                    total_fills += f;
                    fills.push((&self.children[i], f));
                    continue;
                }
            };
            total += csize;
            height =
                height.max(self.children[i].height(&Vec2::new(csize, size.y)));
        }

        let mut left = Vec2::new(size.x, size.y.saturating_sub(total));
        for (child, f) in fills {
            let h = left.y / total_fills * f;
            height = height.max(child.width(&left));
            left.y -= h;
            total_fills -= f;
        }
        height
    }

    fn get_cache<'a>(
        &self,
        rect: &Rect,
        cache: &'a mut Cache,
    ) -> Option<Vec<usize>> {
        let lcache = cache.local::<LayoutCache>()?;
        if !lcache.same_key(rect.size(), &self.direction, &self.constraints) {
            return None;
        }
        Some(lcache.sizes.clone())
    }

    fn create_cache<'a>(
        &self,
        rect: Rect,
        cache: &'a mut Cache,
        sizes: &Vec<usize>,
    ) {
        let lcache = LayoutCache::new(
            *rect.size(),
            self.direction,
            self.constraints.clone(),
        )
        .sizes(sizes.clone());
        cache.local = Some(Box::new(lcache));
    }

    fn content_rect<F>(&self, rect: Rect, sizes: &Vec<usize>, inner: F) -> Rect
    where
        F: Fn(Rect, usize) -> Rect,
    {
        if !self.center {
            return rect;
        }
        let total: usize = sizes.iter().sum();
        inner(rect, total / 2)
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
