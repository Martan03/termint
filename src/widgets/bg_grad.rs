use crate::{
    buffer::Buffer,
    enums::{Color, RGB},
    geometry::{Direction, Padding, Vec2},
};

use super::{widget::Widget, Element};

/// [`Layout`] widget with gradient background
///
/// ## Example usage using [`Term`] (automatically renders on full screen):
/// ```rust
/// # use termint::{term::Term, widgets::BgGrad};
/// # fn example() -> Result<(), &'static str> {
/// // Creates new background gradient with horizontal direction
/// let grad = BgGrad::horizontal((0, 150, 255), (150, 255, 0));
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
/// #     widgets::{BgGrad, Widget},
/// # };
/// // Creates new background gradient with horizontal direction
/// let grad = BgGrad::horizontal((0, 150, 255), (150, 255, 0));
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

impl<W> BgGrad<W> {
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

impl Widget for BgGrad {
    fn render(&self, buffer: &mut Buffer) {
        if buffer.width() == 0 || buffer.height() == 0 {
            return;
        }

        match self.direction {
            Direction::Vertical => self.ver_render(buffer),
            Direction::Horizontal => self.hor_render(buffer),
        };

        let mut cbuffer = buffer.subset(buffer.rect().inner(self.padding));
        self.child.render(&mut cbuffer);
        buffer.merge(cbuffer);
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

impl BgGrad {
    /// Renders horizontal background gradient
    fn hor_render(&self, buffer: &mut Buffer) {
        let step = self.get_step(buffer.width() as i16);
        let (mut r, mut g, mut b) =
            (self.bg_start.r, self.bg_start.g, self.bg_start.b);

        for x in buffer.x()..buffer.width() + buffer.x() {
            let bg = Color::Rgb(r, g, b);
            (r, g, b) = self.add_step((r, g, b), step);

            for y in buffer.y()..buffer.height() + buffer.y() {
                buffer.set_bg(bg, &Vec2::new(x, y));
            }
        }
    }

    /// Renders vertical background gradient
    fn ver_render(&self, buffer: &mut Buffer) {
        let step = self.get_step(buffer.height() as i16);
        let (mut r, mut g, mut b) =
            (self.bg_start.r, self.bg_start.g, self.bg_start.b);

        for y in buffer.y()..buffer.height() + buffer.y() {
            let bg = Color::Rgb(r, g, b);
            (r, g, b) = self.add_step((r, g, b), step);

            for x in buffer.x()..buffer.width() + buffer.x() {
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
impl From<BgGrad> for Box<dyn Widget> {
    fn from(value: BgGrad) -> Self {
        Box::new(value)
    }
}

impl From<BgGrad> for Element {
    fn from(value: BgGrad) -> Self {
        Element::new(value)
        
    }
}
