use std::{
    cmp::max,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
};

use crate::{
    borders,
    buffer::Buffer,
    enums::{Border, BorderType, Color},
    geometry::Padding,
    prelude::{Constraint, Direction, Rect, TextAlign, Vec2},
    style::{Style, Stylize},
    text::Text,
    widgets::{Element, Layout, LayoutNode, Spacer, Widget},
};

/// A widget that wraps another widget and adds border and title.
///
/// [`Block`] is typically used to visualize separation and organize sections.
/// You can customize the border style, type of the border and so on.
///
/// # Example
///
/// ```rust
/// use termint::prelude::*;
///
/// // Creates new block containing horizontal layout
/// let mut main: Block<(), _> = Block::horizontal()
///     // Add any `Text` widget as a title
///     .title("Termint".fg(Color::Red))
///     // Double line border
///     .border_type(BorderType::Double)
///     // Makes the border color light gray
///     .border_color(Color::LightGray);
///
/// // Block exposes `Layout` methods for simplification
/// main.push("Sidebar", Constraint::Percent(30));
/// main.push("Content", Constraint::Fill(1))
/// ```
#[derive(Debug)]
pub struct Block<M: 'static = (), W = Element<M>> {
    titles: Vec<Box<dyn Text>>,
    bot_titles: Vec<Box<dyn Text>>,
    borders: Border,
    border_type: BorderType,
    border_style: Style,
    child: Element<M>,
    child_type: PhantomData<W>,
}

impl<M> Block<M, Element<M>> {
    /// Creates a new [`Block`] wrapping the given widget.
    ///
    /// By default all the borders are visible and no title is set.
    ///
    /// The `child` can be any type convertible to [`Element`].
    #[must_use]
    pub fn new<T>(child: T) -> Self
    where
        T: Into<Element<M>>,
    {
        Self {
            titles: vec![],
            bot_titles: vec![],
            borders: Border::ALL,
            border_type: BorderType::Normal,
            border_style: Default::default(),
            child: child.into(),
            child_type: PhantomData,
        }
    }
}

impl<M, W> Block<M, W> {
    /// Adds the title to the top of the [`Block`].
    ///
    /// This is typically used for section labels in your TUI.
    ///
    /// The `title` can be any type implementing [`Text`] trait.
    ///
    /// # Example
    ///
    /// ```
    /// use termint::prelude::*;
    ///
    /// Block::empty()
    ///     .title("Left1")
    ///     .title(Grad::new("Left2", (0, 255, 0), (255, 0, 0)))
    ///     .title("Center".align(TextAlign::Center))
    ///     .title("Right".red().align(TextAlign::Right));
    ///
    /// // Renders (without colors)
    /// // ┌Left1─Left2────Center──────────Right┐
    /// // └────────────────────────────────────┘
    /// ```
    #[must_use]
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Box<dyn Text>>,
    {
        self.titles.push(title.into());
        self
    }

    /// Adds the title to the bottom of the [`Block`].
    ///
    /// The `title` can be any type implementing [`Text`] trait.
    ///
    /// # Example
    ///
    /// ```
    /// use termint::prelude::*;
    ///
    /// Block::empty()
    ///     .title_bottom("Left1")
    ///     .title_bottom(Grad::new("Left2", (0, 255, 0), (255, 0, 0)))
    ///     .title_bottom("Center".align(TextAlign::Center))
    ///     .title_bottom("Right".red().align(TextAlign::Right));
    ///
    /// // Renders (without colors)
    /// // ┌────────────────────────────────────┐
    /// // └Left1─Left2────Center──────────Right┘
    /// ```
    pub fn title_bottom<T>(mut self, title: T) -> Self
    where
        T: Into<Box<dyn Text>>,
    {
        self.bot_titles.push(title.into());
        self
    }

