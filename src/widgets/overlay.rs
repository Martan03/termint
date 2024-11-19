use crate::{buffer::Buffer, geometry::Vec2};

use super::{Element, Widget};

/// Stacks children in layers, first child is at the bottom, last on top
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     geometry::Rect,
/// #     widgets::{Element, Widget, Spacer, Overlay}
/// # };
/// # fn get_bottom_child() -> Element { Spacer::new().into() }
/// # fn get_top_child() -> Element { Spacer::new().into() }
/// let overlay = Overlay::new(vec![
///     get_bottom_child(),
///     get_top_child(),
/// ]);
///
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 20, 10));
/// overlay.render(&mut buffer);
/// buffer.render();
/// ```
pub struct Overlay {
    children: Vec<Element>,
}

impl Overlay {
    /// Creates new [`Overlay`] with given children
    pub fn new(children: Vec<Element>) -> Self {
        Self {
            children: children.into_iter().map(|c| c.into()).collect(),
        }
    }

    /// Pushes child to the [`Overlay`]
    pub fn push<W>(&mut self, child: W)
    where
        W: Into<Element>,
    {
        self.children.push(child.into());
    }
}

impl Widget for Overlay {
    fn render(&self, buffer: &mut Buffer) {
        self.children.iter().for_each(|c| c.render(buffer));
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
