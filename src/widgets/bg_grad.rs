use crate::{
    buffer::buffer::Buffer,
    enums::{rgb::RGB, Color},
    geometry::{
        constraint::Constraint, coords::Coords, direction::Direction,
        padding::Padding,
    },
};

use super::{layout::Layout, widget::Widget};

/// [`BgGrad`] widget renders Gradient Background and works as [`Layout`]
/// as well
///
/// ## Example usage using [`Term`] (automatically renders on full screen):
/// ```rust
/// # use termint::{term::Term, widgets::bg_grad::BgGrad};
/// # fn example() -> Result<(), &'static str> {
/// // Creates new background gradient with horizontal direction
/// let grad = BgGrad::horizontal((0, 150, 255), (150, 255, 0));
///
/// // Renders background gradient using Term struct
/// let term = Term::new();
/// term.render(grad)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Example usage without using [`Term`]:
/// ```rust
/// # use termint::{
/// #     buffer::buffer::Buffer,
/// #     geometry::rect::Rect,
/// #     widgets::{bg_grad::BgGrad, widget::Widget},
/// # };
/// // Creates new background gradient with horizontal direction
/// let grad = BgGrad::horizontal((0, 150, 255), (150, 255, 0));
///
/// // Renders background gradient using [`Buffer`] (position and size given
/// by the [`Rect`] supplied to the [`Buffer`])
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 20, 9));
/// grad.render(&mut buffer);
/// buffer.render();
/// ```
#[derive(Debug)]
pub struct BgGrad {
    bg_start: RGB,
    bg_end: RGB,
    direction: Direction,
    layout: Layout,
}

impl BgGrad {
    /// Creates new [`BgGrad`] with given gradient colors
    pub fn new<T1, T2>(direction: Direction, start: T1, end: T2) -> Self
    where
        T1: Into<RGB>,
        T2: Into<RGB>,
    {
        Self {
            bg_start: start.into(),
            bg_end: end.into(),
            direction,
            layout: Default::default(),
        }
    }

    /// Creates new vertical [`BgGrad`] with given gradient colors
    pub fn vertical<T1, T2>(start: T1, end: T2) -> Self
    where
        T1: Into<RGB>,
        T2: Into<RGB>,
    {
        Self {
            bg_start: start.into(),
            bg_end: end.into(),
            direction: Direction::Vertical,
            layout: Default::default(),
        }
    }

    /// Creates new horizontal [`BgGrad`] with given gradient colors
    pub fn horizontal<T1, T2>(start: T1, end: T2) -> Self
    where
        T1: Into<RGB>,
        T2: Into<RGB>,
    {
        Self {
            bg_start: start.into(),
            bg_end: end.into(),
            direction: Direction::Horizontal,
            layout: Default::default(),
        }
    }

    /// Sets gradient [`Direction`] of the [`BgGrad`]
    pub fn bg_dir(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets [`Direction`] of the [`Layout`]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout = self.layout.direction(direction);
        self
    }

    /// Sets [`Padding`] of the [`Layout`]
    pub fn padding<T>(mut self, padding: T) -> Self
    where
        T: Into<Padding>,
    {
        self.layout = self.layout.padding(padding);
        self
    }

    /// Centers the [`Layout`] of the [`BgGrad`]
    pub fn center(mut self) -> Self {
        self.layout = self.layout.center();
        self
    }

    /// Adds child to the [`BgGrad`]'s [`Layout`]
    pub fn add_child<T>(&mut self, child: T, constrain: Constraint)
    where
        T: Into<Box<dyn Widget>>,
    {
        self.layout.add_child(child, constrain);
    }
}

impl Widget for BgGrad {
    fn render(&self, buffer: &mut Buffer) {
        if buffer.width() == 0 || buffer.height() == 0 {
            return;
        }

        match self.direction {
            Direction::Vertical => self.render_ver(buffer),
            Direction::Horizontal => self.render_hor(buffer),
        };

        self.layout.render(buffer);
    }

    fn height(&self, size: &Coords) -> usize {
        self.layout.height(size)
    }

    fn width(&self, size: &Coords) -> usize {
        self.layout.width(size)
    }
}

impl BgGrad {
    /// Renders horizontal background gradient
    fn render_hor(&self, buffer: &mut Buffer) {
        let step = self.get_step(buffer.width() as i16);
        let (mut r, mut g, mut b) =
            (self.bg_start.r, self.bg_start.g, self.bg_start.b);

        for x in buffer.x()..buffer.width() + buffer.x() {
            let bg = Color::Rgb(r, g, b);
            (r, g, b) = self.add_step((r, g, b), step);

            for y in buffer.y()..buffer.height() + buffer.y() {
                buffer.set_bg(bg, &Coords::new(x, y));
            }
        }
    }

    /// Renders vertical background gradient
    fn render_ver(&self, buffer: &mut Buffer) {
        let step = self.get_step(buffer.height() as i16);
        let (mut r, mut g, mut b) =
            (self.bg_start.r, self.bg_start.g, self.bg_start.b);

        for y in buffer.y()..buffer.height() + buffer.y() {
            let bg = Color::Rgb(r, g, b);
            (r, g, b) = self.add_step((r, g, b), step);

            for x in buffer.x()..buffer.width() + buffer.x() {
                buffer.set_bg(bg, &Coords::new(x, y));
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
impl From<BgGrad> for Box<dyn Widget> {
    fn from(value: BgGrad) -> Self {
        Box::new(value)
    }
}
