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
/// #     widgets::{Block, Widget, Layout},
/// #     geometry::Rect,
/// # };
/// # fn get_your_widget() -> Block<Layout> { Block::vertical() }
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
    ///
    /// This method accepts parameter that can be converted to [`Rect`] (e.g.
    /// (x, y, width, height), or Rect::new(x, y, width, height)).
    #[must_use]
    pub fn empty<R>(rect: R) -> Self
    where
        R: Into<Rect>,
    {
        let rect = rect.into();
        let area = rect.area();
        Self {
            rect,
            content: vec![Cell::default(); area],
        }
    }

    /// Creates new [`Buffer`] filled with given [`Cell`]
    ///
    /// This method accepts parameter that can be converted to [`Rect`] (e.g.
    /// (x, y, width, height), or Rect::new(x, y, width, height)).
    #[must_use]
    pub fn filled<R>(rect: R, cell: Cell) -> Self
    where
        R: Into<Rect>,
    {
        let rect = rect.into();
        let area = rect.area();
        Self {
            rect,
            content: vec![cell; area],
        }
    }

    /// Prints the content of the buffer to standard output
    pub fn render(&self) {
        let mut id = 0;
        let mut style = (Color::Default, Color::Default, Modifier::empty());

        for y in 0..self.height() {
            print!("{}", Cursor::Pos(self.x(), self.y() + y));
            for _ in 0..self.width() {
                let child = self.content[id];
                style = self.render_cell(&child, style);
                id += 1;
            }
        }
        print!("\x1b[0m");
        _ = stdout().flush();
    }

    /// Prints buffer characters, that are different then in given
    /// buffer
    ///
    /// When the buffer sizes differ, it re-renders the whole buffer
    pub fn render_diff(&self, diff: &Buffer) {
        // TODO: make it compare the cells on shared positions
        if self.rect() != diff.rect() {
            self.render();
            return;
        }

        let mut id = 0;
        let mut style = (Color::Default, Color::Default, Modifier::empty());

        for y in 0..self.height() {
            let mut prev = false;
            for x in 0..self.width() {
                let child = self.content[id];
                let dchild = diff.content[id];

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

    /// Gets subset of the buffer based on given rectangle
    ///
    /// # Panics
    /// Panics if the given rectangle isn't contained in the buffer
    #[must_use]
    pub fn subset(&self, rect: Rect) -> Buffer {
        let mut buffer = Buffer::empty(rect);

        for pos in rect.into_iter() {
            buffer.set(self[self.index_of(&pos)], &pos);
        }
        buffer
    }

    /// Unites current buffer with given one
    #[deprecated(
        since = "0.6.0",
        note = "Kept for compatibility purposes; use `merge` function instead"
    )]
    pub fn union(&mut self, buffer: Buffer) {
        self.merge(buffer);
    }

    /// Merges given buffer to the current. If the given buffer is not
    /// contained in the current buffer, current buffer will be resized so to
    /// contain the given buffer.
    pub fn merge(&mut self, buffer: Buffer) {
        let rect = self.rect().union(buffer.rect());

        let mut merged = Buffer::empty(rect);
        for (i, pos) in self.rect().into_iter().enumerate() {
            merged.set(self.content[i], &pos);
        }
        for (i, pos) in buffer.rect().into_iter().enumerate() {
            merged.set(buffer.content[i], &pos);
        }

        self.rect = merged.rect;
        self.content = merged.content;
    }

    /// Moves buffer to given position
    pub fn move_to(&mut self, pos: Vec2) {
        self.rect.move_to(pos);
    }

    /// Gets [`Cell`] reference from the buffer on given position
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn cell(&self, pos: &Vec2) -> Option<&Cell> {
        let id = self.index_of(pos);
        self.content.get(id)
    }

    /// Gets [`Cell`] mutable reference from the buffer on given position
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn cell_mut(&mut self, pos: &Vec2) -> Option<&mut Cell> {
        let id = self.index_of(pos);
        self.content.get_mut(id)
    }

    /// Sets [`Cell`] on given position in the buffer to given value
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn set(&mut self, cell: Cell, pos: &Vec2) {
        let id = self.index_of(pos);
        self.content[id] = cell;
    }

    /// Prints given string to the [`Buffer`] starting at the given position.
    ///
    /// Truncates the string if it cannot fit the buffer.
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn set_str<T>(&mut self, str: T, pos: &Vec2)
    where
        T: AsRef<str>,
    {
        let mut id = self.index_of(pos);
        let left = self.content.len().saturating_sub(id);

        for c in str.as_ref().chars().take(left) {
            self.content[id] = self.content[id].val(c);
            id += 1;
        }
    }

    /// Prints given string to the [`Buffer`] with given [`Style`] starting at
    /// the given position.
    ///
    /// Truncates the string if it cannot fit the buffer.
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn set_str_styled<T, S>(&mut self, str: T, pos: &Vec2, style: S)
    where
        T: AsRef<str>,
        S: Into<Style>,
    {
        let mut id = self.index_of(pos);
        let left = self.content.len().saturating_sub(id);

        let style = style.into();
        for c in str.as_ref().chars().take(left) {
            self.content[id] = self.content[id].val(c).style(style);
            id += 1;
        }
    }

    /// Sets value of the [`Cell`] on given position in the buffer
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn set_val(&mut self, val: char, pos: &Vec2) {
        let id = self.index_of(pos);
        self.content[id] = self.content[id].val(val);
    }

    /// Sets style of the [`Cell`] on given position in the buffer
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn set_style(&mut self, style: Style, pos: &Vec2) {
        let id = self.index_of(pos);
        self.content[id] = self.content[id].style(style);
    }

    /// Sets foreground of the [`Cell`] on given position in the buffer
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn set_fg(&mut self, fg: Color, pos: &Vec2) {
        let id = self.index_of(pos);
        self.content[id] = self.content[id].fg(fg);
    }

    /// Sets background of the [`Cell`] on given position in the buffer
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn set_bg(&mut self, bg: Color, pos: &Vec2) {
        let id = self.index_of(pos);
        self.content[id] = self.content[id].bg(bg);
    }

    /// Sets modifier of the [`Cell`] on given position in the buffer
    ///
    /// # Panics
    /// Panics if the given position is outside of the buffer
    pub fn set_modifier(&mut self, modifier: u8, pos: &Vec2) {
        let id = self.index_of(pos);
        self.content[id] = self.content[id].modifier(modifier);
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

    /// Gets [`Cell`] index based on given position. Does not check if given
    /// position is inside of the buffer.
    pub fn index_of(&self, pos: &Vec2) -> usize {
        (pos.x - self.x()) + (pos.y - self.y()) * self.rect.width()
    }

    /// Gets [`Cell`] optional index based on given position. Returns `None` if
    /// the position is outside of the buffer
    pub fn index_of_opt(&self, pos: &Vec2) -> Option<usize> {
        if !self.rect.contains_pos(pos) {
            return None;
        }
        Some((pos.x - self.x()) + (pos.y - self.y()) * self.rect.width())
    }

    /// Gets position of the [`Cell`] based on index. Does not check if given
    /// index is inside of the buffer.
    pub fn pos_of(&self, id: usize) -> Vec2 {
        let (x, y) = (id % self.width(), id / self.width());
        Vec2::new(x + self.x(), y + self.y())
    }

    /// Gets optional position of the [`Cell`] based on index. Returns `None`
    /// if the position is outside of the buffer
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
    fn render_cell(
        &self,
        cell: &Cell,
        mut style: (Color, Color, Modifier),
    ) -> (Color, Color, Modifier) {
        if cell.modifier != style.2 {
            style = (Color::Default, Color::Default, cell.modifier);
            print!("\x1b[0m{}", cell.modifier);
        }
        if cell.fg != style.0 {
            style.0 = cell.fg;
            print!("{}", cell.fg.to_fg());
        }
        if cell.bg != style.1 {
            style.1 = cell.bg;
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

impl<P> Index<P> for Buffer
where
    P: Into<Vec2>,
{
    type Output = Cell;

    fn index(&self, index: P) -> &Self::Output {
        let pos = index.into();
        self.cell(&pos).unwrap_or_else(|| {
            panic!("position {} is outside of the buffer", pos)
        })
    }
}

impl<P> IndexMut<P> for Buffer
where
    P: Into<Vec2>,
{
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        let pos = index.into();
        self.cell_mut(&pos).unwrap_or_else(|| {
            panic!("position {} is outside of the buffer", pos)
        })
    }
}