    /// Sets the visible borders of the [`Block`] using the given [`Border`]
    /// flags.
    ///
    /// # Example
    ///
    /// ```rust
    /// use termint::{prelude::*, borders};
    ///
    /// // Creates new [`Block`] with only top and bottom borders
    /// let block1 = Block::<(), _>::empty()
    ///     .borders(Border::TOP | Border::BOTTOM);
    /// // Or shorter using `borders!` macro
    /// let block2 = Block::<(), _>::horizontal()
    ///     .borders(borders!(TOP, BOTTOM));
    /// ```
    #[must_use]
    pub fn borders(mut self, borders: Border) -> Self {
        self.borders = borders;
        self
    }

    /// Sets the [`BorderType`] used to render the [`Block`] border.
    ///
    /// You can look at the [`BorderType`]'s documentation to look at all the
    /// supported border types.
    #[must_use]
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Sets the style applied to [`Block`] borders.
    ///
    /// The `style` can be any type convertible to [`Style`].
    #[must_use]
    pub fn border_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.border_style = style.into();
        self
    }

    /// Sets the foreground color of the [`Block`] borders.
    #[must_use]
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_style = self.border_style.fg(color);
        self
    }
}

impl<M: Clone + 'static> Block<M, Spacer> {
    /// Creates a new [`Block`] containing a [`Spacer`].
    ///
    /// By default all the borders are visible and no title is set. This is
    /// useful for creating empty bordered areas.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            titles: vec![],
            bot_titles: vec![],
            borders: Border::ALL,
            border_type: BorderType::Normal,
            border_style: Default::default(),
            child: Spacer::new().into(),
            child_type: PhantomData,
        }
    }
}

impl<M: Clone + 'static> Block<M, Layout<M>> {
    /// Creates a new [`Block`] wrapping a vertical [`Layout`].
    ///
    /// This is convenience constructor equivalent to
    /// `Block::new(Layout::vertical())`.
    #[must_use]
    pub fn vertical() -> Self {
        Self {
            titles: vec![],
            bot_titles: vec![],
            borders: Border::ALL,
            border_type: Default::default(),
            border_style: Default::default(),
            child: Layout::vertical().into(),
            child_type: PhantomData,
        }
    }

    /// Creates a new [`Block`] wrapping a horizontal [`Layout`].
    ///
    /// This is convenience constructor equivalent to
    /// `Block::new(Layout::horizontal())`.
    #[must_use]
    pub fn horizontal() -> Self {
        Self {
            titles: vec![],
            bot_titles: vec![],
            borders: Border::ALL,
            border_type: Default::default(),
            border_style: Default::default(),
            child: Layout::horizontal().into(),
            child_type: PhantomData,
        }
    }

    /// Sets flexing [`Direction`] of the [`Layout`].
    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.child =
            self.child.map::<Layout<M>, _>(|l| l.direction(direction));
        self
    }

    /// Sets the base style of the [`Layout`].
    ///
    /// The `style` can be any type convertible to [`Style`].
    #[must_use]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.child = self.child.map::<Layout<M>, _>(|l| l.style(style));
        self
    }

    /// Sets base background color of the [`Layout`].
    ///
    /// The `bg` can be any type convertible into `Option<Color>`. If `None` is
    /// supplied, the background is transparent.
    #[must_use]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.child = self.child.map::<Layout<M>, _>(|l| l.bg(bg));
        self
    }

    /// Sets base foreground color of the [`Layout`].
    ///
    /// The `fg` can be any type convertible into `Option<Color>`. If `None` is
    /// supplied, it keeps the original foreground color.
    #[must_use]
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.child = self.child.map::<Layout<M>, _>(|l| l.fg(fg));
        self
    }

    /// Sets the [`Padding`] of the [`Layout`].
    ///
    /// The `padding` can be any type convertible into [`Padding`], such as
    /// `usize` (uniform), `(usize, usize)` (vertical, horizontal). You can
    /// read more in the [`Padding`] documentation.
    #[must_use]
    pub fn padding<T>(mut self, padding: T) -> Self
    where
        T: Into<Padding>,
    {
        self.child = self.child.map::<Layout<M>, _>(|l| l.padding(padding));
        self
    }

    /// Makes [`Layout`] center its content in the direction it flexes.
    ///
    /// If the layout is flexing its children horizontally, the content will
    /// be centered horizontally. Otherwise it will be centered vertically.
    #[must_use]
    pub fn center(mut self) -> Self {
        self.child = self.child.map::<Layout<M>, _>(|l| l.center());
        self
    }

    /// Adds a child widget with its contraint
    ///
    /// The `child` can be any type convertible into [`Element`].
    ///
    /// The `constraint` can be any type convertible into [`Constraint`], such
    /// as `5` (`Constraint::Length(5)`) and `1..` (`Constraint::Min(1)`).
    pub fn push<T, C>(&mut self, child: T, constraint: C)
    where
        T: Into<Element<M>>,
        C: Into<Constraint>,
    {
        if let Some(layout) = self.child.downcast_mut::<Layout<M>>() {
            layout.push(child, constraint);
        }
    }
}

