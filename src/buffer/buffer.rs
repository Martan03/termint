use crate::{
    enums::{bg::Bg, fg::Fg},
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

    /// Unites buffers
    pub fn union(&mut self, buffer: Buffer) {
        for (i, cell) in buffer.content().iter().enumerate() {
            self.set(cell.clone(), buffer.coords_of(i));
        }
    }

    /// Sets cell to given value on given position relative to buffer
    pub fn set<T>(&mut self, cell: Cell, pos: T)
    where
        T: Into<Coords>,
    {
        let pos = pos.into();
        let id = self.index_of(&pos);
        self.content[id] = cell;
    }

    /// Sets value of the cell on given position relative to buffer
    pub fn set_val<T>(&mut self, val: char, pos: T)
    where
        T: Into<Coords>,
    {
        let pos = pos.into();
        let id = self.index_of(&pos);
        self.content[id].val(val);
    }

    /// Sets foreground of the cell on given position relative to buffer
    pub fn set_fg<T1, T2>(&mut self, fg: T1, pos: T2)
    where
        T1: Into<Option<Fg>>,
        T2: Into<Coords>,
    {
        let pos = pos.into();
        let id = self.index_of(&pos);
        self.content[id].fg(fg);
    }

    /// Sets foreground of the cell on given position relative to buffer
    pub fn set_bg<T1, T2>(&mut self, bg: T1, pos: T2)
    where
        T1: Into<Option<Bg>>,
        T2: Into<Coords>,
    {
        let pos = pos.into();
        let id = self.index_of(&pos);
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

    /// Gets y coordinate of the [`Buffer`]
    pub fn y(&self) -> usize {
        self.rect.y()
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
