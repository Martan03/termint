use std::io::{stdout, Write};

use crate::{
    enums::{cursor::Cursor, Color},
    geometry::{coords::Coords, rect::Rect},
};

use super::cell::Cell;

/// Represents rendering buffer
#[derive(Debug)]
pub struct Buffer {
    rect: Rect,
    content: Vec<Cell>,
}

impl Buffer {
    /// Creates new empty [`Buffer`]
    pub fn empty(rect: Rect) -> Self {
        let area = rect.area();
        Self {
            rect,
            content: vec![Cell::default(); area],
        }
    }

    /// Creates new [`Buffer`] filled with given [`Cell`]
    pub fn filled(rect: Rect, cell: Cell) -> Self {
        let area = rect.area();
        Self {
            rect,
            content: vec![cell; area],
        }
    }

    /// Renders the [`Buffer`]
    pub fn render(&self) {
        let mut id = 0;
        for y in 0..self.height() {
            print!("{}", Cursor::Pos(self.x(), self.y() + y));
            for _ in 0..self.width() {
                print!("{}", self.content[id]);
                id += 1;
            }
        }

        print!("\x1b[0m");
        _ = stdout().flush();
    }

    /// Gets subset of the [`Buffer`] based on given rectangle
    pub fn get_subset(&self, rect: Rect) -> Buffer {
        let mut buffer = Buffer::empty(rect);

        for y in buffer.y()..buffer.height() + buffer.y() {
            for x in buffer.x()..buffer.width() + buffer.x() {
                buffer.set(
                    self.content[self.index_of(&Coords::new(x, y))],
                    &Coords::new(x, y),
                );
            }
        }
        buffer
    }

    /// Unites buffers
    pub fn union(&mut self, buffer: Buffer) {
        for (i, cell) in buffer.content().iter().enumerate() {
            self.set(*cell, &buffer.coords_of(i));
        }
    }

    /// Sets cell to given value on given position relative to buffer
    pub fn set(&mut self, cell: Cell, pos: &Coords) {
        let id = self.index_of(pos);
        self.content[id] = cell;
    }

    /// Sets cell values to string starting at given coordinates
    pub fn set_str<T>(&mut self, str: T, pos: &Coords)
    where
        T: AsRef<str>,
    {
        let mut id = self.index_of(pos);
        for c in str.as_ref().chars() {
            self.content[id].val(c);
            id += 1;
        }
    }

    /// Sets cell style and values starting at given coordinates
    pub fn set_str_styled<T1, T2, T3>(
        &mut self,
        str: T1,
        pos: &Coords,
        fg: T2,
        bg: T3,
    ) where
        T1: AsRef<str>,
        T2: Into<Option<Color>>,
        T3: Into<Option<Color>>,
    {
        let fg = fg.into();
        let bg = bg.into();

        let mut id = self.index_of(pos);
        for c in str.as_ref().chars() {
            self.content[id].val(c);

            if let Some(fg) = fg {
                self.content[id].fg(fg);
            }
            if let Some(bg) = bg {
                self.content[id].bg(bg);
            }
            id += 1;
        }
    }

    /// Sets value of the cell on given position relative to buffer
    pub fn set_val(&mut self, val: char, pos: &Coords) {
        let id = self.index_of(pos);
        self.content[id].val(val);
    }

    /// Sets foreground of the cell on given position relative to buffer
    pub fn set_fg(&mut self, fg: Color, pos: &Coords) {
        let id = self.index_of(pos);
        self.content[id].fg(fg);
    }

    /// Sets foreground of the cell on given position relative to buffer
    pub fn set_bg(&mut self, bg: Color, pos: &Coords) {
        let id = self.index_of(pos);
        self.content[id].bg(bg);
    }

    /// Gets position of the [`Buffer`]
    pub fn pos(&self) -> Coords {
        self.rect.pos()
    }

    /// Gets position of the [`Buffer`] as reference
    pub fn pos_ref(&self) -> &Coords {
        self.rect.pos_ref()
    }

    /// Gets x coordinate of the [`Buffer`]
    pub fn x(&self) -> usize {
        self.rect.x()
    }

    /// Gets x coordinate of the [`Buffer`]
    pub fn left(&self) -> usize {
        self.rect.left()
    }

    /// Gets x coordinate of the most right cell of the [`Buffer`]
    pub fn right(&self) -> usize {
        self.rect.right()
    }

    /// Gets y coordinate of the [`Buffer`]
    pub fn y(&self) -> usize {
        self.rect.y()
    }

    /// Gets y coordinate of the [`Buffer`]
    pub fn top(&self) -> usize {
        self.rect.top()
    }

    /// Gets y coordinate of the most bottom cell of the [`Buffer`]
    pub fn bottom(&self) -> usize {
        self.rect.bottom()
    }

    /// Gets size of the [`Buffer`]
    pub fn size(&self) -> Coords {
        self.rect.size()
    }

    /// Gets size of the [`Buffer`] as reference
    pub fn size_ref(&self) -> &Coords {
        self.rect.size_ref()
    }

    /// Gets width of the [`Buffer`]
    pub fn width(&self) -> usize {
        self.rect.width()
    }

    /// Gets height of the [`Buffer`]
    pub fn height(&self) -> usize {
        self.rect.height()
    }

    /// Gets area of the [`Buffer`]
    pub fn area(&self) -> usize {
        self.rect.area()
    }

    /// Gets [`Buffer`] content
    pub fn content(&self) -> &[Cell] {
        &self.content
    }

    /// Gets [`Cell`] index based on coordinates
    pub fn index_of(&self, pos: &Coords) -> usize {
        (pos.x - self.x()) + (pos.y - self.y()) * self.rect.width()
    }

    /// Gets coordinates of the [`Cell`] based on index
    pub fn coords_of(&self, id: usize) -> Coords {
        let (x, y) = (id % self.width(), id / self.width());
        Coords::new(x + self.x(), y + self.y())
    }
}
