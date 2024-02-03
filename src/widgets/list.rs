use std::cmp::min;

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
    items: Vec<&'static str>,
    current: Option<usize>,
    offset: usize,
    fg: Fg,
    sel_fg: Fg,
    scrollbar_fg: Fg,
    thumb_fg: Fg,
}

impl List {
    /// Creates new [`List`] with given items
    pub fn new(items: Vec<&'static str>) -> Self {
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

    /// Sets current item in [`List`] and scrolls to it
    pub fn current_scroll(mut self, current: Option<usize>) -> Self {
        self.current = current;
        self.offset = current.unwrap_or(0);
        self
    }

    /// Sets offset of [`List`]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
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

        for i in self.offset..self.items.len() {
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
        );
    }

    /// Gets height of the [`List`] with all items
    fn height(&self, size: &Coords) -> usize {
        let mut height = 0;
        for i in 0..self.items.len() {
            let span = self.items[i].to_span();
            height += span.height(size);
        }
        height
    }

    fn width(&self, _size: &Coords) -> usize {
        todo!()
    }
}

impl Default for List {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            current: None,
            offset: 0,
            fg: Fg::Default,
            sel_fg: Fg::Cyan,
            scrollbar_fg: Fg::Default,
            thumb_fg: Fg::Default,
        }
    }
}

// whole / x = part
// x = whole / part
impl List {
    /// Renders [`List`] scrollbar
    fn render_scrollbar(&self, pos: &Coords, size: &Coords) {
        let rat = self.items.len() as f32 / size.y as f32;
        let thumb_size = min((size.y as f32 / rat) as usize, size.y);
        let thumb_offset =
            min((self.offset as f32 / rat) as usize, size.y - thumb_size);

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
}

// From implementations
impl From<List> for Box<dyn Widget> {
    fn from(value: List) -> Self {
        Box::new(value)
    }
}
