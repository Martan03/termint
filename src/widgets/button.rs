use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{
    buffer::Buffer,
    geometry::Padding,
    prelude::{MouseEvent, Rect, Vec2},
    style::Style,
    term::backend::{MouseButton, MouseEventKind},
    widgets::{
        cache::Cache,
        layout::{self, Node},
        widget::EventResult,
        Element, Spacer, Widget,
    },
};

/// A clickable wrapper widget that triggers a message when clicked.
///
/// In order to make the [`Button`] trigger the message, you have to enable
/// mouse capture. You can do that by calling
/// [`Term::with_mouse`](crate::term::Term::with_mouse) on
/// [`Term`](crate::term::Term) struct or
/// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not using
/// the [`Term`](crate::term::Term).
///
/// # Example
///
/// ```rust
/// use termint::prelude::*;
///
/// #[derive(Clone)]
/// enum Msg { Submit, Cancel }
///
/// let submit_btn = Button::new("Submit")
///     // Makes the button green with black text
///     .style((Color::Black, Color::Green))
///     // Adds 1 char vertical and 2 char horizontal padding
///     .padding((1, 2))
///     // Sets the left click trigger Message
///     .on_click(Msg::Submit);
///
/// let cancel_btn = Button::new("Cancel")
///     .style((Color::Black, Color::Red))
///     .padding((1, 2))
///     // Sets the right click trigger Message
///     .on_press(MouseButton::Right, Msg::Cancel);
/// ```
pub struct Button<M: 'static> {
    child: Element<M>,
    padding: Padding,
    style: Style,
    handlers: Vec<(MouseButton, M)>,
}

impl<M> Button<M> {
    /// Creates a new [`Button`] wrapping the given child widget.
    ///
    /// The `child` can be any widget convertible into [`Element`]. You can
    /// supply `&str` or `String` for example.
    #[must_use]
    pub fn new<T>(child: T) -> Self
    where
        T: Into<Element<M>>,
    {
        Self {
            child: child.into(),
            padding: Default::default(),
            style: Default::default(),
            handlers: vec![],
        }
    }

    /// Sets the internal [`Padding`] between the [`Button`]'s edges and its
    /// child.
    ///
    /// The `padding` can be any type convertible into [`Padding`], such as
    /// `usize` (uniform), `(usize, usize)` (vertical, horizontal). You can
    /// read more in the [`Padding`] documentation.
    #[must_use]
    pub fn padding<P>(mut self, padding: P) -> Self
    where
        P: Into<Padding>,
    {
        self.padding = padding.into();
        self
    }

    /// Sets the base [`Style`] of the [`Button`].
    ///
    /// The `style` can be any type convertible to [`Style`].
    #[must_use]
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the message to return when the left mouse button is clicked.
    ///
    /// If a handler for the left mouse button already exists, it will be
    /// replaced.
    ///
    /// This is a convenience wrapper around [`Button::on_press`].
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn on_click(self, response: M) -> Self {
        self.on_press(MouseButton::Left, response)
    }

    /// Sets the message to return when the given [`MouseButton`] is clicked.
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
    ///
    /// let btn = Button::new("Middle click me!")
    ///     .on_press(MouseButton::Middle, "Clicked!");
    /// ```
    #[must_use]
    pub fn on_press(mut self, button: MouseButton, response: M) -> Self {
        self.handlers.retain(|(b, _)| *b != button);
        self.handlers.push((button, response));
        self
    }
}

impl<M: Clone + 'static> Button<M> {
    /// Creates a new [`Button`] containing a [`Spacer`].
    ///
    /// This is useful if you want clickable area with no content.
    #[must_use]
    pub fn empty() -> Self {
        Self::new(Spacer::new())
    }
}

impl<M: Clone + 'static> Widget<M> for Button<M> {
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        buffer.set_area_style(self.style, rect);
        self.child.render(
            buffer,
            rect.inner(self.padding),
            &mut cache.children[0],
        );
    }

    fn height(&self, size: &Vec2) -> usize {
        let size = Vec2::new(
            size.x.saturating_sub(self.padding.get_horizontal()),
            size.y.saturating_sub(self.padding.get_vertical()),
        );
        self.child.height(&size) + self.padding.get_vertical()
    }

    fn width(&self, size: &Vec2) -> usize {
        let size = Vec2::new(
            size.x.saturating_sub(self.padding.get_horizontal()),
            size.y.saturating_sub(self.padding.get_vertical()),
        );
        self.child.width(&size) + self.padding.get_horizontal()
    }

    fn children(&self) -> Vec<&Element<M>> {
        vec![&self.child]
    }

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.padding.hash(&mut hasher);
        hasher.finish()
    }

    fn layout(&self, node: &mut Node, area: Rect) {
        layout::padded(node, area, self.padding, |n, a| {
            self.child.layout(&mut n.children[0], a)
        });
    }

    fn on_event(
        &self,
        area: Rect,
        cache: &mut Cache,
        event: &MouseEvent,
    ) -> EventResult<M> {
        if !area.contains_pos(&event.pos) {
            return EventResult::None;
        }

        let cr = area.inner(self.padding);
        self.child
            .on_event(cr, &mut cache.children[0], event)
            .or_else(|| self.handle_click(event))
    }
}

impl<M: Clone> Button<M> {
    fn handle_click(&self, event: &MouseEvent) -> EventResult<M> {
        match &event.kind {
            MouseEventKind::Down(button) => self
                .handlers
                .iter()
                .find(|(b, _)| b == button)
                .map(|(_, m)| EventResult::Response(m.clone()))
                .unwrap_or(EventResult::None),
            _ => EventResult::None,
        }
    }
}

impl<M: Clone + 'static> From<Button<M>> for Box<dyn Widget<M>> {
    fn from(value: Button<M>) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Button<M>> for Element<M> {
    fn from(value: Button<M>) -> Self {
        Element::new(value)
    }
}
