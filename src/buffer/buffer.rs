use std::{
    io::{stdout, Write},
    ops::{Index, IndexMut},
};

use crate::{
    enums::{Color, Cursor, Modifier},
    geometry::{Rect, Vec2},
    style::Style,
};

use super::cell::Cell;

/// A buffer that stores the result of the widget render method. Every widget
/// interacts with the buffer, instead of printing to the terminal.
///
/// [`Term`] struct automatically renders on the whole screen using the
/// [`Buffer`], so you don't have to care about it. But if you would like to do
/// it without the [`Term`] struct, you can do it like this:
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     widgets::{Block, Widget},
/// #     geometry::Rect,
/// # };
/// # fn get_your_widget() -> Block { Block::vertical() }
/// // Creates new buffer with desired position and size given by the [`Rect`]
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 20, 9));
///
/// // Gets widget and renders it to the [`Buffer`]
/// let widget = get_your_widget();
/// widget.render(&mut buffer);
///
/// // Renders [`Buffer`], which prints the result to the terminal
/// buffer.render();
/// ```
#[derive(Debug, Clone)]
pub struct Buffer {
    rect: Rect,
    content: Vec<Cell>,
}

impl Buffer {
    /// Creates new [`Buffer`] with all cells set to the default cell
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
        let mut style =
            (Color::Default, Color::Default, Modifier::empty()).into();