impl<M, W> Widget<M> for Block<M, W>
where
    M: Clone + 'static,
    W: Widget<M>,
{
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        let rect = layout.area;
        let (_, r, _, l) = self.render_border(buffer, &rect);
        let pos = Vec2::new(rect.x() + l, rect.y());
        let size = Vec2::new(rect.width().saturating_sub(l + r), 1);

        let mut trect = Rect::from_coords(pos, size);
        self.render_title(buffer, trect, &self.titles);

        trect.pos.y = rect.bottom();
        self.render_title(buffer, trect, &self.bot_titles);

        self.child.render(buffer, &layout.children[0]);
    }

    fn height(&self, size: &Vec2) -> usize {
        let (width, height) = self.border_size();
        let size = Vec2::new(
            size.x.saturating_sub(width),
            size.y.saturating_sub(height),
        );
        height + self.child.height(&size)
    }

    fn width(&self, size: &Vec2) -> usize {
        let (width, height) = self.border_size();
        let size = Vec2::new(
            size.x.saturating_sub(width),
            size.y.saturating_sub(height),
        );

        let size = Vec2::new(size.x, 1);
        let mut twidth: usize =
            self.titles.iter().map(|t| t.width(&size)).sum();

        let bwidth = self.bot_titles.iter().map(|t| t.width(&size)).sum();
        twidth = twidth.max(bwidth) + self.titles.len().saturating_sub(1);

        max(max(self.child.width(&size), twidth) + width, size.x)
    }

    fn children(&self) -> Vec<&Element<M>> {
        vec![&self.child]
    }

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.titles
            .iter()
            .for_each(|t| t.layout_hash().hash(&mut hasher));
        self.borders.hash(&mut hasher);

        hasher.finish()
    }

    fn layout(&self, node: &mut LayoutNode, area: Rect) {
        node.children[0].layout(&self.child, area.inner(self.borders));
    }
}

impl<M, W> Block<M, W> {
    /// Renders [`Block`] border
    fn render_border(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
    ) -> (usize, usize, usize, usize) {
        let l = self.ver_border(buffer, rect, rect.left(), Border::LEFT);
        let r = self.ver_border(buffer, rect, rect.right(), Border::RIGHT);
        let t = self.hor_border(buffer, rect, rect.top(), Border::TOP);
        let b = self.hor_border(buffer, rect, rect.bottom(), Border::BOTTOM);

        if rect.width() <= 1 || rect.height() <= 1 {
            return (t, r, b, l);
        }

        self.render_corner(buffer, *rect.pos(), borders!(TOP, LEFT));
        self.render_corner(buffer, rect.top_right(), borders!(TOP, RIGHT));
        self.render_corner(buffer, rect.bottom_left(), borders!(BOTTOM, LEFT));
        self.render_corner(
            buffer,
            rect.bottom_right(),
            borders!(BOTTOM, RIGHT),
        );

        (t, r, b, l)
    }

