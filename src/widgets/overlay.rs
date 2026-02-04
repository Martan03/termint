use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
    prelude::MouseEvent,
    widgets::cache::Cache,
};

use super::{Element, Widget};

/// A widget that stacks its children in layers, from bottom to top.
///
/// The first child is rendered at the bottom, and each subsequent child is
/// rendered on top of the previous one, making [`Overlay`] useful for building
/// layered TUIs such as modal.
///
/// # Example
/// ```rust
/// # use termint::{
/// #     term::Term,
/// #     geometry::Rect,
/// #     widgets::{Element, Widget, Spacer, Overlay}
/// # };
/// # fn get_bottom_child() -> Element { Spacer::new().into() }
/// # fn get_top_child() -> Element { Spacer::new().into() }
/// # fn example() -> Result<(), termint::Error> {
/// let overlay = Overlay::new(vec![
///     get_bottom_child(),
///     get_top_child(),
/// ]);
///
/// let mut term = Term::default();
/// term.render(overlay)?;
/// # Ok(())
/// # }
/// ```
///
/// In this example, the second child (`get_top_child()`) is rendered on top of
/// the first.
pub struct Overlay<M: 'static = ()> {
    children: Vec<Element<M>>,
}

impl<M> Overlay<M> {
    /// Creates a new [`Overlay`] from a list of child widgets.
    ///
    /// The first widget will be at the bottom, and the last on top.
    #[must_use]
    pub fn new(children: Vec<Element<M>>) -> Self {
        Self { children }
    }

    /// Creates an empty [`Overlay`] with no children.
    #[must_use]
    pub fn empty() -> Self {
        Self { children: vec![] }
    }

    /// Adds a child to the [`Overlay`], playing it on top of the existing
    /// children.
    pub fn push<W>(&mut self, child: W)
    where
        W: Into<Element<M>>,
    {
        self.children.push(child.into());
    }
}

impl<M> Widget<M> for Overlay<M> {
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        self.children
            .iter()
            .enumerate()
            .for_each(|(i, c)| c.render(buffer, rect, &mut cache.children[i]));
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
    ) -> Option<M> {
        for child in self.children.iter().rev() {
            if let Some(message) = child.on_event(area, cache, event) {
                return Some(message);
            }
        }
        None
    }
}

impl<M> From<Overlay<M>> for Element<M> {
    fn from(value: Overlay<M>) -> Self {
        Element::new(value)
    }
}

impl<M> From<Overlay<M>> for Box<dyn Widget<M>> {
    fn from(value: Overlay<M>) -> Self {
        Box::new(value)
    }
}
