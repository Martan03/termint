use std::iter::repeat;

use crate::{enums::cursor::Cursor, geometry::coords::Coords};

use super::{border::Border, widget::Widget};

pub struct Block {
    title: String,
    borders: u8,
}

impl Block {
    /// Creates new [`Block`] with given title and all borders
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            borders: Border::ALL,
        }
    }

    /// Sets title of [`Block`]
    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    /// Sets on which sides border of the [`Block`] should be rendered
    pub fn borders(mut self, borders: u8) -> Self {
        self.borders = borders;
        self
    }

    /// Renders corner of [`Block`] border if needed based on `border` value
    fn render_corner(&self, c: char, x: usize, y: usize, border: u8) {
        if (self.borders & border) == border {
            println!("{}{c}", Cursor::Pos(x, y));
        }
    }
}

impl Widget for Block {
    /// Renders [`Block`] with selected borders and title
    fn render(&self, pos: Coords, size: Coords) {
        if (self.borders & Border::LEFT) != 0 {
            for y in 0..=size.y {
                println!("{}\u{2502}", Cursor::Pos(pos.x, pos.y + y));
            }
        }
        if (self.borders & Border::RIGHT) != 0 {
            for y in 0..=size.y {
                println!("{}\u{2502}", Cursor::Pos(pos.x + size.x, pos.y + y));
            }
        }

        let hor_border: String = repeat('\u{2500}').take(size.x).collect();

        if (self.borders & Border::TOP) != 0 {
            println!("{}{hor_border}", Cursor::Pos(pos.x, pos.y));
        }
        if (self.borders & Border::BOTTOM) != 0 {
            println!("{}{hor_border}", Cursor::Pos(pos.x, pos.y + size.y));
        }

        if size.x != 1 && size.y != 1 {
            self.render_corner(
                '\u{250C}',
                pos.x,
                pos.y,
                Border::TOP | Border::LEFT,
            );
            self.render_corner(
                '\u{2510}',
                pos.x + size.x,
                pos.y,
                Border::TOP | Border::RIGHT,
            );
            self.render_corner(
                '\u{2514}',
                pos.x,
                pos.y + size.y,
                Border::BOTTOM | Border::LEFT,
            );
            self.render_corner(
                '\u{2518}',
                pos.x + size.x,
                pos.y + size.y,
                Border::BOTTOM | Border::RIGHT,
            );
        }

        println!("{}{}", Cursor::Pos(pos.x + 2, pos.y), self.title);
    }
}
