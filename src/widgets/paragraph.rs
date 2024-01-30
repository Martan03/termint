use crate::enums::bg::Bg;

use super::{grad::Grad, span::Span, widget::Widget};

/// [`Paragraph`] allow to use multiple [`Span`] and [`Grad`] in one Widget,
/// separating them with set separator. Spans and Grads are places after each
/// other, which you can't really achieve with Layout
pub struct Paragraph {
    children: Vec<Box<dyn Widget>>,
    separator: String,
    bg: Bg,
}

impl Paragraph {
    /// Creates new [`Paragraph`]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            separator: " ".to_string(),
            bg: Bg::Default,
        }
    }

    /// Sets [`Paragraph`] separator to given string
    pub fn separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }

    /// Sets [`Paragraph`] backround color
    pub fn bg(mut self, bg: Bg) -> Self {
        self.bg = bg;
        self
    }

    /// Adds [`Span`] to [`Paragraph`]
    pub fn add(&mut self, span: Span) {
        self.children.push(Box::new(span));
    }

    /// Adds [`Grad`] to [`Paragraph`]
    pub fn add_grad(&mut self, grad: Grad) {
        self.children.push(Box::new(grad));
    }
}

impl Widget for Paragraph {
    fn render(
        &self,
        _pos: &crate::geometry::coords::Coords,
        _size: &crate::geometry::coords::Coords,
    ) {
        todo!()
    }

    fn height(&self, _size: &crate::geometry::coords::Coords) -> usize {
        todo!()
    }

    fn width(&self, _size: &crate::geometry::coords::Coords) -> usize {
        todo!()
    }
}
