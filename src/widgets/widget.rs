use std::{
    any::{Any, TypeId},
    fmt,
};

use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
    prelude::MouseEvent,
    widgets::cache::Cache,
};

/// The result of processing a mouse event within a widget.
///
/// This enum allows event propagation in the widget tree. It tells the parent
/// widget whether an event was already handled or not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventResult<M> {
    /// The event was not handled by the widget.
    ///
    /// The parent widget should pass the event to the next child, try to
    /// handle itself or propagate [`EventResult::None`] to indicate unhandled
    /// event.
    None,
    /// This event was handled internally by the widget.
    ///
    /// This signifies that the widget's state has changed (such as list
    /// scrolled) and the UI should be updated. No message is generated.
    Consumed,
    /// The events was handled and produced a message.
    ///
    /// This message will bubble up to the [`crate::term::Application::update`]
    /// method.
    Response(M),
}

impl<M> EventResult<M> {
    /// Returns `true` if the result is [`EventResult::None`].
    pub fn is_none(&self) -> bool {
        matches!(self, EventResult::None)
    }

    /// Returns either this [`EventResult`] or [`EventResult`] given by the
    /// closure if this result is [`EventResult::None`].
    pub fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        if self.is_none() {
            f()
        } else {
            self
        }
    }
}

/// Trait implemented by all the widgets.
///
/// A widget is a visual component that can render itself to a [`Buffer`] and
/// report its size requirements for layout purposes.
///
/// Use [`Element`] to store and manipulate widgets in a uniform way.
///
/// Users will use [`Widget`] trait directly only when implementing custom
/// widget, otherwise they will use built-in widgets like [`Span`], [`List`]
/// and so on.
pub trait Widget<Message: Clone + 'static = ()>: Any {
    /// Renders the widget into the given [`Buffer`] within the provided
    /// [`Rect`] bounds.
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache);

    /// Returns the height of the [`Widget`] based on the width of the given
    /// size.
    fn height(&self, size: &Vec2) -> usize;

    /// Returns the width of the [`Widget`] based on the height of the given
    /// size.
    fn width(&self, size: &Vec2) -> usize;

    /// Gets widget's children
    fn children(&self) -> Vec<&Element<Message>> {
        vec![]
    }

    /// Handles the mouse event, returns None if outside of this widget or
    /// no event handler set, otherwise returns corresponding Message.
    fn on_event(
        &self,
        _area: Rect,
        _cache: &mut Cache,
        _event: &MouseEvent,
    ) -> EventResult<Message> {
        EventResult::None
    }
}

impl<M> dyn Widget<M> {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<M> fmt::Debug for dyn Widget<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Converted widget")
    }
}

/// A container for any widget implementing the [`Widget`] trait.
///
/// This is the primary type used to store and manipulate widgets in layout
/// trees. `Element` wraps a widget in a dynamic trait object.
///
/// Use [`Element::new`] to convert a widget into an `Element`.
#[derive(Debug)]
pub struct Element<Message: 'static = ()> {
    pub type_id: TypeId,
    pub widget: Box<dyn Widget<Message>>,
}

impl<Message: Clone + 'static> Element<Message> {
    /// Creates a new [`Element`] from a given widget.
    ///
    /// This is commonly used to wrap widgets when composing layouts.
    ///
    /// # Example
    /// ```
    /// # use termint::widgets::{Span, Element};
    /// let span = Span::new("Hello");
    /// let element = Element::<()>::new(span);
    /// ```
    pub fn new<W>(widget: W) -> Self
    where
        W: Widget<Message> + 'static,
    {
        Self {
            type_id: TypeId::of::<W>(),
            widget: Box::new(widget),
        }
    }

    pub fn map<W: Widget<Message> + 'static, F: FnOnce(W) -> W>(
        self,
        f: F,
    ) -> Self {
        let Element { type_id: _, widget } = self;
        let boxed_any: Box<dyn Any> = widget;

        match boxed_any.downcast::<W>() {
            Ok(bx) => {
                let new = f(*bx);
                Self {
                    type_id: TypeId::of::<W>(),
                    widget: Box::new(new),
                }
            }
            Err(orig) => {
                let widget: Box<dyn Widget<Message>> = *orig
                    .downcast::<Box<dyn Widget<Message>>>()
                    .expect("Original type must be Box<dyn Widget>");
                let type_id = widget.as_ref().type_id();

                Self { type_id, widget }
            }
        }
    }

    /// Downcasts widget stored in the [`Element`] to given type, returns
    /// optional reference to the downcast widget on success.
    pub fn downcast_ref<W: Widget<Message>>(&self) -> Option<&W> {
        self.widget.as_ref().as_any().downcast_ref::<W>()
    }

    /// Downcasts widget stored in the [`Element`] to given type, returns
    /// optional mutable reference to the downcast widget on success.
    pub fn downcast_mut<W: Widget<Message>>(&mut self) -> Option<&mut W> {
        self.widget.as_mut().as_any_mut().downcast_mut::<W>()
    }
}

impl<Message: Clone + 'static> Widget<Message> for Element<Message> {
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        self.widget.render(buffer, rect, cache)
    }

    fn height(&self, size: &Vec2) -> usize {
        self.widget.height(size)
    }

    fn width(&self, size: &Vec2) -> usize {
        self.widget.width(size)
    }

    fn children(&self) -> Vec<&Element<Message>> {
        self.widget.children()
    }

    fn on_event(
        &self,
        area: Rect,
        cache: &mut Cache,
        event: &MouseEvent,
    ) -> EventResult<Message> {
        self.widget.on_event(area, cache, event)
    }
}
