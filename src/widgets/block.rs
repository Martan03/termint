use std::iter::repeat;

use crate::{
    enums::cursor::Cursor,
    geometry::{constrain::Constrain, coords::Coords, direction::Direction},
};

use super::{
    border::{Border, BorderType},
    layout::Layout,
    widget::Widget,
};

pub struct Block {
    title: String,
    borders: u8,
    border_type: BorderType,
    layout: Layout,
}

impl Block {
    /// Creates new [`Block`] with no title and all borders
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            borders: Border::ALL,
            border_type: BorderType::Normal,
            layout: Layout::vertical(),
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

    /// Sets [`BorderType`] of the [`Block`]
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Sets [`Direction`] of the [`Block`]'s [`Layout`]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout = self.layout.direction(direction);
        self
    }

    /// Adds child to the [`Block`]'s [`Layout`]
    pub fn add_child(&mut self, child: Box<dyn Widget>, constrain: Constrain) {
        self.layout.add_child(child, constrain);
    }

    /// Renders corner of [`Block`] border if needed based on `border` value
    fn render_corner(&self, x: usize, y: usize, border: u8) {
        let c = self.border_type.get(border);
        if (self.borders & border) == border {
            println!("{}{c}", Cursor::Pos(x, y));
        }
    }
}

impl Widget for Block {
    /// Renders [`Block`] with selected borders and title
    fn render(&self, pos: &Coords, size: &Coords) {
        let ver = self.border_type.get(Border::LEFT);
        if (self.borders & Border::LEFT) != 0 {
            for y in 0..size.y {
                println!("{}{ver}", Cursor::Pos(pos.x, pos.y + y));
            }
        }
        if (self.borders & Border::RIGHT) != 0 {
            for y in 0..size.y {
                println!(
                    "{}{ver}",
                    Cursor::Pos(pos.x + size.x - 1, pos.y + y)
                );
            }
        }

        let hor = self.border_type.get(Border::TOP);
        let hor_border: String = repeat(hor).take(size.x).collect();
        if (self.borders & Border::TOP) != 0 {
            println!("{}{hor_border}", Cursor::Pos(pos.x, pos.y));
        }
        if (self.borders & Border::BOTTOM) != 0 {
            println!("{}{hor_border}", Cursor::Pos(pos.x, pos.y + size.y - 1));
        }

        if size.x != 1 && size.y != 1 {
            self.render_corner(pos.x, pos.y, Border::TOP | Border::LEFT);
            self.render_corner(
                pos.x + size.x - 1,
                pos.y,
                Border::TOP | Border::RIGHT,
            );
            self.render_corner(
                pos.x,
                pos.y + size.y - 1,
                Border::BOTTOM | Border::LEFT,
            );
            self.render_corner(
                pos.x + size.x - 1,
                pos.y + size.y - 1,
                Border::BOTTOM | Border::RIGHT,
            );
        }
        println!("{}{}", Cursor::Pos(pos.x + 1, pos.y), self.title);

        self.layout.render(
            &Coords::new(pos.x + 1, pos.y + 1),
            &Coords::new(size.x - 2, size.y - 2),
        );
    }
}
