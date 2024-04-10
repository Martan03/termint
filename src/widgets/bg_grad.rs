use std::io::{stdout, Write};

use crate::{
    enums::{bg::Bg, cursor::Cursor, rgb::RGB},
    geometry::{
        constrain::Constrain, coords::Coords, direction::Direction,
        padding::Padding,
    },
};

use super::{layout::Layout, widget::Widget};

/// [`BgGrad`] widget renders Gradient background and works as [`Layout`]
/// as well
///
/// Note that the background behind the layout will be overriden
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     geometry::coords::Coords,
/// #     widgets::{bg_grad::BgGrad, widget::Widget},
/// # };
/// // Creates new background gradient with horizontal direction
/// let grad = BgGrad::new((0, 150, 255), (150, 255, 0));
/// // Renders background gradient
/// grad.render(&Coords::new(1, 1), &Coords::new(20, 9));
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
    pub fn new<T, R>(start: T, end: R) -> Self
    where
        T: Into<RGB>,
        R: Into<RGB>,
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
    pub fn padding<T: Into<Padding>>(mut self, padding: T) -> Self {
        self.layout = self.layout.padding(padding);
        self
    }

    /// Centers the [`Layout`] of the [`BgGrad`]
    pub fn center(mut self) -> Self {
        self.layout = self.layout.center();
        self
    }

    /// Adds child to the [`BgGrad`]'s [`Layout`]
    pub fn add_child<T>(&mut self, child: T, constrain: Constrain)
    where
        T: Into<Box<dyn Widget>>,
    {
        self.layout.add_child(child, constrain);
    }
}

impl Widget for BgGrad {
    fn render(&self, pos: &Coords, size: &Coords) {
        print!("{}", self.get_string(pos, size));
        _ = stdout().flush();
    }

    fn get_string(&self, pos: &Coords, size: &Coords) -> String {
        if size.x == 0 || size.y == 0 {
            return String::new();
        }

        let mut res = match self.direction {
            Direction::Vertical => self.ver_render(pos, size),
            Direction::Horizontal => self.hor_render(pos, size),
        };
        res.push_str("\x1b[0m");
        res.push_str(&self.layout.get_string(pos, size));
        res
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
    fn hor_render(&self, pos: &Coords, size: &Coords) -> String {
        let step = self.get_step(size.x as i16);
        let (mut r, mut g, mut b) =
            (self.bg_start.r, self.bg_start.g, self.bg_start.b);

        let mut line = String::new();
        for _ in 0..size.x {
            line.push_str(&format!("{} ", Bg::RGB(r, g, b)));
            (r, g, b) = self.add_step((r, g, b), step);
        }

        let mut res = String::new();
        for y in 0..size.y {
            res.push_str(&Cursor::Pos(pos.x, pos.y + y).to_string());
            res.push_str(&line);
        }
        res
    }

    /// Renders vertical background gradient
    fn ver_render(&self, pos: &Coords, size: &Coords) -> String {
        let step = self.get_step(size.y as i16);
        let (mut r, mut g, mut b) =
            (self.bg_start.r, self.bg_start.g, self.bg_start.b);

        let mut res = String::new();
        for y in 0..size.y {
            let line = format!("{} ", Bg::RGB(r, g, b)).repeat(size.x);
            res.push_str(&Cursor::Pos(pos.x, pos.y + y).to_string());
            res.push_str(&line);
            (r, g, b) = self.add_step((r, g, b), step);
        }
        res
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
