use std::marker::PhantomData;

use crate::{
    buffer::Buffer,
    enums::{Color, RGB},
    geometry::{Constraint, Direction, Padding, Rect, Vec2},
    style::Style,
    widgets::cache::Cache,
};

use super::{widget::Widget, Element, Layout, Spacer};

/// A container widget that renders a gradient background behind its child
/// widget.
///
/// The [`BgGrad`] widget supports horizontal and vertical gradients. You can
/// set the gradient direction by providing [`Direction`] directly using
/// [`BgGrad::new`] method, or you can use methods like [`BgGrad::horizontal`]
/// and [`BgGrad::vertical`].
///
/// By default BgGrad is empty, it doesn't have a child. To set the child
/// widget, you can use [`BgGrad::child`] method.
///
/// # Examples
///
/// ```rust
/// # use termint::{term::Term, widgets::BgGrad};
/// # fn example() -> Result<(), &'static str> {
/// let grad = BgGrad::horizontal((0, 150, 255), (150, 255, 0));
///
/// let mut term = Term::new();
/// term.render(grad)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct BgGrad<W = Element> {
    bg_start: RGB,
    bg_end: RGB,
    direction: Direction,
    padding: Padding,
    child: Element,
    child_type: PhantomData<W>,
}

impl BgGrad<Spacer> {
    /// Creates a new empty [`BgGrad`] with the given gradient colors and
    /// direction.
    ///
    /// For `start` and `end` you can provide any type that can be converted
    /// into RGB, such as `u32`, `(u8 ,u8, u8)`.
    ///
    /// You can add child to be rendered on top of the gradient using
    /// [`BgGrad::child`] method.
    ///
    /// # Example
    /// ```rust
    /// # use termint::{widgets::BgGrad, geometry::Direction};
    /// let widget = BgGrad::new(
    ///     Direction::Vertical,
    ///     (0, 150, 255),
    ///     (150, 255, 0)
    /// );
    /// ```
    #[must_use]
    pub fn new<T1, T2>(dir: Direction, start: T1, end: T2) -> Self
    where
        T1: Into<RGB>,
        T2: Into<RGB>,
    {
        Self::construct(start.into(), end.into(), dir, Spacer::new())
    }

    /// Creates a new empty vertical [`BgGrad`] with the given gradient colors.
    ///
    /// For `start` and `end` you can provide any type that can be converted
    /// into RGB, such as `u32`, `(u8 ,u8, u8)`.
    ///
    /// You can add child to be rendered on top of the gradient using
    /// [`BgGrad::child`] method.
    ///
    /// # Example
    /// ```rust
    /// # use termint::widgets::BgGrad;
    /// let widget = BgGrad::vertical((0, 150, 255), (150, 255, 0));
    /// ```
    #[must_use]
    pub fn vertical<T1, T2>(start: T1, end: T2) -> Self
    where
        T1: Into<RGB>,
        T2: Into<RGB>,
    {
        Self::construct(
            start.into(),
            end.into(),
            Direction::Vertical,
            Spacer::new(),
        )
    }

    /// Creates a new empty horizontal [`BgGrad`] with the given gradient
    /// colors.
    ///
    /// For `start` and `end` you can provide any type that can be converted
    /// into RGB, such as `u32`, `(u8 ,u8, u8)`.
    ///
    /// You can add child to be rendered on top of the gradient using
    /// [`BgGrad::child`] method.
    ///
    /// # Example
    /// ```rust
    /// # use termint::widgets::BgGrad;
    /// let widget = BgGrad::horizontal((0, 150, 255), (150, 255, 0));
    /// ```
    #[must_use]
    pub fn horizontal<T1, T2>(start: T1, end: T2) -> Self
    where
        T1: Into<RGB>,
        T2: Into<RGB>,
    {
        Self::construct(
            start.into(),
            end.into(),
            Direction::Horizontal,
            Spacer::new(),
        )
    }
}

impl<W> BgGrad<W> {
    /// Sets the child widget to be displayed in front of the gradient.
    ///
    /// # Example
    /// ```rust
    /// # use termint::widgets::{BgGrad, Spacer};
    /// # type SomeWidget = Spacer;
    /// let widget = BgGrad::vertical((0, 150, 255), (150, 255, 0))
    ///     .child(SomeWidget::new());
    /// ```
    #[must_use]
    pub fn child<CW>(self, child: CW) -> BgGrad<CW>
    where
        CW: Into<Element>,
    {
        BgGrad {
            bg_start: self.bg_start,
            bg_end: self.bg_end,
            direction: self.direction,
            padding: self.padding,
            child: child.into(),
            child_type: PhantomData,
        }
    }
}

