use std::{
    cell::RefCell,
    cmp::{max, min},
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use crate::{
    buffer::Buffer,
    enums::Color,
    geometry::Padding,
    prelude::{MouseButton, MouseEvent, Rect, Vec2},
    style::{Style, Styleable},
    term::backend::MouseEventKind,
    text::Text,
    widgets::{Element, EventResult, LayoutNode, Span, ToSpan, Widget},
};

type ListHandler<M> = Box<dyn Fn(usize) -> M>;

/// A scrollable list widget with support for item selection and highlighting.
///
/// The [`List`] widget displays a list of strings, supporting vertical
/// scrolling, item selection and custom highlighting. A scrollbar is
/// automatically hidden when the content fits in the available height.
///
/// # Mouse support
///
/// List supports mouse event handling. In order to enable it, you have to
/// enable mouse capture. You can do that by calling
/// [`Term::with_mouse`](crate::term::Term::with_mouse) on
/// [`Term`](crate::term::Term) struct or
/// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not using
/// the [`Term`](crate::term::Term).
///
/// By default [`List`] automatically handles scrolling. When you scroll up,
/// previous item is selected and when you scroll down, the next item is
/// selected. You can customize it using [`List::on_scroll`] or disable it
/// using [`List::scrollable`].
///
/// You can setup click handlers reporting which item was clicked using
/// [`List::on_click`] and [`List::on_press`].
///
/// # Example
/// ```rust
/// use termint::prelude::*;
/// use std::{cell::RefCell, rc::Rc};
///
/// // Creates list state with offset 0 and with selected item at index 1
/// let state = Rc::new(RefCell::new(ListState::selected(0, 1)));
///
/// // Creates a list with given items.
/// let items = vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6"];
/// let list = List::<()>::new(items, state.clone())
///     // Highlight the selected item in yellow.
///     .selected_style(Color::Yellow)
///     // Add `*` prefix before the selected item.
///     .highlight_symbol("*")
///     // Automatically scroll selected item into view.
///     .auto_scroll();
/// ```
pub struct List<M: 'static = ()> {
    items: Vec<String>,
    state: Rc<RefCell<ListState>>,
    auto_scroll: bool,
    handle_scroll: bool,
    scroll_step: usize,
    style: Style,
    sel_style: Style,
    highlight: String,
    highlight_style: Style,
    force_scrollbar: bool,
    scrollbar_fg: Color,
    thumb_fg: Color,
    handlers: Vec<(MouseButton, ListHandler<M>)>,
    on_scroll: Option<Box<dyn Fn(isize) -> M>>,
}

/// Stores the state of the [`List`] widget.
///
/// It includes:
/// - Scroll offset = based on items, e.g. 5 means 5th item is first visible
/// - Selected = optional ID of the selected item (zero-based)
#[derive(Debug, Default, Hash)]
pub struct ListState {
    /// The index of the first item currently visible in the list.
    pub offset: usize,
    /// The index of the currently selected item (if any).
    pub selected: Option<usize>,
}

