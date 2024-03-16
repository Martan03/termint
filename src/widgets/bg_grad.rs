use crate::{
    enums::{bg::Bg, cursor::Cursor, rgb::RGB},
    geometry::{coords::Coords, direction::Direction, padding::Padding},
};

use super::{layout::Layout, widget::Widget};

/// [`BgGrad`] widget renders Gradient background and works as [`Layout`]
/// as well
#[derive(Debug)]
pub struct BgGrad {
    bg_start: RGB,
    bg_end: RGB,
    direction: Direction,
    layout: Layout,
}

impl BgGrad {
    /// Creates new [`BgGrad`] with given gradient colors
    pub fn new<T: Into<RGB>>(start: T, end: T) -> Self {
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
}

impl Widget for BgGrad {
    fn render(&self, pos: &Coords, size: &Coords) {
        self.hor_render(pos, size);
        println!("{}\x1b[0m", Cursor::Pos(pos.x, pos.y));
    }

    fn height(&self, size: &Coords) -> usize {
        todo!()
    }

    fn width(&self, size: &Coords) -> usize {
        todo!()
    }
}

impl BgGrad {
    /// Renders horizontal background gradient
    fn hor_render(&self, pos: &Coords, size: &Coords) {
        let step = self.get_step(size.x as i16);
        let (mut r, mut g, mut b) =
            (self.bg_start.r, self.bg_start.g, self.bg_start.b);

        let mut line = String::new();
        for _ in 0..size.x {
            line.push_str(&format!("{} ", Bg::RGB(r, g, b)));
            (r, g, b) = self.add_step((r, g, b), step);
        }

        for y in 0..size.y {
            print!("{}{}", Cursor::Pos(pos.x, pos.y + y), line);
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