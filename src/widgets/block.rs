use std::iter::repeat;

use crate::geometry::coords::Coords;

use super::border::Border;

pub struct Block {
    title: String,
    borders: u8,
}

impl Block {
    /// Creates new [`Block`] with given title and all borders
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self {
            title: title.into(),
            borders: Border::ALL,
        }
    }

    /// Sets on which sides border of the [`Block`] should be rendered
    pub fn borders(mut self, borders: u8) -> Self {
        self.borders = borders;
        self
    }

    /// Renders [`Block`] with selected borders and title
    pub fn render(&self, size: Coords, pos: Coords) {
        let hor_border: String = repeat('\u{2500}').take(size.x).collect();

        if (self.borders & Border::TOP) != 0 {
            println!("\x1b[{};{}H{hor_border}", pos.y, pos.x);
        }
        if (self.borders & Border::BOTTOM) != 0 {
            println!("\x1b[{};{}H{hor_border}", pos.y + size.y, pos.x);
        }

        if (self.borders & Border::LEFT) != 0 {
            for y in 0..=size.y {
                println!("\x1b[{};{}H\u{2502}", pos.y + y, pos.x);
            }
        }
        if (self.borders & Border::RIGHT) != 0 {
            for y in 0..=size.y {
                println!("\x1b[{};{}H\u{2502}", pos.y + y, pos.x + size.x);
            }
        }

        if (self.borders & (Border::TOP | Border::LEFT)) ==
            (Border::TOP | Border::LEFT) {
            println!("\x1b[{};{}H\u{250C}", pos.y, pos.x)
        }
        if (self.borders & (Border::TOP | Border::RIGHT)) ==
            (Border::TOP | Border::RIGHT) {
            println!("\x1b[{};{}H\u{2510}", pos.y, pos.x + size.x)
        }
        if (self.borders & (Border::BOTTOM | Border::LEFT)) ==
            (Border::BOTTOM | Border::LEFT) {
            println!("\x1b[{};{}H\u{2514}", pos.y + size.y, pos.x)
        }
        if (self.borders & (Border::BOTTOM | Border::RIGHT)) ==
            (Border::BOTTOM | Border::RIGHT) {
            println!("\x1b[{};{}H\u{2518}", pos.y + size.y, pos.x + size.x)
        }

        println!("\x1b[{};{}f{}", pos.y, pos.x + 2, self.title);
    }
}
