use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

use crate::{
    buffer::Buffer,
    enums::Color,
    geometry::{Rect, Vec2},
    style::Style,
};

use super::{span::StrSpanExtension, text::Text, widget::Widget, Element};

/// List widget with scrollbar, that displays vector of strings
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
/// ```rust
/// # use std::{
/// #     cell::RefCell,
/// #     rc::Rc,
/// # };
/// # use termint::{
/// #     buffer::Buffer,
/// #     enums::Color,
/// #     widgets::{List, ListState, Widget},
/// #     geometry::Rect,
/// # };
/// // Creates list state with selected item on position 1 and scroll offset 0
/// let state = Rc::new(RefCell::new(ListState::selected(0, 1)));
///
/// // Creates list, where selected item has yellow foreground and '*' in
/// // front of it and automatically scrolls to selected item
/// let items = vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6"];
/// let list =
///     List::new(items, state.clone())
///         .selected_style((Color::Yellow))
///         .highlight_symbol("*")
///         .auto_scroll();
///
/// // Renders using the buffer
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 20, 5));
/// list.render(&mut buffer);
/// buffer.render();
/// ```
#[derive(Debug)]
pub struct List {
    items: Vec<String>,
    state: Rc<RefCell<ListState>>,
    auto_scroll: bool,
    style: Style,
    sel_style: Style,
    highlight: String,
    highlight_style: Style,
    scrollbar_fg: Color,
    thumb_fg: Color,
}

/// State of the [`List`] widget
#[derive(Debug)]
pub struct ListState {
    pub offset: usize,
    pub selected: Option<usize>,
}

impl List {
    /// Creates new [`List`] with given items and given state
    pub fn new<T>(items: T, state: Rc<RefCell<ListState>>) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<str>,
    {
        let items =
            items.into_iter().map(|i| i.as_ref().to_string()).collect();

        Self {
            items,
            state,
            auto_scroll: false,
            style: Default::default(),
            sel_style: Default::default(),
            highlight: String::new(),
            highlight_style: Default::default(),
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

    /// Automatically scrolls so the selected item is visible
    pub fn auto_scroll(mut self) -> Self {
        self.auto_scroll = true;
        self
    }

    /// Sets style of the [`List`]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets style of the selected item in the [`List`]
    pub fn selected_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.sel_style = style.into();
        self
    }

    /// Sets highlight symbol of the selected item
    pub fn highlight_symbol<T>(mut self, sel_char: T) -> Self
    where
        T: AsRef<str>,
    {
        self.highlight = sel_char.as_ref().to_string();
        self
    }

    /// Sets style of the highlight symbol
    /// (seperate from the selected item style)
    pub fn highlight_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.highlight_style = style.into();
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

impl Widget for List {
    fn render(&self, buffer: &mut Buffer) {
        if self.auto_scroll {
            self.scroll_offset(buffer.size());
        }

        let mut text_pos =
            Vec2::new(buffer.x() + self.highlight.len(), buffer.y());
        let mut text_size =
            Vec2::new(buffer.width() - self.highlight.len(), buffer.height());

        if !self.fits(buffer.size()) {
            text_size.x -= 1;
            self.render_scrollbar(buffer);
        }

        let selected = self.state.borrow().selected;
        for i in self.state.borrow().offset..self.items.len() {
            let mut span = self.items[i].style(self.style);
            if Some(i) == selected {
                buffer.set_str_styled(
                    &self.highlight,
                    &Vec2::new(buffer.x(), text_pos.y),
                    self.highlight_style,
                );
                span = self.items[i].style(self.sel_style);
            }

            let mut ibuffer =
                buffer.subset(Rect::from_coords(text_pos, text_size));
            let res_pos = span.render_offset(&mut ibuffer, 0, None);
            buffer.merge(ibuffer);

            text_size.y = text_size.y.saturating_sub(res_pos.y - text_pos.y);
            text_pos.y = res_pos.y + 1;

            if buffer.y() + buffer.height() <= text_pos.y {
                break;
            }
            text_size.y = buffer.y() + buffer.height() - text_pos.y;
        }
    }

    fn height(&self, size: &Vec2) -> usize {
        let mut height = 0;
        for i in 0..self.items.len() {
            let span = self.items[i].to_span();
            height += span.height(size);
        }
        height
    }

    fn width(&self, size: &Vec2) -> usize {
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
        let thumb_size = min(
            (buffer.height() as f32 / rat).floor() as usize,
            buffer.height(),
        );
        let thumb_offset = min(
            (self.state.borrow().offset as f32 / rat) as usize,
            buffer.height() - thumb_size,
        );

        let x = (buffer.x() + buffer.width()).saturating_sub(1);
        let mut bar_pos = Vec2::new(x, buffer.y());
        for _ in 0..buffer.height() {
            buffer.set_val('│', &bar_pos);
            buffer.set_fg(self.scrollbar_fg, &bar_pos);
            bar_pos.y += 1;
        }

        bar_pos = Vec2::new(x, buffer.y() + thumb_offset);
        for _ in 0..thumb_size {
            buffer.set_val('┃', &bar_pos);
            buffer.set_fg(self.thumb_fg, &bar_pos);
            bar_pos.y += 1;
        }
    }

    /// Automatically scrolls so the selected item is visible
    fn scroll_offset(&self, size: &Vec2) {
        let Some(selected) = self.state.borrow().selected else {
            return;
        };

        if selected < self.state.borrow().offset {
            self.state.borrow_mut().offset = selected;
            return;
        }

        while !self.is_visible(selected, self.state.borrow().offset, size) {
            self.state.borrow_mut().offset += 1;
        }
    }

    /// Checks if item is visible with given offset
    fn is_visible(&self, item: usize, offset: usize, size: &Vec2) -> bool {
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
    fn fits(&self, size: &Vec2) -> bool {
        self.is_visible(self.items.len() - 1, 0, size)
    }
}

// From implementations
impl From<List> for Box<dyn Widget> {
    fn from(value: List) -> Self {
        Box::new(value)
    }
}

impl From<List> for Element {
    fn from(value: List) -> Self {
        Element::new(value)
    }
}
