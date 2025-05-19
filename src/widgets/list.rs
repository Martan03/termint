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
    text::Text,
};

use super::{span::ToSpan, widget::Widget, Element};

/// A scrollable list widget with suuport for item selection and highlighting.
///
/// The [`List`] widgets displays a list of strings with optional selection
/// highlighting and vertical scrollbar. The scrollbar is only shown if needed.
///
/// # Features
/// - **Scrollbar** (doesn't show when not necessary):
///     - Custom scrollbar foreground color
///     - Custom scrollbar thumb color
/// - **Selected item styling**:
///     - Highlight symbol
///     - Custom style
///
/// # Example
/// ```rust
/// # use std::{cell::RefCell, rc::Rc};
/// # use termint::{
/// #     term::Term,
/// #     enums::Color,
/// #     widgets::{List, ListState, Widget},
/// # };
/// # fn example() -> Result<(), &'static str> {
/// // Creates list state with offset 0 and with selected item at index 1
/// let state = Rc::new(RefCell::new(ListState::selected(0, 1)));
///
/// // Creates a list, highlight the selected item in yellow with '*' prefix,
/// // and automatically scroll to keep the selected item in view
/// let items = vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6"];
/// let list =
///     List::new(items, state.clone())
///         .selected_style((Color::Yellow))
///         .highlight_symbol("*")
///         .auto_scroll();
///
/// let mut term = Term::new();
/// term.render(list)?;
/// # Ok(())
/// # }
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

/// State of the [`List`] widget, including scroll offset and selected index.
#[derive(Debug)]
pub struct ListState {
    pub offset: usize,
    pub selected: Option<usize>,
}

impl List {
    /// Creates a new [`List`] with given items and given state.
    #[must_use]
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

    /// Sets the currently selected item in the [`List`].
    #[must_use]
    pub fn selected<T>(self, current: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.state.borrow_mut().selected = current.into();
        self
    }

    /// Enables automatic scrolling to ensure the selected item is visible.
    #[must_use]
    pub fn auto_scroll(mut self) -> Self {
        self.auto_scroll = true;
        self
    }

    /// Sets the base [`Style`] of the [`List`].
    #[must_use]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the [`Style`] of the selected item in the [`List`].
    #[must_use]
    pub fn selected_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.sel_style = style.into();
        self
    }

    /// Sets the highlight symbol of the selected item.
    ///
    /// This symbol appears before the selected item and can be set, for
    /// example, to `"*"`, which would result to selected item being shown as
    /// `* Item`.
    #[must_use]
    pub fn highlight_symbol<T>(mut self, sel_char: T) -> Self
    where
        T: AsRef<str>,
    {
        self.highlight = sel_char.as_ref().to_string();
        self
    }

    /// Sets the [`Style`] of the highlight symbol
    /// (separate from the selected item style)
    #[must_use]
    pub fn highlight_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.highlight_style = style.into();
        self
    }

    /// Sets the foreground color of the scrollbar.
    #[must_use]
    pub fn scrollbar_fg(mut self, fg: Color) -> Self {
        self.scrollbar_fg = fg;
        self
    }

    /// Sets the foreground color of the scrollbar's thumb (draggable part).
    #[must_use]
    pub fn thumb_fg(mut self, fg: Color) -> Self {
        self.thumb_fg = fg;
        self
    }
}

impl ListState {
    /// Creates a new [`ListState`] with the given scroll offset and no
    /// selected item.
    #[must_use]
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            selected: None,
        }
    }

    /// Creates a new [`ListState`] with given scroll offset and selected item.
    #[must_use]
    pub fn selected(offset: usize, selected: usize) -> Self {
        Self {
            offset,
            selected: Some(selected),
        }
    }
}

impl Widget for List {
    fn render(&self, buffer: &mut Buffer, rect: Rect) {
        if self.auto_scroll {
            self.scroll_offset(rect.size());
        }

        let mut text_pos =
            Vec2::new(rect.x() + self.highlight.len(), rect.y());
        let mut text_size =
            Vec2::new(rect.width() - self.highlight.len(), rect.height());

        if !self.fits(rect.size()) {
            text_size.x -= 1;
            self.render_scrollbar(buffer, &rect);
        }

        let selected = self.state.borrow().selected;
        for i in self.state.borrow().offset..self.items.len() {
            let mut span = self.items[i].style(self.style);
            if Some(i) == selected {
                buffer.set_str_styled(
                    &self.highlight,
                    &Vec2::new(rect.x(), text_pos.y),
                    self.highlight_style,
                );
                span = self.items[i].style(self.sel_style);
            }

            let irect = Rect::from_coords(text_pos, text_size);
            let res_pos = span.render_offset(buffer, irect, 0, None);

            text_size.y = text_size.y.saturating_sub(res_pos.y - text_pos.y);
            text_pos.y = res_pos.y + 1;

            if rect.y() + rect.height() <= text_pos.y {
                break;
            }
            text_size.y = rect.y() + rect.height() - text_pos.y;
        }
    }

    fn height(&self, size: &Vec2) -> usize {
        self.items.iter().map(|i| i.to_span().height(size)).sum()
    }

    fn width(&self, size: &Vec2) -> usize {
        let mut width = 0;
        let mut height = 0;
        for item in self.items.iter() {
            let span = item.to_span();
            let h = span.height(size);
            width = max(span.width(&Vec2::new(size.x, h)), width);
            height += h;
        }
        width + self.highlight.len() + (height > size.y) as usize
    }
}

impl List {
    /// Renders [`List`] scrollbar
    fn render_scrollbar(&self, buffer: &mut Buffer, rect: &Rect) {
        let rat = self.items.len() as f32 / rect.height() as f32;
        let thumb_size =
            min((rect.height() as f32 / rat).floor() as usize, rect.height());
        let thumb_offset = min(
            (self.state.borrow().offset as f32 / rat) as usize,
            rect.height() - thumb_size,
        );

        let x = (rect.x() + rect.width()).saturating_sub(1);
        let mut bar_pos = Vec2::new(x, rect.y());
        for _ in 0..rect.height() {
            buffer.set_val('│', &bar_pos);
            buffer.set_fg(self.scrollbar_fg, &bar_pos);
            bar_pos.y += 1;
        }

        bar_pos = Vec2::new(x, rect.y() + thumb_offset);
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
