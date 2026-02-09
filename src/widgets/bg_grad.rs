use crate::{
    buffer::Buffer,
    enums::{Color, RGB},
    geometry::{Direction, Padding, Rect, Vec2},
    prelude::MouseEvent,
    widgets::{cache::Cache, widget::EventResult},
};

use super::{widget::Widget, Element, Spacer};

/// A container widget that renders a gradient background behind child.
///
/// [`BgGrad`] fills the assigned area with linear gradient (vertical or
/// horizontal) and renders the optional child widget on top of it.
///
/// By default, a new [`BgGrad`] contains a [`Spacer`], meaning only the
/// gradient background will be visible. You can use [`BgGrad::child`] to add
/// any widget as its child.
///
/// # Example
///
/// ```rust
/// use termint::{prelude::*, widgets::BgGrad};
/// # type AnyWidget = Spacer;
///
/// // Horizontal blue to green gradient
/// let grad = BgGrad::<()>::new(Direction::Horizontal, (0, 0, 255), (0, 255, 0));
/// // Or using the horizontal gradient constructor
/// let grad = BgGrad::<()>::horizontal((0, 0, 255), (0, 255, 0));
///
/// // Vertical blue-green gradient
/// let grad = BgGrad::<()>::vertical((0, 0, 255), (0, 255, 0))
///     // Add any widget as its child
///     .child(AnyWidget::new())
///     // Add uniform padding between child and gradient edges
///     .padding(1);
/// ```
#[derive(Debug)]
pub struct BgGrad<M: 'static = ()> {
    bg_start: RGB,
    bg_end: RGB,
    direction: Direction,
    padding: Padding,
    child: Element<M>,
}

impl<M: Clone + 'static> BgGrad<M> {
    /// Creates a new [`BgGrad`] with the specified direction and colors.
    ///
    /// The `start` and `end` colors can be any type convertible into [`RGB`],
    /// such as `u32`, `(u8 ,u8, u8)`. You can read more in the [`RGB`]
    /// documentation.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::BgGrad};
    ///
    /// let bg = BgGrad::<()>::new(
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

    /// Creates a vertical (top-to-bottom) [`BgGrad`] with the specified
    /// colors.
    ///
    /// The `start` and `end` colors can be any type convertible into [`RGB`],
    /// such as `u32`, `(u8 ,u8, u8)`. You can read more in the [`RGB`]
    /// documentation.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::BgGrad};
    ///
    /// let bg = BgGrad::<()>::vertical((0, 150, 255), (150, 255, 0));
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

    /// Creates a horizontal (left-to-right) [`BgGrad`] with the specified
    /// colors.
    ///
    /// The `start` and `end` colors can be any type convertible into [`RGB`],
    /// such as `u32` (hex), `(u8 ,u8, u8)` (RGB). You can read more in the
    /// [`RGB`] documentation.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::BgGrad};
    ///
    /// let bg = BgGrad::<()>::horizontal((0, 150, 255), (150, 255, 0));
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

    fn construct<W>(start: RGB, end: RGB, dir: Direction, child: W) -> Self
    where
        W: Widget<M>,
    {
        Self {
            bg_start: start,
            bg_end: end,
            direction: dir,
            padding: Default::default(),
            child: Element::new(child),
        }
    }
}

impl<M> BgGrad<M> {
    /// Sets the child widget to be rendered on top of the gradient.
    ///
    /// If a child was already set, this will replace it.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, widgets::BgGrad};
    ///
    /// # type SomeWidget = Spacer;
    /// let widget = BgGrad::<()>::vertical((0, 150, 255), (150, 255, 0))
    ///     .child(SomeWidget::new());
    /// ```
    #[must_use]
    pub fn child<I>(mut self, child: I) -> Self
    where
        I: Into<Element<M>>,
    {
        self.child = child.into();
        self
    }

    /// Sets the gradient direction of the [`BgGrad`] background.
    ///
    /// The direction determines in which direction is the gradient drawn.
    #[deprecated(
        since = "0.7.0",
        note = "Replaced by `BgGrad::direction` for clarity."
    )]
    #[must_use]
    pub fn bg_dir(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the gradient direction of the [`BgGrad`] background.
    ///
    /// The direction determines in which direction is the gradient drawn.
    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the padding between the gradient adges and the child widget.
    ///
    /// The `padding` can be any type convertible into [`Padding`], such as
    /// `usize` (uniform), `(usize, usize)` (vertical, horizontal). You can
    /// read more in the [`Padding`] documentation.
    #[must_use]
    pub fn padding<T>(mut self, padding: T) -> Self
    where
        T: Into<Padding>,
    {
        self.padding = padding.into();
        self
    }
}

impl<M> Widget<M> for BgGrad<M>
where
    M: Clone + 'static,
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

    fn children(&self) -> Vec<&Element<M>> {
        vec![&self.child]
    }

    fn on_event(
        &self,
        area: Rect,
        cache: &mut Cache,
        event: &MouseEvent,
    ) -> EventResult<M> {
        if !area.contains_pos(&event.pos) {
            return EventResult::None;
        }
        self.child.on_event(
            area.inner(self.padding),
            &mut cache.children[0],
            event,
        )
    }
}

impl<M> BgGrad<M> {
    /// Renders horizontal background gradient
    fn hor_render(&self, buffer: &mut Buffer, rect: &Rect) {
        let step = self.get_step(rect.width() as i16 - 1);
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
        let step = self.get_step(rect.height() as i16 - 1);
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
        if len <= 0 {
            return (0, 0, 0);
        }
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
impl<M> From<BgGrad<M>> for Box<dyn Widget<M>>
where
    M: Clone + 'static,
{
    fn from(value: BgGrad<M>) -> Self {
        Box::new(value)
    }
}

impl<M> From<BgGrad<M>> for Element<M>
where
    M: Clone + 'static,
{
    fn from(value: BgGrad<M>) -> Self {
        Element::new(value)
    }
}
