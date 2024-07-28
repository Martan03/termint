use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

use crate::{
    buffer::buffer::Buffer,
    enums::Color,
    geometry::{coords::Coords, rect::Rect},
};

use super::{span::StrSpanExtension, text::Text, widget::Widget};

/// State of the [`List`] widget
#[derive(Debug)]
pub struct ListState {
    pub offset: usize,
    pub selected: Option<usize>,
}

impl ListState {
    /// Creates new [`ListState`] with given offset and no item selected
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            selected: None,
        }
    }

    /// Creates new [`ListState`] with given offset and selected item
    pub fn selected(offset: usize, selected: usize) -> Self {
        Self {
            offset,
            selected: Some(selected),
        }
    }
}

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
/// ```ignore
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
    state: Rc<RefCell<ListState>>,
    fg: Color,
    sel_fg: Color,
    sel_bg: Option<Color>,
    sel_char: String,
    scrollbar_fg: Color,
    thumb_fg: Color,
}

impl List {
    /// Creates new [`List`] with given items and given state
    pub fn new<T>(items: Vec<T>, state: Rc<RefCell<ListState>>) -> Self
    where
        T: AsRef<str>,
    {
        let items: Vec<String> =
            items.iter().map(|i| i.as_ref().to_string()).collect();

        Self {
            items,
            state,
            fg: Color::Default,
            sel_fg: Color::Cyan,
            sel_bg: None,
            sel_char: String::new(),
            scrollbar_fg: Color::Default,
            thumb_fg: Color::Default,
        }
    }

    /// Sets selected item in [`List`]
    pub fn selected<T>(self, current: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.state.borrow_mut().selected = current.into();
        self
    }

    /// Sets foreground of [`List`] item
    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    /// Sets [`List`] selected item foreground color
    pub fn sel_fg(mut self, sel_color: Color) -> Self {
        self.sel_fg = sel_color;
        self
    }

    /// Sets [`List`] selected item background color
    pub fn sel_bg<T>(mut self, sel_color: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.sel_bg = sel_color.into();
        self
    }

    /// Sets [`List`] selected item character
    /// Character that will display in front of selected items.
    /// Other items will be shifted to be aligned with selected item
    pub fn sel_char<T>(mut self, sel_char: T) -> Self
    where
        T: AsRef<str>,
    {
        self.sel_char = sel_char.as_ref().to_string();
        self
    }

    /// Sets [`List`] scrollbar color
    pub fn scrollbar_fg(mut self, fg: Color) -> Self {
        self.scrollbar_fg = fg;
        self
    }

    /// Sets [`List`] scrollbar thumb color
    pub fn thumb_fg(mut self, fg: Color) -> Self {
        self.thumb_fg = fg;
        self
    }
}

impl Widget for List {
    fn render(&self, buffer: &mut Buffer) {
        if !self.fits(buffer.size_ref()) {
            self.render_scrollbar(buffer);
        }

        let mut text_pos =
            Coords::new(buffer.x() + self.sel_char.len(), buffer.y());
        let mut text_size =
            Coords::new(buffer.width() - self.sel_char.len(), buffer.height());
        let offset = self.get_render_offset(buffer.size_ref());

        let selected = self.state.borrow().selected;
        for i in offset..self.items.len() {
            let mut fg = self.fg;
            let mut bg: Option<Color> = None;
            if Some(i) == selected {
                fg = self.sel_fg;
                bg = self.sel_bg;
            }

            let span = self.items[i].fg(fg).bg(bg);
            let mut ibuffer =
                buffer.get_subset(Rect::from_coords(text_pos, text_size));
            let res_pos = span.render_offset(&mut ibuffer, 0, None);
            buffer.union(ibuffer);

            text_size.y = text_size.y.saturating_sub(res_pos.y - text_pos.y);
            text_pos.y = res_pos.y + 1;

            if buffer.y() + buffer.height() <= text_pos.y {
                break;
            }
            text_size.y = buffer.y() + buffer.height() - text_pos.y;
        }
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

impl List {
    /// Renders [`List`] scrollbar
    fn render_scrollbar(&self, buffer: &mut Buffer) {
        let rat = self.items.len() as f32 / buffer.height() as f32;
        let thumb_size =
            min((buffer.height() as f32 / rat) as usize, buffer.height());
        let thumb_offset = min(
            (self.state.borrow().offset as f32 / rat) as usize,
            buffer.height() - thumb_size,
        );

        let x = (buffer.x() + buffer.width()).saturating_sub(1);
        let mut bar_pos = Coords::new(x, buffer.y());
        for _ in 0..buffer.height() {
            buffer.set_val('|', &bar_pos);
            buffer.set_fg(self.scrollbar_fg, &bar_pos);
            bar_pos.y += 1;
        }

        bar_pos = Coords::new(x, buffer.y() + thumb_offset);
        for _ in 0..thumb_size {
            buffer.set_val('┃', &bar_pos);
            buffer.set_fg(self.thumb_fg, &bar_pos);
            bar_pos.y += 1;
        }
    }

    fn get_render_offset(&self, _size: &Coords) -> usize {
        let Some(current) = self.state.borrow().selected else {
            return self.state.borrow().offset;
        };
        // let Some(prev_offset) = self.prev_offset else {
        //     return self.state.borrow().offset;
        // };

        // if prev_offset > current {
        //     return current;
        // }

        // let mut offset = prev_offset;
        // while !self.is_visible(current, offset, size) {
        //     offset += 1;
        // }
        // offset
        current
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
