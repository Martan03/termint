use std::{cell::Cell, cmp::min, rc::Rc};

use crate::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
};

use super::{Element, Scrollbar, ScrollbarState, Widget};

/// Wraps widget and allows overflown content to be accessed by scrolling
///
/// ## Example usage:
/// ```rust
/// # use std::{cell::Cell, rc::Rc};
/// # use termint::{
/// #     buffer::Buffer,
/// #     geometry::Rect,
/// #     widgets::{ToSpan, Scrollable, Widget, ScrollbarState}
/// # };
/// // Widget to wrap scrollable around
/// let span = "Long text that cannot fit so scrolling is needed".to_span();
///
/// // Scrollable state containing offset
/// let state = Rc::new(Cell::new(ScrollbarState::new(0)));
///
/// // Creates scrollable widget with vertical scrolling
/// let scrollable = Scrollable::vertical(span, state);
///
/// // Renders using the buffer
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 9, 5));
/// scrollable.render(&mut buffer);
/// buffer.render();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Scrollable<W = Element> {
    horizontal: Option<Scrollbar>,
    vertical: Option<Scrollbar>,
    child: W,
}

impl<W> Scrollable<W>
where
    W: Widget,
{
    /// Creates new vertical [`Scrollable`] with given widget as its content
    pub fn vertical(child: W, state: Rc<Cell<ScrollbarState>>) -> Self {
        Self {
            vertical: Some(Scrollbar::vertical(state.clone())),
            horizontal: None,
            child,
        }
    }

    /// Creates new horizontal [`Scrollable`] with given widget as its content
    pub fn horizontal(child: W, state: Rc<Cell<ScrollbarState>>) -> Self {
        Self {
            vertical: None,
            horizontal: Some(Scrollbar::horizontal(state)),
            child,
        }
    }

    /// Creates new [`Scrollable`] scrolling in both directions with given
    /// widget as its content
    pub fn both(
        child: W,
        ver_state: Rc<Cell<ScrollbarState>>,
        hor_state: Rc<Cell<ScrollbarState>>,
    ) -> Self {
        Self {
            vertical: Some(Scrollbar::vertical(ver_state)),
            horizontal: Some(Scrollbar::horizontal(hor_state)),
            child,
        }
    }
}

impl<W> Widget for Scrollable<W>
where
    W: Widget,
{
    fn render(&self, buffer: &mut Buffer, rect: Rect) {
        match (self.vertical.as_ref(), self.horizontal.as_ref()) {
            (None, None) => self.child.render(buffer, rect),
            (None, Some(hor)) => self.hor_render(buffer, &rect, hor),
            (Some(ver), None) => self.ver_render(buffer, &rect, ver),
            (Some(ver), Some(hor)) => {
                self.both_render(buffer, &rect, ver, hor)
            }
        }
    }

    /// TODO both direction scrolling not correct
    fn height(&self, size: &Vec2) -> usize {
        match (self.vertical.is_some(), self.horizontal.is_some()) {
            (true, true) => self.child.height(&Vec2::new(
                size.x.saturating_sub(1),
                size.y.saturating_sub(1),
            )),
            (true, false) => min(
                size.y,
                self.child
                    .height(&Vec2::new(size.x.saturating_sub(1), size.y))
                    + 1,
            ),
            (false, true) => {
                self.child
                    .height(&Vec2::new(size.x, size.y.saturating_sub(1)))
                    + 1
            }
            (false, false) => self.child.height(size),
        }
    }

    /// TODO both direction scrolling not correct
    fn width(&self, size: &Vec2) -> usize {
        match (self.vertical.is_some(), self.horizontal.is_some()) {
            (true, true) => self.child.width(&Vec2::new(
                size.x.saturating_sub(1),
                size.y.saturating_sub(1),
            )),
            (true, false) => {
                self.child
                    .width(&Vec2::new(size.x.saturating_sub(1), size.y))
                    + 1
            }
            (false, true) => min(
                size.x,
                self.child
                    .width(&Vec2::new(size.x, size.y.saturating_sub(1)))
                    + 1,
            ),
            (false, false) => self.child.width(size),
        }
    }
}

impl<W> Scrollable<W>
where
    W: Widget,
{
    /// Renders vertical scrollable
    fn ver_render(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        vertical: &Scrollbar,
    ) {
        let mut size =
            Vec2::new(rect.width().saturating_sub(1), rect.height());
        size.y = self.child.height(&size);

        let srect = Rect::new(rect.right(), rect.y(), 1, rect.height());
        Self::scrollbar(buffer, vertical, srect, size.y);

        let crect = Rect::new(
            rect.x(),
            rect.y() + vertical.get_state().offset,
            size.x,
            rect.height(),
        );
        self.render_content(buffer, size, crect);
    }

    /// Renders horizontal scrollable
    fn hor_render(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        horizontal: &Scrollbar,
    ) {
        let mut size =
            Vec2::new(rect.width(), rect.height().saturating_sub(1));
        size.x = self.child.width(&size);

        let srect = Rect::new(rect.x(), rect.bottom(), rect.width(), 1);
        Self::scrollbar(buffer, horizontal, srect, size.x);

        let crect = Rect::new(
            rect.x() + horizontal.get_state().offset,
            rect.y(),
            rect.width(),
            size.y,
        );
        self.render_content(buffer, size, crect);
    }

    /// Renders the both directions scrollable
    fn both_render(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        vertical: &Scrollbar,
        horizontal: &Scrollbar,
    ) {
        let mut size = rect.size().saturating_sub((1, 1));
        size.y = self.child.height(&size);
        size.x = self.child.width(&size);

        let mut vis = rect.height().saturating_sub(1);
        let mut rect = Rect::new(rect.right(), rect.y(), 1, vis);
        Self::scrollbar(buffer, vertical, rect, size.y);

        vis = rect.width().saturating_sub(1);
        rect = Rect::new(rect.x(), rect.bottom(), vis, 1);
        Self::scrollbar(buffer, horizontal, rect, size.x);

        let rect = Rect::new(
            rect.x() + horizontal.get_state().offset,
            rect.y() + vertical.get_state().offset,
            rect.width().saturating_sub(1),
            rect.height().saturating_sub(1),
        );
        self.render_content(buffer, size, rect);
    }

    /// Renders the scrollable content
    fn render_content(&self, buffer: &mut Buffer, size: Vec2, mut rect: Rect) {
        let crect = Rect::from_coords(*buffer.pos(), size);
        let mut cbuffer = Buffer::empty(crect);
        self.child.render(&mut cbuffer, crect);

        rect = rect.intersection(cbuffer.rect());
        let mut cutout = cbuffer.subset(rect);
        cutout.move_to(*rect.pos());
        buffer.merge(cutout);
    }

    /// Renders the scrollbar
    fn scrollbar(
        buffer: &mut Buffer,
        scroll: &Scrollbar,
        rect: Rect,
        size: usize,
    ) {
        scroll.content_len(size);
        scroll.render(buffer, rect);
    }
}

impl<W> From<Scrollable<W>> for Box<dyn Widget>
where
    W: Widget + 'static,
{
    fn from(value: Scrollable<W>) -> Self {
        Box::new(value)
    }
}

impl<W> From<Scrollable<W>> for Element
where
    W: Widget + 'static,
{
    fn from(value: Scrollable<W>) -> Self {
        Element::new(value)
    }
}
