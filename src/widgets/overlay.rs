use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
    prelude::MouseEvent,
    widgets::{cache::Cache, layout::LayoutNode, widget::EventResult},
};

use super::{Element, Widget};

/// A container widget that stacks its children on top of each other (z-axis).
///
/// [`Overlay`] is useful for creating layered interface, such as modals and
/// popups, or floating elements.
///
/// # Layout behavior
///
/// [`Overlay`] passes the full available area to every child. The first child
/// is rendered at the bottom, and each subsequent child is rendered on top of
/// the previous one.
///
/// # Example
/// ```rust
/// use termint::prelude::*;
///
/// # fn get_content() -> Element { Spacer::new().into() }
/// # fn get_modal() -> Element { Spacer::new().into() }
/// let overlay = Overlay::new(vec![
///     get_content(),
///     // Modal is rendered on top of the content
///     get_modal(),
/// ]);
/// ```
pub struct Overlay<M: 'static = ()> {
    children: Vec<Element<M>>,
}

impl<M> Overlay<M> {
    /// Creates a new [`Overlay`] from a list of children.
    ///
    /// The first widget will be at the bottom, and the last on top.
    ///
    /// The `children` can be any type convertible into an iterator of
    /// [`Element`]s.
    ///
    /// # Example
    ///
    /// ```rust
    /// use termint::prelude::*;
    ///
    /// let overlay = Overlay::<()>::new(vec![
    ///     "Bottom",
    ///     "Middle",
    ///     "Top",
    /// ]);
    /// ```
    #[must_use]
    pub fn new<I>(children: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Element<M>>,
    {
        Self {
            children: children.into_iter().map(|i| i.into()).collect(),
        }
    }

    /// Creates an empty [`Overlay`] with no children.
    #[must_use]
    pub fn empty() -> Self {
        Self { children: vec![] }
    }

    /// Pushes a new widget onto the top of the [`Overlay`] stack.
    ///
    /// The given widget will be rendered last, covering previous widgets.
    ///
    /// The `child` can be any type convertible into [`Element`].
    pub fn push<W>(&mut self, child: W)
    where
        W: Into<Element<M>>,
    {
        self.children.push(child.into());
    }
}

impl<M: Clone + 'static> Widget<M> for Overlay<M> {
    fn render(
        &self,
        buffer: &mut Buffer,
        layout: &LayoutNode,
        cache: &mut Cache,
    ) {
        self.children.iter().enumerate().for_each(|(i, c)| {
            c.render(buffer, &layout.children[i], &mut cache.children[i])
        });
    }

    fn height(&self, size: &Vec2) -> usize {
        self.children
            .iter()
            .map(|c| c.height(size))
            .max()
            .unwrap_or(0)
    }

    fn width(&self, size: &Vec2) -> usize {
        self.children
            .iter()
            .map(|c| c.width(size))
            .max()
            .unwrap_or(0)
    }

    fn children(&self) -> Vec<&Element<M>> {
        self.children.iter().collect()
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

        for (i, child) in self.children.iter().rev().enumerate() {
            let m = child.on_event(area, &mut cache.children[i], event);
            if !m.is_none() {
                return m;
            }
        }
        EventResult::None
    }
}

impl<M: Clone + 'static> From<Overlay<M>> for Element<M> {
    fn from(value: Overlay<M>) -> Self {
        Element::new(value)
    }
}

impl<M: Clone + 'static> From<Overlay<M>> for Box<dyn Widget<M>> {
    fn from(value: Overlay<M>) -> Self {
        Box::new(value)
    }
}
