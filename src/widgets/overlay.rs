use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
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
/// # fn example() -> Result<(), &'static str> {
/// let overlay = Overlay::new(vec![
///     get_bottom_child(),
///     get_top_child(),
/// ]);
///
/// let mut term = Term::new();
/// term.render(overlay)?;
/// # Ok(())
/// # }
/// ```
///
/// In this example, the second child (`get_top_child()`) is rendered on top of
/// the first.
pub struct Overlay {
    children: Vec<Element>,
}

impl Overlay {
    /// Creates a new [`Overlay`] from a list of child widgets.
    ///
    /// The first widget will be at the bottom, and the last on top.
    #[must_use]
    pub fn new(children: Vec<Element>) -> Self {
        Self {
            children: children.into_iter().map(|c| c.into()).collect(),
        }
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
        W: Into<Element>,
    {
        self.children.push(child.into());
    }
}

impl Widget for Overlay {
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

    fn children(&self) -> Vec<&Element> {
        self.children.iter().collect()
    }
}

impl From<Overlay> for Element {
    fn from(value: Overlay) -> Self {
        Element::new(value)
    }
}

impl From<Overlay> for Box<dyn Widget> {
    fn from(value: Overlay) -> Self {
        Box::new(value)
    }
}
