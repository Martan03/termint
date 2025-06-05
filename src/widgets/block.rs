use std::{cmp::max, marker::PhantomData};

use crate::{
    borders,
    buffer::Buffer,
    enums::{Border, BorderType, Color},
    geometry::{Constraint, Direction, Padding, Rect, Vec2},
    style::Style,
    text::Text,
    widgets::{cache::Cache, span::Span},
};

use super::{widget::Widget, Element, Layout, Spacer};

/// A widget that wraps another widget and adds border and title.
///
/// [`Block`] is typically used to visualize separation and organize sections.
/// You can customize the border style, type of the border and so on.
///
/// # Example
/// ```rust
/// # use termint::{
/// #     term::Term,
/// #     enums::{Color, BorderType},
/// #     geometry::{Constraint, Rect},
/// #     widgets::{Block, ToSpan, Widget},
/// # };
/// # fn example() -> Result<(), &'static str> {
/// let mut main = Block::horizontal()
///     .title("Termint".fg(Color::Red))
///     .border_type(BorderType::Double)
///     .border_color(Color::LightGray);
///
/// let block1 = Block::vertical().title("Sub block");
/// main.add_child(block1, Constraint::Percent(50));
/// let block2 = Block::vertical().title("Another");
/// main.add_child(block2, Constraint::Percent(50));
///
/// let mut term = Term::new();
/// term.render(main)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Block<W = Element> {
    title: Box<dyn Text>,
    borders: Border,
    border_type: BorderType,
    border_style: Style,
    child: Element,
    child_type: PhantomData<W>,
}

impl<W> Block<W>
where
    W: Widget,
{
    /// Returns a new [`Block`] wrapping the given widget, with all borders
    /// enabled and no title.
    ///
    /// You can provide any type implementing the [`Widget`] trait.
    #[must_use]
    pub fn new<T>(child: T) -> Self
    where
        T: Into<Element>,
    {
        Self {
            title: Box::new(Span::new("")),
            borders: Border::ALL,
            border_type: BorderType::Normal,
            border_style: Default::default(),
            child: child.into(),
            child_type: PhantomData,
        }
    }

    /// Sets the [`Text`] title displayed at the top of the [`Block`].
    ///
    /// This is typically used for section labels in your TUI.
    #[must_use]
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Box<dyn Text>>,
    {
        self.title = title.into();
        self
    }

    /// Sets the visible borders of the [`Block`] using the given [`Border`]
    /// flags.
    ///
    /// # Example
    /// ```rust
    /// # use termint::{enums::Border, widgets::Block, borders};
    /// let block1 = Block::horizontal().borders(Border::TOP | Border::BOTTOM);
    /// let block2 = Block::horizontal().borders(borders!(TOP, BOTTOM));
    /// ```
    #[must_use]
    pub fn borders(mut self, borders: Border) -> Self {
        self.borders = borders;
        self
    }

    /// Sets the [`BorderType`] used to render the [`Block`] border.
    #[must_use]
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Sets the style applied to [`Block`] borders.
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

impl Block<Spacer> {
    /// Returns an empty [`Block`] with no title and all borders
    #[must_use]
    pub fn empty() -> Self {
        Self {
            title: Box::new(Span::new("")),
            borders: Border::ALL,
            border_type: BorderType::Normal,
            border_style: Default::default(),
            child: Spacer::new().into(),
            child_type: PhantomData,
        }
    }
}

impl Block<Layout> {
    /// Returns a [`Block`] containing a vertical [`Layout`] as its child.
    ///
    /// Very often you want to have a layout inside of a [`Block`] widget. This
    /// constructor makes it easier.
    ///
    /// # Example
    /// ```rust
    /// # use termint::widgets::{Block, Layout};
    /// // Creates block with vertical layout as its child
    /// let block1 = Block::new(Layout::vertical());
    /// // Does the same, but shorter
    /// let block2 = Block::vertical();
    /// ```
    #[must_use]
    pub fn vertical() -> Self {
        Self {
            title: Box::new(Span::new("")),
            borders: Border::ALL,
            border_type: Default::default(),
            border_style: Default::default(),
            child: Layout::vertical().into(),
            child_type: PhantomData,
        }
    }

    /// Returns a [`Block`] containing a horizontal [`Layout`] as its child.
    ///
    /// Very often you want to have a layout inside of a [`Block`] widget. This
    /// constructor makes it easier.
    ///
    /// # Example
    /// ```rust
    /// # use termint::widgets::{Block, Layout};
    /// // Creates block with horizontal layout as its child
    /// let block1 = Block::new(Layout::horizontal());
    /// // Does the same, but shorter
    /// let block2 = Block::horizontal();
    /// ```
    #[must_use]
    pub fn horizontal() -> Self {
        Self {
            title: Box::new(Span::new("")),
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
        self.child = self.child.map::<Layout, _>(|l| l.direction(direction));
        self
    }

    /// Sets the base style of the [`Layout`].
    #[must_use]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.child = self.child.map::<Layout, _>(|l| l.style(style));
        self
    }

    /// Sets base background color of the [`Layout`].
    #[must_use]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.child = self.child.map::<Layout, _>(|l| l.bg(bg));
        self
    }

    /// Sets base foreground color of the [`Layout`].
    #[must_use]
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.child = self.child.map::<Layout, _>(|l| l.fg(fg));
        self
    }

    /// Sets the [`Padding`] of the [`Layout`].
    #[must_use]
    pub fn padding<T>(mut self, padding: T) -> Self
    where
        T: Into<Padding>,
    {
        self.child = self.child.map::<Layout, _>(|l| l.padding(padding));
        self
    }

    /// Makes [`Layout`] center its content in the direction it flexes.
    ///
    /// If the layout is flexing its children horizontally, the content will
    /// be centered horizontally. Otherwise it will be centered vertically.
    #[must_use]
    pub fn center(mut self) -> Self {
        self.child = self.child.map::<Layout, _>(|l| l.center());
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
        if let Some(layout) = self.child.downcast_mut::<Layout>() {
            layout.push(child, constraint);
        }
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
        if let Some(layout) = self.child.downcast_mut::<Layout>() {
            layout.push(child, constraint);
        }
    }
}

