use crate::{
    buffer::Buffer,
    enums::{Color, RGB},
    geometry::{Constraint, Direction, Padding, Rect, Vec2},
    style::Style,
};

use super::{widget::Widget, Element, Layout};

/// Widget that renders gradient background and contains a children
///
/// ## Example usage using [`Term`] (automatically renders on full screen):
/// ```rust
/// # use termint::{term::Term, widgets::{BgGrad, Spacer}};
/// # fn get_child() -> Spacer { Spacer::new() }
/// # fn example() -> Result<(), &'static str> {
/// // Creates new background gradient with horizontal direction
/// let grad = BgGrad::horizontal(get_child(), (0, 150, 255), (150, 255, 0));
///
/// // Renders background gradient using Term struct
/// let mut term = Term::new();
/// term.render(grad)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Example usage without using [`Term`]:
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     geometry::Rect,
/// #     widgets::{BgGrad, Widget, Spacer},
/// # };
/// # fn get_child() -> Spacer { Spacer::new() }
/// // Creates new background gradient with horizontal direction
/// let grad = BgGrad::horizontal(get_child(), (0, 150, 255), (150, 255, 0));
///
/// // Renders background gradient using [`Buffer`]
/// // (position and size given by the Rect supplied to the Buffer)
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 20, 9));
/// grad.render(&mut buffer);
/// buffer.render();
/// ```
#[derive(Debug)]
pub struct BgGrad<W = Element> {
    bg_start: RGB,
    bg_end: RGB,
    direction: Direction,
    padding: Padding,
    child: W,
}

impl<W> BgGrad<W>
where
    W: Widget,
{
    /// Creates new vertical [`BgGrad`] with given gradient colors
    pub fn vertical<T1, T2>(child: W, start: T1, end: T2) -> Self
    where
        T1: Into<RGB>,
        T2: Into<RGB>,
    {
        Self {
            bg_start: start.into(),
            bg_end: end.into(),
            direction: Direction::Vertical,
            padding: Default::default(),
            child,
        }
    }

    /// Creates new horizontal [`BgGrad`] with given gradient colors
    pub fn horizontal<T1, T2>(child: W, start: T1, end: T2) -> Self
    where
        T1: Into<RGB>,
        T2: Into<RGB>,
    {
        Self {
            bg_start: start.into(),
            bg_end: end.into(),
            direction: Direction::Horizontal,
            padding: Default::default(),
            child,
        }
    }

    /// Sets gradient [`Direction`] of the [`BgGrad`]
    pub fn bg_dir(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets padding of the [`BgGrad`]
    pub fn padding<T>(mut self, padding: T) -> Self
    where
        T: Into<Padding>,
    {
        self.padding = padding.into();
        self
    }
}

impl BgGrad<Layout> {
    /// Sets [`Direction`] of the [`Layout`]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.child = self.child.direction(direction);
        self
    }

    /// Sets the base style of the [`Layout`]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.child = self.child.style(style);
        self
    }

    /// Sets base background color of the [`Layout`]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.child = self.child.bg(bg);
        self
    }

    /// Sets base foreground color of the [`Layout`]
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.child = self.child.fg(fg);
        self
    }

    /// Makes [`Layout`] center its content in its direction
    pub fn center(mut self) -> Self {
        self.child = self.child.center();
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
        self.child.push(child, constraint);
    }

    /// Pushes child with its [`Constraint`] to the [`Layout`]
    pub fn push<T, C>(&mut self, child: T, constraint: C)
    where
        T: Into<Box<dyn Widget>>,
        C: Into<Constraint>,
    {
        self.child.push(child, constraint);
    }
}

impl<W> Widget for BgGrad<W>
where
    W: Widget,
{
    fn render(&self, buffer: &mut Buffer, rect: Rect) {
        if rect.is_empty() {
            return;
        }

        match self.direction {
            Direction::Vertical => self.ver_render(buffer, &rect),
            Direction::Horizontal => self.hor_render(buffer, &rect),
        };
        self.child.render(buffer, rect.inner(self.padding));
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