impl<M> List<M> {
    /// Creates a new [`List`] with given items and given state.
    ///
    /// The `items` can be any iterable yielding strings (e.g. `Vec<String>` or
    /// `&[&str]`).
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
            scroll_step: 1,
            style: Default::default(),
            sel_style: Default::default(),
            highlight: String::new(),
            highlight_style: Default::default(),
            force_scrollbar: false,
            scrollbar_fg: Color::Default,
            thumb_fg: Color::Default,
            handlers: vec![],
            on_scroll: None,
        }
    }

    /// Sets the currently selected item in the [`List`].
    ///
    /// This directly modifies the inner [`ListState`]. Generally you can
    /// modify the state directly.
    #[must_use]
    pub fn selected<T>(self, current: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.state.borrow_mut().selected = current.into();
        self
    }

    /// Enables automatic scrolling to ensure the selected item is always
    /// visible.
    ///
    /// If enabled, the list will automatically adjust its `offset` during
    /// rendering to keep the `selected` item in the view.
    #[must_use]
    pub fn auto_scroll(mut self) -> Self {
        self.auto_scroll = true;
        self
    }

    /// Enables or disables automatic mouse scroll handling.
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn scrollable(mut self, enabled: bool) -> Self {
        self.handle_scroll = enabled;
        self
    }

    /// Sets the numbers of items to scroll per mouse wheel step.
    ///
    /// It is mainly used in automatic mouse scroll handling, but the step
    /// size also determines the value returned in the Message if custom
    /// scroll handler is used.
    ///
    /// Default is `1`.
    #[must_use]
    pub fn scroll_step(mut self, size: usize) -> Self {
        self.scroll_step = size;
        self
    }

    /// Sets the base [`Style`] of the [`List`].
    ///
    /// The `style` can be any type convertible to [`Style`].
    #[must_use]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the [`Style`] of the selected item in the [`List`].
    ///
    /// The `style` can be any type convertible to [`Style`].
    #[must_use]
    pub fn selected_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.sel_style = style.into();
        self
    }

    /// Sets the prefix string to be displayed before the selected item.
    ///
    /// # Example
    ///
    /// ```rust
    /// use termint::prelude::*;
    /// # fn get_items() -> Vec<String> { vec![] }
    ///
    /// // Selected item will look like: "> Item text".
    /// let list = List::<()>::new(get_items(), Default::default())
    ///     .highlight_symbol("> ");
    ///
    /// // You can also set general style using `Stylize` trait
    /// let list = List::<()>::new(get_items(), Default::default())
    ///     .black()
    ///     .on_white()
    ///     .strike();
    /// ```
    #[must_use]
    pub fn highlight_symbol<T>(mut self, sel_char: T) -> Self
    where
        T: AsRef<str>,
    {
        self.highlight = sel_char.as_ref().to_string();
        self
    }

    /// Sets the [`Style`] of the highlight symbol.
    ///
    /// This allows the prefix to have a different color than the selected
    /// item itself.
    ///
    /// The `style` can be any type convertible to [`Style`].
    #[must_use]
    pub fn highlight_style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.highlight_style = style.into();
        self
    }

    /// Forces scrollbar to be always visible, even if content fits.
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

    /// Sets the foreground color of the scrollbar's thumb (the moving part).
    #[must_use]
    pub fn thumb_fg(mut self, fg: Color) -> Self {
        self.thumb_fg = fg;
        self
    }

    /// Sets the message to return when the left mouse button is clicked.
    ///
    /// The `response` is a closure that receives the index of the clicked
    /// item and returns the corresponding message.
    ///
    /// If a handler for the left mouse button already exists, it will be
    /// replaced.
    ///
    /// This is a convenience wrapper around [`List::on_press`].
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn on_click<F>(self, response: F) -> Self
    where
        F: Fn(usize) -> M + 'static,
    {
        self.on_press(MouseButton::Left, response)
    }

    /// Sets the message to return when the given [`MouseButton`] is clicked.
    ///
    /// The `response` is a closure that receives the index of the clicked
    /// item and returns the corresponding message.
    ///
    /// If a handler for the given mouse button already exists, it will be
    /// replaced.
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    ///
    /// # Example
    ///
    /// ```rust
    /// use termint::prelude::*;
    /// # fn get_items() -> Vec<String> { vec![] }
    ///
    /// let btn = List::new(get_items(), Default::default())
    ///     .on_press(MouseButton::Middle, |i| format!("Clicked {i}!"));
    /// ```
    #[must_use]
    pub fn on_press<F>(mut self, button: MouseButton, response: F) -> Self
    where
        F: Fn(usize) -> M + 'static,
    {
        self.handlers.retain(|(b, _)| *b != button);
        self.handlers.push((button, Box::new(response)));
        self
    }

    /// Sets the message to return when the mouse scroll event occures.
    ///
    /// The `response` is a closure that receives the scroll delta (e.g. -1
    /// when scrolled up) and returns the corresponding message.
    ///
    /// This disables the default on scroll handler, so only the given response
    /// will be used.
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn on_scroll<F>(mut self, response: F) -> Self
    where
        F: Fn(isize) -> M + 'static,
    {
        self.on_scroll = Some(Box::new(response));
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
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        let rect = layout.area;
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

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.items.hash(&mut hasher);
        self.force_scrollbar.hash(&mut hasher);
        self.highlight.hash(&mut hasher);
        self.state.borrow().hash(&mut hasher);

        hasher.finish()
    }

    fn on_event(&self, node: &LayoutNode, e: &MouseEvent) -> EventResult<M> {
        if !node.area.contains_pos(&e.pos) {
            return EventResult::None;
        }

        let mut area = node.area.inner(Padding::left(self.highlight.len()));
        if self.force_scrollbar || !self.fits(area.size()) {
            area.size.x -= 1;
        }

        for i in self.state.borrow().offset..self.items.len() {
            let span: Element<M> = self.items[i].to_span().into();
            let height = span.height(area.size());

            let mut irect = Rect::from_coords(*area.pos(), *area.size());
            irect.size.y = height;
            if !irect.contains_pos(&e.pos) {
                area = area.inner(Padding::top(height));
                continue;
            }

            let m = self.item_event(e, i);
            if !m.is_none() {
                return m;
            }
        }
        self.handle_mouse(e)
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
        if let Some(selected) = self.state.borrow().selected
            && selected + 1 == self.items.len()
        {
            thumb_offset = rect.height() - thumb_size;
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
            true => selected.saturating_sub(delta.unsigned_abs()),
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
        let delta = match &event.kind {
            MouseEventKind::ScrollDown => self.scroll_step as isize,
            MouseEventKind::ScrollUp => -(self.scroll_step as isize),
            _ => return EventResult::None,
        };

        if let Some(handler) = &self.on_scroll {
            return EventResult::Response(handler(delta));
        }

        if self.handle_scroll {
            self.move_selection(delta);
            return EventResult::Consumed;
        }

        EventResult::None
    }
}

impl<M> Styleable for List<M> {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
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