    fn render_title(&self, buf: &mut Buffer, area: Rect, t: &[Box<dyn Text>]) {
        if t.is_empty() || area.is_empty() {
            return;
        }

        let mut lrect = area.clone();
        let (mut clines, mut cwidth) = (vec![], 0);
        let (mut rlines, mut rwidth) = (vec![], 0);
        for title in t {
            let mut lines = vec![];
            title.append_lines(&mut lines, &Vec2::new(usize::MAX, 1), None);

            let Some(line) = lines.into_iter().next() else {
                continue;
            };

            let (l, w) = match title.get_align() {
                TextAlign::Left => {
                    line.render(buf, lrect, TextAlign::Left);
                    lrect = lrect.inner(Padding::left(line.width + 1));
                    continue;
                }
                TextAlign::Center => (&mut clines, &mut cwidth),
                TextAlign::Right => (&mut rlines, &mut rwidth),
            };
            *w += line.width;
            l.push(line);
        }

        cwidth += clines.len().saturating_sub(1);
        let coffset = area.width().saturating_sub(cwidth) / 2;
        let mut crect = area.inner(Padding::left(coffset));
        for line in clines {
            line.render(buf, crect, TextAlign::Left);
            crect = crect.inner(Padding::left(line.width + 1));
        }

        rwidth += rlines.len().saturating_sub(1);
        let roffset = area.width().saturating_sub(rwidth);
        let mut rrect = area.inner(Padding::left(roffset));
        for line in rlines {
            line.render(buf, rrect, TextAlign::Left);
            rrect = rrect.inner(Padding::left(line.width + 1));
        }
    }

    /// Adds horizontal border to the buffer
    fn hor_border(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        y: usize,
        border: Border,
    ) -> usize {
        if (self.borders & border) == Border::NONE {
            return 0;
        }

        let c = self.border_type.get(border);
        let mut pos = Vec2::new(rect.x(), y);
        while pos.x <= rect.right() {
            buffer[pos].char(c).style(self.border_style);
            pos.x += 1;
        }
        1
    }

    /// Adds vertical border to the buffer
    fn ver_border(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        x: usize,
        border: Border,
    ) -> usize {
        if (self.borders & border) == Border::NONE {
            return 0;
        }

        let c = self.border_type.get(border);
        let mut pos = Vec2::new(x, rect.y());
        while pos.y <= rect.bottom() {
            buffer[pos].char(c).style(self.border_style);
            pos.y += 1;
        }
        1
    }

    /// Adds corner of [`Block`] border to the string
    fn render_corner(&self, buffer: &mut Buffer, pos: Vec2, border: Border) {
        if (self.borders & border) == border {
            let c = self.border_type.get(border);
            buffer[pos].char(c).style(self.border_style);
        }
    }

    /// Gets border size
    fn border_size(&self) -> (usize, usize) {
        (self.hor_border_size(), self.ver_border_size())
    }

    /// Gets horizontal border size
    fn hor_border_size(&self) -> usize {
        (self.borders & Border::RIGHT != Border::NONE) as usize
            + (self.borders & Border::LEFT != Border::NONE) as usize
    }

    /// Gets vertical border size and acounting title as well
    fn ver_border_size(&self) -> usize {
        (self.borders & Border::TOP != Border::NONE) as usize
            + (self.borders & Border::BOTTOM != Border::NONE) as usize
    }
}

// From implementations
impl<M, W> From<Block<M, W>> for Box<dyn Widget<M>>
where
    M: Clone + 'static,
    W: Widget<M> + 'static,
{
    fn from(value: Block<M, W>) -> Self {
        Box::new(value)
    }
}

impl<M, W> From<Block<M, W>> for Element<M>
where
    M: Clone + 'static,
    W: Widget<M> + 'static,
{
    fn from(value: Block<M, W>) -> Self {
        Element::new(value)
    }
}
