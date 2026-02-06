use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

use crate::{
    buffer::Buffer,
    enums::Color,
    geometry::{Padding, Rect, Vec2},
    prelude::MouseEvent,
    style::Style,
    term::backend::{MouseButton, MouseEventKind},
    text::Text,
    widgets::{cache::Cache, widget::EventResult, Span},
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
/// # fn example() -> Result<(), termint::Error> {
/// // Creates list state with offset 0 and with selected item at index 1
/// let state = Rc::new(RefCell::new(ListState::selected(0, 1)));
///
/// // Creates a list, highlight the selected item in yellow with '*' prefix,
/// // and automatically scroll to keep the selected item in view
/// let items = vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6"];
/// let list =
///     List::<()>::new(items, state.clone())
///         .selected_style((Color::Yellow))
///         .highlight_symbol("*")
///         .auto_scroll();
///
/// let mut term = Term::default();
/// term.render(list)?;
/// # Ok(())
/// # }
/// ```
pub struct List<M: 'static = ()> {
    items: Vec<String>,
    state: Rc<RefCell<ListState>>,
    auto_scroll: bool,
    handle_scroll: bool,
    scroll_dist: usize,
    style: Style,
    sel_style: Style,
    highlight: String,
    highlight_style: Style,
    force_scrollbar: bool,
    scrollbar_fg: Color,
    thumb_fg: Color,
    handlers: Vec<(MouseButton, Box<dyn Fn(usize) -> M>)>,
}

/// State of the [`List`] widget, including scroll offset and selected index.
#[derive(Debug)]
pub struct ListState {
    pub offset: usize,
    pub selected: Option<usize>,
}

impl<M> List<M> {
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
            handle_scroll: true,
            scroll_dist: 1,
            style: Default::default(),
            sel_style: Default::default(),
            highlight: String::new(),
            highlight_style: Default::default(),
            force_scrollbar: false,
            scrollbar_fg: Color::Default,
            thumb_fg: Color::Default,
            handlers: vec![],
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

    /// Enables or disables automatic mouse scroll handling.
    #[must_use]
    pub fn scrollable(mut self, enabled: bool) -> Self {
        self.handle_scroll = enabled;
        self
    }

