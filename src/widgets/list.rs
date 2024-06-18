use std::{
    cmp::{max, min},
    io::{stdout, Write},
};

use crate::{
    buffer::buffer::Buffer,
    enums::{bg::Bg, cursor::Cursor, fg::Fg},
    geometry::coords::Coords,
};

use super::{span::StrSpanExtension, widget::Widget};

/// List widget with scrollbar
///
/// ### Features:
/// - Scrollbar (doesn't show when not necessary):
///     - Scrollbar foreground
///     - Scrollbar thumb color
/// - Selected item:
///     - Foreground
///     - Background
///     - Character in front
///
/// ## Example usage:
/// ```
/// # use termint::{
/// #     enums::fg::Fg, widgets::list::List,
/// #     geometry::coords::Coords, widgets::widget::Widget
/// # };
/// // Creates list, where selected item has yellow foreground and '*' in
/// // front of it
/// let list =
///     List::new(vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6"])
///         .current(Some(1))
///         .sel_fg(Fg::Yellow)
///         .sel_char("*");
/// list.render(&Coords::new(1, 1), &Coords::new(20, 5));
/// ```
#[derive(Debug)]
pub struct List {
    items: Vec<String>,
    current: Option<usize>,
    prev_offset: Option<usize>,
    offset: usize,
    fg: Fg,
    sel_fg: Fg,
    sel_bg: Option<Bg>,
    sel_char: String,
    scrollbar_fg: Fg,
    thumb_fg: Fg,
}

impl List {
    /// Creates new [`List`] with given items.
    /// Automatically sets current to the first item, when `items` aren't empty
    pub fn new<T>(items: Vec<T>) -> Self
    where
        T: AsRef<str>,
    {
        let items: Vec<String> =
            items.iter().map(|i| i.as_ref().to_string()).collect();
        let current = if items.is_empty() { None } else { Some(0) };

        Self {
            items,
            current,
            ..Default::default()
        }
    }

    /// Sets selected item in [`List`]
    /// This method exists for compatibility purposes and is deprecated, use
    /// `selected` method instead
    #[deprecated]
    pub fn current<T: Into<Option<usize>>>(mut self, current: T) -> Self {
        self.current = current.into();
        self
    }

    /// Sets selected item in [`List`]
    pub fn selected<T: Into<Option<usize>>>(mut self, current: T) -> Self {
        self.current = current.into();
        self
    }

    /// Sets scroll offset of the [`List`]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Gets current [`List`] offset
    pub fn get_offset(&self) -> usize {
        self.offset
    }

    /// Scrolls [`List`] from given offset so current item is visible
    pub fn to_current(mut self, from: usize) -> Self {
        self.prev_offset = Some(from);
        self
    }

    /// Sets foreground of [`List`] item
    pub fn fg(mut self, fg: Fg) -> Self {
        self.fg = fg;
        self
    }

    /// Sets [`List`] selected item foreground color
    pub fn sel_fg(mut self, sel_color: Fg) -> Self {
        self.sel_fg = sel_color;
        self
    }

    /// Sets [`List`] selected item background color
    pub fn sel_bg<T: Into<Option<Bg>>>(mut self, sel_color: T) -> Self {
        self.sel_bg = sel_color.into();
        self
    }

    /// Sets [`List`] selected item character
    /// Character that will display in front of selected items.
    /// Other items will be shifted to be aligned with selected item
    pub fn sel_char<T: AsRef<str>>(mut self, sel_char: T) -> Self {
        self.sel_char = sel_char.as_ref().to_string();
        self
    }

    /// Sets [`List`] scrollbar color
    pub fn scrollbar_fg(mut self, fg: Fg) -> Self {
        self.scrollbar_fg = fg;
        self
    }

    /// Sets [`List`] scrollbar thumb color
    pub fn thumb_fg(mut self, fg: Fg) -> Self {
        self.thumb_fg = fg;
        self
    }
}

impl Widget for List {
    fn render(&self, buffer: &mut Buffer) {
        print!("{}", self.get_string(&buffer.pos(), &buffer.size()));
        _ = stdout().flush();
    }

