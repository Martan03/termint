use crate::{enums::fg::Fg, geometry::coords::Coords};

use super::{span::StrSpanExtension, widget::Widget};

#[derive(Debug)]
pub struct List {
    items: Vec<&'static str>,
    current: Option<usize>,
    offset: usize,
    fg: Fg,
    sel_fg: Fg,
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
}

impl Widget for List {
    fn render(&self, pos: &Coords, size: &Coords) {
        let mut text_pos = Coords::new(pos.x, pos.y);
        let mut text_size = Coords::new(size.x, size.y);

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
    }

    fn height(&self, _size: &Coords) -> usize {
        todo!()
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
        }
    }
}

// From implementations
impl From<List> for Box<dyn Widget> {
    fn from(value: List) -> Self {
        Box::new(value)
    }
}