        for y in 0..self.height() {
            print!("{}", Cursor::Pos(self.x(), self.y() + y));
            for _ in 0..self.width() {
                let child = self[id];
                style = self.render_cell(&child, style);
                id += 1;
            }
        }
        print!("\x1b[0m");
        _ = stdout().flush();
    }

    /// Renders different characters only between current buffer and given one
    pub fn render_diff(&self, diff: &Buffer) {
        // Rerenders whole buffer when size or position is different
        // TODO: make it compare the cells on shared positions
        if self.rect() != diff.rect() {
            self.render();
            return;
        }

        let mut id = 0;
        let mut style =
            (Color::Default, Color::Default, Modifier::empty()).into();

        for y in 0..self.height() {
            let mut prev = false;
            for x in 0..self.width() {
                let child = self[id];
                let dchild = diff[id];

                id += 1;
                if child == dchild {
                    prev = false;
                    continue;
                }

                if !prev {
                    print!("{}", Cursor::Pos(self.x() + x, self.y() + y))
                }
                style = self.render_cell(&child, style);
                prev = true;
            }
        }

        print!("\x1b[0m");
        _ = stdout().flush();
    }

    /// Gets subset of the [`Buffer`] based on given rectangle
    ///
    /// # Panics
    /// Will panic if the given rectangle isn't contained in current buffer
    pub fn subset(&self, rect: Rect) -> Buffer {
        let mut buffer = Buffer::empty(rect);

        for pos in rect.into_iter() {
            buffer.set(self[self.index_of(&pos)], &pos);
        }
        buffer
    }

    /// Unites current buffer with given one
    #[deprecated(
        since = "0.5.1",
        note = "Kept for compatibility purposes; use `merge` function instead"
    )]
    pub fn union(&mut self, buffer: Buffer) {
        self.merge(buffer);
    }

    /// Merges given buffer to the current,
    pub fn merge(&mut self, buffer: Buffer) {
        let rect = self.rect().union(buffer.rect());

        let mut merged = Buffer::empty(rect);
        for pos in self.rect().into_iter() {
            merged.set(self[pos], &pos);
        }
        for pos in buffer.rect().into_iter() {
            merged.set(buffer[pos], &pos);
        }

        self.rect = merged.rect;
        self.content = merged.content;
    }

    /// Gets [`Cell`] reference from the buffer on given position
    ///
    /// # Panics
    /// Will panic if the given position is outside of the buffer
    pub fn cell(&self, pos: &Vec2) -> Option<&Cell> {
        let id = self.index_of(pos);
        self.content.get(id)
    }

    /// Gets [`Cell`] mutable reference from the buffer on given position
    ///
    /// # Panics
    /// Will panic if the given position is outside of the buffer
    pub fn cell_mut(&mut self, pos: &Vec2) -> Option<&mut Cell> {
        let id = self.index_of(pos);
        self.content.get_mut(id)
    }

    /// Sets cell to given value on given position relative to buffer
    ///
    /// # Panics
    /// Will panic if the given position is outside of the buffer
    pub fn set(&mut self, cell: Cell, pos: &Vec2) {
        let id = self.index_of(pos);
        self.content[id] = cell;
    }

    /// Sets cell values to string starting at given coordinates
    /// TODO: set only part of the string that fits to buffer
    pub fn set_str<T>(&mut self, str: T, pos: &Vec2)
    where
        T: AsRef<str>,
    {
        let mut id = self.index_of(pos);
        for c in str.as_ref().chars() {
            self[id] = self[id].val(c);
            id += 1;
        }
    }

    /// Sets cell style and values starting at given coordinates
    /// TODO: set only part of the string that fits to buffer
    pub fn set_str_styled<T1, T2>(&mut self, str: T1, pos: &Vec2, style: T2)
    where
        T1: AsRef<str>,
        T2: Into<Style>,
    {
        let style = style.into();
        let mut id = self.index_of(pos);
        for c in str.as_ref().chars() {
            self[id] = self[id].val(c).style(style);
            id += 1;
        }
    }

    /// Sets value of the cell on given position relative to buffer
    ///
    /// # Panics
    /// Will panic if the given position is outside of the buffer
    pub fn set_val(&mut self, val: char, pos: &Vec2) {
        let id = self.index_of(pos);
        self[id] = self[id].val(val);
    }

    /// Sets style of the cell on given coordinates to given value
    ///
    /// # Panics
    /// Will panic if the given position is outside of the buffer
    pub fn set_style(&mut self, style: Style, pos: &Vec2) {
        let id = self.index_of(pos);
        self[id] = self[id].style(style);
    }

    /// Sets foreground of the cell on given position
    ///
    /// # Panics
    /// Will panic if the given position is outside of the buffer
    pub fn set_fg(&mut self, fg: Color, pos: &Vec2) {
        let id = self.index_of(pos);
        self[id] = self[id].fg(fg);
    }

    /// Sets foreground of the cell on given position
    ///
    /// # Panics
    /// Will panic if the given position is outside of the buffer
    pub fn set_bg(&mut self, bg: Color, pos: &Vec2) {
        let id = self.index_of(pos);
        self[id] = self[id].bg(bg);
    }

    /// Sets modifier of the cell on given position
    ///
    /// # Panics
    /// Will panic if the given position is outside of the buffer
    pub fn set_modifier(&mut self, modifier: u8, pos: &Vec2) {
        let id = self.index_of(pos);
        self[id] = self[id].modifier(modifier);
    }

    /// Gets reference to [`Rect`] of the [`Buffer`]
    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    /// Gets reference to position of the [`Buffer`]
    pub fn pos(&self) -> &Vec2 {
        self.rect.pos()
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

    /// Gets reference to size of the [`Buffer`]
    pub fn size(&self) -> &Vec2 {
        self.rect.size()
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

    /// Gets [`Cell`] index based on given position
    pub fn index_of(&self, pos: &Vec2) -> usize {
        self.index_of_opt(pos).unwrap_or_else(|| {
            panic!("position {} is outside of the buffer", pos)
        })
    }

    /// Gets [`Cell`] optional index based on given position
    pub fn index_of_opt(&self, pos: &Vec2) -> Option<usize> {
        if !self.rect.contains_pos(pos) {
            return None;
        }
        Some((pos.x - self.x()) + (pos.y - self.y()) * self.rect.width())
    }

    /// Gets position of the [`Cell`] based on index
    pub fn pos_of(&self, id: usize) -> Vec2 {
        self.pos_of_opt(id)
            .unwrap_or_else(|| panic!("index {id} is outside of the buffer"))
    }

    /// Gets optional position of the [`Cell`] based on index
    pub fn pos_of_opt(&self, id: usize) -> Option<Vec2> {
        if id >= self.content.len() {
            return None;
        }
        let (x, y) = (id % self.width(), id / self.width());
        Some(Vec2::new(x + self.x(), y + self.y()))
    }
}

impl Buffer {
    /// Renders given cell and returns current style
    fn render_cell(&self, cell: &Cell, mut style: Style) -> Style {
        if cell.modifier != style.modifier {
            style = (Color::Default, Color::Default, cell.modifier).into();
            print!("\x1b[0m{}", cell.modifier);
        }
        if cell.fg != style.fg.unwrap_or(Color::Default) {
            style = style.fg(cell.fg);
            print!("{}", cell.fg.to_fg());
        }
        if cell.bg != style.bg.unwrap_or(Color::Default) {
            style = style.bg(cell.bg);
            print!("{}", cell.bg.to_bg());
        }
        print!("{}", cell.val);
        style
    }
}

impl Index<usize> for Buffer {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        self.content.get(index).unwrap_or_else(|| {
            panic!("index {index} is outside of the buffer")
        })
    }
}

impl IndexMut<usize> for Buffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.content.get_mut(index).unwrap_or_else(|| {
            panic!("index {index} is outside of the buffer")
        })
    }
}

impl Index<Vec2> for Buffer {
    type Output = Cell;

    fn index(&self, index: Vec2) -> &Self::Output {
        self.cell(&index).unwrap_or_else(|| {
            panic!("position {} is outside of the buffer", index)
        })
    }
}

impl IndexMut<Vec2> for Buffer {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        self.cell_mut(&index).unwrap_or_else(|| {
            panic!("position {} is outside of the buffer", index)
        })
    }
}