    /// Sets the scroll distance used in the automatic mouse scroll handling.
    #[must_use]
    pub fn scroll_distance(mut self, distance: usize) -> Self {
        self.scroll_dist = distance;
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

    /// Forces scrollbar to be always visible. By default the scrollbar hides
    /// when the content doesn't overflow.
    #[must_use]
    pub fn force_scrollbar(mut self) -> Self {
        self.force_scrollbar = true;
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

    /// Sets the response Message of the on click handler.
    ///
    /// This overwrites any already set click response message.
    #[must_use]
    pub fn on_click<F>(self, response: F) -> Self
    where
        F: Fn(usize) -> M + 'static,
    {
        self.on_press(MouseButton::Left, response)
    }

    /// Sets the response Message for the given button click handler.
    ///
    /// This overwrites any already set response message for the given button.
    #[must_use]
    pub fn on_press<F>(mut self, button: MouseButton, response: F) -> Self
    where
        F: Fn(usize) -> M + 'static,
    {
        self.handlers.retain(|(b, _)| *b != button);
        self.handlers.push((button, Box::new(response)));
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

impl<M: Clone + 'static> Widget<M> for List<M> {
    fn render(&self, buffer: &mut Buffer, rect: Rect, _cache: &mut Cache) {
        let mut text_pos =
            Vec2::new(rect.x() + self.highlight.len(), rect.y());
        let mut text_size =
            Vec2::new(rect.width() - self.highlight.len(), rect.height());

        let has_bar = self.force_scrollbar || !self.fits(&text_size);
        if has_bar {
            text_size.x = text_size.x.saturating_sub(1);
        }

        if self.auto_scroll {
            self.scroll_offset(&text_size);
        }

        if has_bar {
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
        self.items
            .iter()
            .map(|i| <Span as Widget<M>>::height(&i.to_span(), size))
            .sum()
    }

    fn width(&self, size: &Vec2) -> usize {
        let mut width = 0;
        let mut height = 0;
        for item in self.items.iter() {
            let span = item.to_span();
            let h = <Span as Widget<M>>::height(&span, size);
            width = max(
                <Span as Widget<M>>::width(&span, &Vec2::new(size.x, h)),
                width,
            );
            height += h;
        }
        width + self.highlight.len() + (height > size.y) as usize
    }

    fn on_event(
        &self,
        area: Rect,
        _cache: &mut Cache,
        event: &MouseEvent,
    ) -> EventResult<M> {
        if !area.contains_pos(&event.pos) {
            return EventResult::None;
        }

        let mut area = area.inner(Padding::left(self.highlight.len()));
        if self.force_scrollbar || !self.fits(area.size()) {
            area.size.x -= 1;
        }

        for i in self.state.borrow().offset..self.items.len() {
            let span: Element<M> = self.items[i].to_span().into();
            let height = span.height(area.size());

            let mut irect = Rect::from_coords(*area.pos(), *area.size());
            irect.size.y = height;
            if !irect.contains_pos(&event.pos) {
                area = area.inner(Padding::top(height));
                continue;
            }

            let m = self.item_event(event, i);
            if !m.is_none() {
                return m;
            }
        }
        self.handle_mouse(event)
    }
}

impl<M: Clone + 'static> List<M> {
    /// Renders [`List`] scrollbar
    fn render_scrollbar(&self, buffer: &mut Buffer, rect: &Rect) {
        let rat = self.items.len() as f32 / rect.height() as f32;
        let thumb_size = max(
            1,
            min((rect.height() as f32 / rat).round() as usize, rect.height()),
        );
        let mut thumb_offset = min(
            (self.state.borrow().offset as f32 / rat) as usize,
            rect.height() - thumb_size,
        );
        if let Some(selected) = self.state.borrow().selected {
            if selected + 1 == self.items.len() {
                thumb_offset = rect.height() - thumb_size;
            };
        }

        let x = (rect.x() + rect.width()).saturating_sub(1);
        let mut bar_pos = Vec2::new(x, rect.y());
        for _ in 0..rect.height() {
            buffer[bar_pos].char('│').fg(self.scrollbar_fg);
            bar_pos.y += 1;
        }

        bar_pos = Vec2::new(x, rect.y() + thumb_offset);
        for _ in 0..thumb_size {
            buffer[bar_pos].char('┃').fg(self.thumb_fg);
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

    fn move_selection(&self, delta: isize) {
        let mut state = self.state.borrow_mut();
        let Some(selected) = state.selected else {
            return;
        };

        let id = match delta < 0 {
            true => selected.saturating_sub(delta.unsigned_abs() as usize),
            false => (selected + delta as usize)
                .min(self.items.len().saturating_sub(1)),
        };
        state.selected = Some(id);
    }

    /// Checks if item is visible with given offset
    fn is_visible(&self, item: usize, offset: usize, size: &Vec2) -> bool {
        let mut height = 0;
        for i in offset..self.items.len() {
            height +=
                <Span as Widget<M>>::height(&self.items[i].to_span(), size);
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

    fn item_event(&self, event: &MouseEvent, id: usize) -> EventResult<M> {
        match &event.kind {
            MouseEventKind::Down(button) => self
                .handlers
                .iter()
                .find(|(b, _)| b == button)
                .map(|(_, m)| EventResult::Response(m(id)))
                .unwrap_or(EventResult::None),
            _ => EventResult::None,
        }
    }

    fn handle_mouse(&self, event: &MouseEvent) -> EventResult<M> {
        if !self.handle_scroll {
            return EventResult::None;
        }

        match &event.kind {
            MouseEventKind::ScrollDown => {
                self.move_selection(self.scroll_dist as isize);
                EventResult::Consumed
            }
            MouseEventKind::ScrollUp => {
                self.move_selection(-(self.scroll_dist as isize));
                EventResult::Consumed
            }
            _ => EventResult::None,
        }
    }
}

// From implementations
impl<M: Clone + 'static> From<List<M>> for Box<dyn Widget<M>> {
    fn from(value: List<M>) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<List<M>> for Element<M> {
    fn from(value: List<M>) -> Self {
        Element::new(value)
    }
}
