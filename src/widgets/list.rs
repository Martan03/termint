use std::cmp::{max, min};

use crate::{enums::fg::Fg, geometry::coords::Coords};

use super::{span::StrSpanExtension, widget::Widget};

/// List widget with scrollbar
///
/// ## Example usage:
/// ```
/// # use termint::{
/// #     enums::fg::Fg, widgets::list::List,
/// #     geometry::coords::Coords, widgets::widget::Widget
/// # };
/// let list =
///     List::new(vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6"])
///         .current(Some(1))
///         .sel_fg(Fg::Yellow);
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
    scrollbar_fg: Fg,
    thumb_fg: Fg,
}

impl List {
    /// Creates new [`List`] with given items
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

    /// Sets current item in [`List`]
    pub fn current(mut self, current: Option<usize>) -> Self {
        self.current = current;
        self
    }

    /// Sets scroll offset of the [`List`]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Scrolls [`List`] from given item so current item is visible
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
    fn render(&self, pos: &Coords, size: &Coords) {
        let mut text_pos = Coords::new(pos.x, pos.y);
        let mut text_size = Coords::new(size.x - 1, size.y);
        let offset = self.get_offset(size);

        for i in offset..self.items.len() {
            let mut fg = self.fg;
            if Some(i) == self.current {
                fg = self.sel_fg;
            }

            let span = self.items[i].fg(fg);
            span.render(&text_pos, &text_size);
            text_pos.y += span.height(&text_size);

            if pos.y + size.y <= text_pos.y {
                break;
            }
            text_size.y = pos.y + size.y - text_pos.y;
        }
        self.render_scrollbar(
            &Coords::new((pos.x + size.x).saturating_sub(1), pos.y),
            size,
            offset,
        );
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
            scrollbar_fg: Fg::Default,
            thumb_fg: Fg::Default,
        }
    }
}

impl List {
    /// Renders [`List`] scrollbar
    fn render_scrollbar(&self, pos: &Coords, size: &Coords, offset: usize) {
        let rat = self.items.len() as f32 / size.y as f32;
        let thumb_size = min((size.y as f32 / rat) as usize, size.y);
        let thumb_offset =
            min((offset as f32 / rat) as usize, size.y - thumb_size);

        let mut bar_pos = Coords::new(pos.x, pos.y);
        let bar = "│".fg(self.scrollbar_fg);
        for _ in 0..size.y {
            bar.render(&bar_pos, size);
            bar_pos.y += 1;
        }

        bar_pos = Coords::new(pos.x, pos.y + thumb_offset);
        let thumb = "┃".fg(self.thumb_fg);
        for _ in 0..thumb_size {
            thumb.render(&bar_pos, size);
            bar_pos.y += 1;
        }
    }

    fn get_offset(&self, size: &Coords) -> usize {
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
}

// From implementations
impl From<List> for Box<dyn Widget> {
    fn from(value: List) -> Self {
        Box::new(value)
    }
}