impl<W> BgGrad<W>
where
    W: Widget,
{
    /// Sets the gradient direction of the [`BgGrad`] background.
    ///
    /// The direction determines in which direction is the gradient drawn.
    #[must_use]
    pub fn bg_dir(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets padding around the child widget of the [`BgGrad`].
    ///
    /// You can provide any type that can be converted into [`Padding`], such
    /// as `usize`, `(usize, usize)`, or `(usize, usize, usize, usize)`.
    #[must_use]
    pub fn padding<T>(mut self, padding: T) -> Self
    where
        T: Into<Padding>,
    {
        self.padding = padding.into();
        self
    }

    fn construct(start: RGB, end: RGB, dir: Direction, child: W) -> Self {
        Self {
            bg_start: start,
            bg_end: end,
            direction: dir,
            padding: Default::default(),
            child: Element::new(child),
            child_type: PhantomData,
        }
    }
}

impl BgGrad<Layout> {
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
    ///   [`Constraint`])
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

impl<W> Widget for BgGrad<W>
where
    W: Widget,
{
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        if rect.is_empty() {
            return;
        }

        match self.direction {
            Direction::Vertical => self.ver_render(buffer, &rect),
            Direction::Horizontal => self.hor_render(buffer, &rect),
        };
        self.child.render(
            buffer,
            rect.inner(self.padding),
            &mut cache.children[0],
        );
    }

    fn height(&self, size: &Vec2) -> usize {
        let size = Vec2::new(
            size.x.saturating_sub(self.padding.get_horizontal()),
            size.y.saturating_sub(self.padding.get_vertical()),
        );
        self.child.height(&size) + self.padding.get_vertical()
    }

    fn width(&self, size: &Vec2) -> usize {
        let size = Vec2::new(
            size.x.saturating_sub(self.padding.get_horizontal()),
            size.y.saturating_sub(self.padding.get_vertical()),
        );
        self.child.width(&size) + self.padding.get_horizontal()
    }

    fn children(&self) -> Vec<&Element> {
        vec![&self.child]
    }
}

impl<W> BgGrad<W>
where
    W: Widget,
{
    /// Renders horizontal background gradient
    fn hor_render(&self, buffer: &mut Buffer, rect: &Rect) {
        let step = self.get_step(rect.width() as i16);
        let (mut r, mut g, mut b) =
            (self.bg_start.r, self.bg_start.g, self.bg_start.b);

        for x in rect.x()..rect.width() + rect.x() {
            let bg = Color::Rgb(r, g, b);
            (r, g, b) = self.add_step((r, g, b), step);

            for y in rect.y()..rect.height() + rect.y() {
                buffer.set_bg(bg, &Vec2::new(x, y));
            }
        }
    }

    /// Renders vertical background gradient
    fn ver_render(&self, buffer: &mut Buffer, rect: &Rect) {
        let step = self.get_step(rect.height() as i16);
        let (mut r, mut g, mut b) =
            (self.bg_start.r, self.bg_start.g, self.bg_start.b);

        for y in rect.y()..rect.height() + rect.y() {
            let bg = Color::Rgb(r, g, b);
            (r, g, b) = self.add_step((r, g, b), step);

            for x in rect.x()..rect.width() + rect.x() {
                buffer.set_bg(bg, &Vec2::new(x, y));
            }
        }
    }

    /// Gets step per character based on start and eng background color
    fn get_step(&self, len: i16) -> (i16, i16, i16) {
        (
            (self.bg_end.r as i16 - self.bg_start.r as i16) / len,
            (self.bg_end.g as i16 - self.bg_start.g as i16) / len,
            (self.bg_end.b as i16 - self.bg_start.b as i16) / len,
        )
    }

    /// Adds given step to RGB value in tuple
    fn add_step(
        &self,
        rgb: (u8, u8, u8),
        step: (i16, i16, i16),
    ) -> (u8, u8, u8) {
        (
            (rgb.0 as i16 + step.0) as u8,
            (rgb.1 as i16 + step.1) as u8,
            (rgb.2 as i16 + step.2) as u8,
        )
    }
}

// From implementations
impl<W> From<BgGrad<W>> for Box<dyn Widget>
where
    W: Widget + 'static,
{
    fn from(value: BgGrad<W>) -> Self {
        Box::new(value)
    }
}

impl<W> From<BgGrad<W>> for Element
where
    W: Widget + 'static,
{
    fn from(value: BgGrad<W>) -> Self {
        Element::new(value)
    }
}