impl<W> Widget for Block<W>
where
    W: Widget,
{
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        let (t, r, b, l) = self.render_border(buffer, &rect);
        let mut pos = Vec2::new(rect.x() + l, rect.y());
        let mut size = Vec2::new(rect.width().saturating_sub(l + r), 1);

        let trect = Rect::from_coords(pos, size);
        _ = self.title.render_offset(buffer, trect, 0, None);

        pos.y += t;
        size.y = rect.height().saturating_sub(t + b);
        let crect = Rect::from_coords(pos, size);
        if !rect.contains(&crect) {
            return;
        }
        self.child.render(buffer, crect, &mut cache.children[0]);
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
        max(self.child.width(&size), self.title.get_text().len()) + width
    }

    fn children(&self) -> Vec<&Element> {
        vec![&self.child]
    }
}

impl<W> Block<W>
where
    W: Widget,
{
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
            buffer[pos] = buffer[pos].val(c).style(self.border_style);
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
            buffer[pos] = buffer[pos].val(c).style(self.border_style);
            pos.y += 1;
        }
        1
    }

    /// Adds corner of [`Block`] border to the string
    fn render_corner(&self, buffer: &mut Buffer, pos: Vec2, border: Border) {
        if (self.borders & border) == border {
            let c = self.border_type.get(border);
            buffer[pos] = buffer[pos].val(c).style(self.border_style);
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
impl<W> From<Block<W>> for Box<dyn Widget>
where
    W: Widget + 'static,
{
    fn from(value: Block<W>) -> Self {
        Box::new(value)
    }
}

impl<W> From<Block<W>> for Element
where
    W: Widget + 'static,
{
    fn from(value: Block<W>) -> Self {
        Element::new(value)
    }
}