    fn get_string(&self, pos: &Coords, size: &Coords) -> String {
        let mut res = String::new();
        let mut text_pos = Coords::new(pos.x + self.sel_char.len(), pos.y);
        let mut text_size = Coords::new(size.x - self.sel_char.len(), size.y);
        let offset = self.get_render_offset(size);

        let fits = self.fits(size);
        if !fits {
            text_size.x -= 1;
            self.get_scrollbar(
                &mut res,
                &Coords::new((pos.x + size.x).saturating_sub(1), pos.y),
                size,
                offset,
            );
        }

        for i in offset..self.items.len() {
            let mut fg = self.fg;
            let mut bg: Option<Bg> = None;
            if Some(i) == self.current {
                res.push_str(&Cursor::Pos(pos.x, text_pos.y).to_string());
                res.push_str(&self.sel_char);
                fg = self.sel_fg;
                bg = self.sel_bg;
            }

            let span = self.items[i].fg(fg).bg(bg);
            res.push_str(&span.get_string(&text_pos, &text_size));
            text_pos.y += span.height(&text_size);

            if pos.y + size.y <= text_pos.y {
                break;
            }
            text_size.y = pos.y + size.y - text_pos.y;
        }
        res
    }

    fn height(&self, size: &Coords) -> usize {
        let mut height = 0;
        for i in 0..self.items.len() {
            let span = self.items[i].to_span();
            height += span.height(size);
        }
        height
    }

    fn width(&self, size: &Coords) -> usize {
        let mut width = 0;
        for item in self.items.iter() {
            let span = item.to_span();
            width = max(span.width(size), width);
        }
        width + 1
    }
}

impl Default for List {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            current: None,
            prev_offset: None,
            offset: 0,
            fg: Fg::Default,
            sel_fg: Fg::Cyan,
            sel_bg: None,
            sel_char: String::new(),
            scrollbar_fg: Fg::Default,
            thumb_fg: Fg::Default,
        }
    }
}

impl List {
    /// Renders [`List`] scrollbar
    fn get_scrollbar(
        &self,
        res: &mut String,
        pos: &Coords,
        size: &Coords,
        offset: usize,
    ) {
        let rat = self.items.len() as f32 / size.y as f32;
        let thumb_size = min((size.y as f32 / rat) as usize, size.y);
        let thumb_offset =
            min((offset as f32 / rat) as usize, size.y - thumb_size);

        let mut bar_pos = Coords::new(pos.x, pos.y);
        let bar = "│".fg(self.scrollbar_fg);
        for _ in 0..size.y {
            res.push_str(&bar.get_string(&bar_pos, size));
            bar_pos.y += 1;
        }

        bar_pos = Coords::new(pos.x, pos.y + thumb_offset);
        let thumb = "┃".fg(self.thumb_fg);
        for _ in 0..thumb_size {
            res.push_str(&thumb.get_string(&bar_pos, size));
            bar_pos.y += 1;
        }
    }

    fn get_render_offset(&self, size: &Coords) -> usize {
        let Some(current) = self.current else {
            return self.offset;
        };
        let Some(prev_offset) = self.prev_offset else {
            return self.offset;
        };

        if prev_offset > current {
            return current;
        }

        let mut offset = prev_offset;
        while !self.is_visible(current, offset, size) {
            offset += 1;
        }
        offset
    }

    /// Checks if item is visible with given offset
    fn is_visible(&self, item: usize, offset: usize, size: &Coords) -> bool {
        let mut height = 0;
        for i in offset..self.items.len() {
            height += self.items[i].to_span().height(size);
            if height > size.y {
                return false;
            }

            if i == item {
                return true;
            }
        }
        false
    }

    /// Checks if list fits to the visible area
    fn fits(&self, size: &Coords) -> bool {
        self.is_visible(self.items.len() - 1, 0, size)
    }
}

// From implementations
impl From<List> for Box<dyn Widget> {
    fn from(value: List) -> Self {
        Box::new(value)
    }
}
