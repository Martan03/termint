use std::{
    cell::Cell,
    cmp::min,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
    rc::Rc,
    usize,
};

use crate::{
    buffer::Buffer,
    geometry::{Direction, Rect, Vec2},
    prelude::{KeyModifiers, MouseEvent},
    term::backend::MouseEventKind,
    widgets::{cache::Cache, layout::LayoutNode, widget::EventResult},
};

use super::{Element, Scrollbar, ScrollbarState, Widget};

/// A wrapper widget that adds scrollability to its child when content
/// overflows.
///
/// It supports vertical, horizontal or bidirectional scrolling. Scrolling
/// requires a [`ScrollbarState`] to track the offset. For single axis, you
/// can construct [`Scrollable`] using [`Scrollable::new`],
/// [`Scrollable::vertical`] or [`Scrollable::horizontal`]. For bidirectional
/// scrolling, use [`Scrollable::both`] (requires two state).
///
/// # Mouse support
///
/// Scrollable supports mouse event handling. In order to enable it, you have
/// to enable mouse capture. You can do that by calling
/// [`Term::with_mouse`](crate::term::Term::with_mouse) on
/// [`Term`](crate::term::Term) struct or
/// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not using
/// the [`Term`](crate::term::Term).
///
/// By default [`Scrollable`] automatically handles scrolling. You can
/// customize it using [`Scrollable::on_scroll`] (vertical scrolling) and
/// [`Scrollable::on_scroll_horizontal] (horizontal scrolling), or disable it
/// using [`Scrollable::scrollable`].
///
/// # Example
/// ```rust
/// use termint::{prelude::*, widgets::{Scrollable, ScrollbarState}};
/// use std::{cell::Cell, rc::Rc};
///
/// // Content that may overflow
/// let span = "Long text that cannot fit so scrolling is needed".to_span();
///
/// // Shared scrollbar state for managing scroll offset
/// let state = Rc::new(Cell::new(ScrollbarState::new(0)));
///
/// // Creates vertical scrollable widget
/// let scrollable: Scrollable<(), Span> = Scrollable::vertical(span, state);
/// ```
pub struct Scrollable<M: 'static = (), W = Element<M>> {
    horizontal: Option<Element<M>>,
    hor_state: Option<Rc<Cell<ScrollbarState>>>,
    vertical: Option<Element<M>>,
    ver_state: Option<Rc<Cell<ScrollbarState>>>,
    child: Element<M>,
    handle_scroll: bool,
    scroll_dist: Vec2,
    on_scroll_ver: Option<Box<dyn Fn(isize) -> M>>,
    on_scroll_hor: Option<Box<dyn Fn(isize) -> M>>,
    child_type: PhantomData<W>,
}

impl<M, W> Scrollable<M, W>
where
    M: Clone + 'static,
{
    /// Creates a [`Scrollable`] that scrolls in a single direction.
    ///
    /// The `child` can be any type convertible into [`Element`].
    ///
    /// To enable scrolling in both directions, use [`Scrollable::both`].
    pub fn new<T>(
        child: T,
        state: Rc<Cell<ScrollbarState>>,
        dir: Direction,
    ) -> Self
    where
        T: Into<Element<M>>,
    {
        match dir {
            Direction::Vertical => Self::vertical(child, state),
            Direction::Horizontal => Self::horizontal(child, state),
        }
    }

    /// Creates a vertically scrolling [`Scrollable`].
    ///
    /// The `child` can be any type convertible into [`Element`].
    pub fn vertical<T>(child: T, state: Rc<Cell<ScrollbarState>>) -> Self
    where
        T: Into<Element<M>>,
    {
        Self {
            vertical: Some(Scrollbar::vertical(state.clone()).into()),
            ver_state: Some(state),
            horizontal: None,
            hor_state: None,
            child: child.into(),
            handle_scroll: true,
            scroll_dist: Vec2::new(1, 1),
            on_scroll_ver: None,
            on_scroll_hor: None,
            child_type: PhantomData,
        }
    }

    /// Creates a horizontally scrolling [`Scrollable`].
    ///
    /// The `child` can be any type convertible into [`Element`].
    pub fn horizontal<T>(child: T, state: Rc<Cell<ScrollbarState>>) -> Self
    where
        T: Into<Element<M>>,
    {
        Self {
            vertical: None,
            ver_state: None,
            horizontal: Some(Scrollbar::horizontal(state.clone()).into()),
            hor_state: Some(state),
            child: child.into(),
            handle_scroll: true,
            scroll_dist: Vec2::new(1, 1),
            on_scroll_ver: None,
            on_scroll_hor: None,
            child_type: PhantomData,
        }
    }

    /// Creates a [`Scrollable`] that supports both vertical and horizontal
    /// scrolling.
    ///
    /// The `child` can be any type convertible into [`Element`].
    pub fn both<T>(
        child: T,
        ver_state: Rc<Cell<ScrollbarState>>,
        hor_state: Rc<Cell<ScrollbarState>>,
    ) -> Self
    where
        T: Into<Element<M>>,
    {
        Self {
            vertical: Some(Scrollbar::vertical(ver_state.clone()).into()),
            ver_state: Some(ver_state),
            horizontal: Some(Scrollbar::horizontal(hor_state.clone()).into()),
            hor_state: Some(hor_state),
            child: child.into(),
            handle_scroll: true,
            scroll_dist: Vec2::new(1, 1),
            on_scroll_ver: None,
            on_scroll_hor: None,
            child_type: PhantomData,
        }
    }

    /// Enables or disables automatic mouse scroll handling.
    ///
    /// If enabled (default), the widget will update the `state` automatically
    /// on scroll events (if mouse capture is enabled). Otherwise default
    /// scroll event handling is turned off.
    #[must_use]
    pub fn scrollable(mut self, enabled: bool) -> Self {
        self.handle_scroll = enabled;
        self
    }

    /// Sets the numbers of units to scroll per mouse wheel step for both axes.
    ///
    /// It is mainly used in automatic mouse scroll handling, but the step
    /// size also determines the value returned in the Message if custom
    /// scroll handler is used.
    ///
    /// Default is `1`.
    #[must_use]
    pub fn scroll_distance(mut self, distance: usize) -> Self {
        self.scroll_dist.x = distance;
        self.scroll_dist.y = distance;
        self
    }

    /// Sets the numbers of units to scroll per mouse wheel step for horizontal
    /// axis.
    ///
    /// It is mainly used in automatic mouse scroll handling, but the step
    /// size also determines the value returned in the Message if custom
    /// scroll handler is used.
    ///
    /// Default is `1`.
    #[must_use]
    pub fn scroll_distance_x(mut self, distance: usize) -> Self {
        self.scroll_dist.x = distance;
        self
    }

    /// Sets the numbers of units to scroll per mouse wheel step for vertical
    /// axis.
    ///
    /// It is mainly used in automatic mouse scroll handling, but the step
    /// size also determines the value returned in the Message if custom
    /// scroll handler is used.
    ///
    /// Default is `1`.
    #[must_use]
    pub fn scroll_distance_y(mut self, distance: usize) -> Self {
        self.scroll_dist.y = distance;
        self
    }

    /// Sets the response to the vertical mouse scroll event.
    ///
    /// This disables the default vertical scroll handler, so only the given
    /// response will be used.
    ///
    /// The `response` is closure accepting a `isize` value - scroll offset
    /// based on the scroll direction and the set vertical scroll step size.
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
        self.on_scroll_ver = Some(Box::new(response));
        self
    }

    /// Sets the response to the horizontal mouse scroll event.
    ///
    /// This disables the default horizontal scroll handler, so only the given
    /// response will be used.
    ///
    /// The `response` is closure accepting a `isize` value - scroll offset
    /// based on the scroll direction and the set horizontal scroll step size.
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn on_scroll_horizontal<F>(mut self, response: F) -> Self
    where
        F: Fn(isize) -> M + 'static,
    {
        self.on_scroll_hor = Some(Box::new(response));
        self
    }
}

impl<M, W> Widget<M> for Scrollable<M, W>
where
    M: Clone + 'static,
    W: Widget<M>,
{
    fn render(
        &self,
        buffer: &mut Buffer,
        layout: &LayoutNode,
        cache: &mut Cache,
    ) {
        if layout.area.is_empty() {
            return;
        }

        let child_node = &layout.children[0];
        let ver_active = child_node.area.height() > layout.area.height();
        let hor_active = child_node.area.width() > layout.area.width();

        let mut viewport = layout.area;
        if ver_active {
            viewport.size.x = viewport.size.x.saturating_sub(1);
        }
        if hor_active {
            viewport.size.y = viewport.size.y.saturating_sub(1);
        }

        let h = child_node.area.height();
        let offset_y = Self::process_state(&self.ver_state, ver_active, h);
        let w = child_node.area.height();
        let offset_x = Self::process_state(&self.hor_state, hor_active, w);

        let mut cid = 1;
        let mut draw_scrollbar = |a: bool, e: &Option<Element<M>>, r: Rect| {
            if a && let Some(e) = e {
                let snode = &layout.children[cid];
                Self::scrollbar(buffer, snode, &mut cache.children[cid], e, r);
                cid += 1;
            }
        };

        let vrect = Rect::new(
            layout.area.right(),
            layout.area.y(),
            1,
            viewport.height(),
        );
        draw_scrollbar(ver_active, &self.vertical, vrect);

        let hrect = Rect::new(
            layout.area.x(),
            layout.area.bottom(),
            viewport.width(),
            1,
        );
        draw_scrollbar(hor_active, &self.horizontal, hrect);

        let cnode = &mut cache.children[cid];
        self.render_content(
            buffer, viewport, child_node, cnode, offset_x, offset_y,
        );
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

    fn children(&self) -> Vec<&Element<M>> {
        std::iter::once(&self.child)
            .chain(self.vertical.iter())
            .chain(self.horizontal.iter())
            .collect()
    }

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.horizontal.is_some().hash(&mut hasher);
        self.vertical.is_some().hash(&mut hasher);

        if let Some(state) = &self.hor_state {
            state.get().hash(&mut hasher);
        }
        if let Some(state) = &self.ver_state {
            state.get().hash(&mut hasher);
        }

        hasher.finish()
    }

    fn layout(&self, node: &mut LayoutNode, area: Rect) {
        if !node.is_dirty && !node.has_dirty_child {
            return;
        }

        node.area = area;
        node.is_dirty = false;
        node.has_dirty_child = false;

        let has_ver = self.ver_state.is_some();
        let has_hor = self.hor_state.is_some();
        if !has_ver && !has_hor {
            self.child.layout(&mut node.children[0], area);
            return;
        }

        let mut size = area.size;
        let mut ver_scroll = false;
        let mut hor_scroll = false;
        let calc_size = |size: &mut Vec2| {
            if has_ver {
                size.y = self.child.height(size);
            }
            if has_hor {
                size.x = self.child.width(size);
            }
        };

        let mut test_size = Vec2::new(
            if has_hor { usize::MAX } else { size.x },
            if has_ver { usize::MAX } else { size.y },
        );
        calc_size(&mut test_size);

        if has_ver && test_size.y > size.y {
            ver_scroll = true;
            size.x = size.x.saturating_sub(1);
        }

        if has_hor && test_size.x > size.x {
            hor_scroll = true;
            size.y = size.y.saturating_sub(1);

            if has_ver && !ver_scroll && test_size.y > size.y {
                ver_scroll = true;
                size.x = size.x.saturating_sub(1);
            }
        }

        test_size = Vec2::new(
            if has_hor { usize::MAX } else { size.x },
            if has_ver { usize::MAX } else { size.y },
        );
        if ver_scroll || hor_scroll {
            calc_size(&mut test_size);
        }

        let child_rect = Rect::new(
            area.x(),
            area.y(),
            if hor_scroll { test_size.x } else { size.x },
            if ver_scroll { test_size.y } else { size.y },
        );
        self.child.layout(&mut node.children[0], child_rect);
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
        self.child
            .on_event(area, &mut cache.children[0], event)
            .or_else(|| self.handle_mouse(area, event))
    }
}

impl<M, W> Scrollable<M, W>
where
    M: Clone + 'static,
    W: Widget<M>,
{
    /// Renders the scrollable content
    fn render_content(
        &self,
        buffer: &mut Buffer,
        rect: Rect,
        node: &LayoutNode,
        cache: &mut Cache,
        offset_x: usize,
        offset_y: usize,
    ) {
        let mut cbuffer = Buffer::empty(node.area);

        let mut mask = rect;
        mask.pos.x += offset_x;
        mask.pos.y += offset_y;

        let mut cutout = buffer.subset(rect);
        cutout.move_to(*mask.pos());
        cbuffer.merge(cutout);

        self.child.render(&mut cbuffer, node, cache);

        mask = mask.intersection(cbuffer.rect());
        let mut cutout = cbuffer.subset(mask);
        cutout.move_to(*rect.pos());
        buffer.merge(cutout);
    }

    /// Renders the scrollbar
    fn scrollbar(
        buffer: &mut Buffer,
        layout: &LayoutNode,
        cache: &mut Cache,
        scroll: &Element<M>,
        rect: Rect,
    ) {
        let mut temp_node = layout.clone();
        temp_node.area = rect;
        scroll.render(buffer, &temp_node, cache);
    }

    fn process_state(
        state: &Option<Rc<Cell<ScrollbarState>>>,
        is_active: bool,
        content_len: usize,
    ) -> usize {
        let Some(st) = state else {
            return 0;
        };

        if is_active {
            let mut s = st.get();
            s.content_len = content_len;
            st.set(s);
        }

        st.get().offset
    }

    fn handle_mouse(&self, area: Rect, event: &MouseEvent) -> EventResult<M> {
        if !self.handle_scroll {
            return EventResult::None;
        }

        use MouseEventKind::*;

        let dx = self.scroll_dist.x as isize;
        let dy = self.scroll_dist.y as isize;
        match &event.kind {
            ScrollDown if event.modifiers.contains(KeyModifiers::SHIFT) => {
                self.hor_move_offset(area, dx)
            }
            ScrollUp if event.modifiers.contains(KeyModifiers::SHIFT) => {
                self.hor_move_offset(area, -dx)
            }
            ScrollDown => self.ver_move_offset(area, dy),
            ScrollUp => self.ver_move_offset(area, -dy),
            ScrollLeft => self.hor_move_offset(area, -dx),
            ScrollRight => self.hor_move_offset(area, dx),
            _ => EventResult::None,
        }
    }

    fn ver_move_offset(&self, area: Rect, delta: isize) -> EventResult<M> {
        let scroll = || {
            let height = area
                .height()
                .saturating_sub(self.horizontal.is_some() as usize);
            self.apply_scroll(&self.ver_state, height, delta);
        };
        self.handle_scroll(&self.on_scroll_ver, scroll, delta)
    }

    fn hor_move_offset(&self, area: Rect, delta: isize) -> EventResult<M> {
        let scroll = || {
            let width = area
                .width()
                .saturating_sub(self.vertical.is_some() as usize);
            self.apply_scroll(&self.hor_state, width, delta);
        };
        self.handle_scroll(&self.on_scroll_hor, scroll, delta)
    }

    fn handle_scroll<F>(
        &self,
        handler: &Option<Box<dyn Fn(isize) -> M>>,
        scroll: F,
        delta: isize,
    ) -> EventResult<M>
    where
        F: Fn(),
    {
        if let Some(handler) = handler {
            return EventResult::Response(handler(delta));
        }

        if !self.handle_scroll {
            return EventResult::None;
        }
        scroll();
        EventResult::Consumed
    }

    fn apply_scroll(
        &self,
        scrollbar: &Option<Rc<Cell<ScrollbarState>>>,
        size: usize,
        delta: isize,
    ) {
        if let Some(state) = &scrollbar {
            let s = state.get();
            state.set(s.offset(Self::get_offset(&s, delta, size)));
        };
    }

    fn get_offset(state: &ScrollbarState, delta: isize, size: usize) -> usize {
        if delta < 0 {
            state.offset.saturating_sub(delta.unsigned_abs())
        } else {
            (state.offset + delta as usize)
                .min(state.content_len.saturating_sub(size))
        }
    }
}

impl<M, W> From<Scrollable<M, W>> for Box<dyn Widget<M>>
where
    M: Clone + 'static,
    W: Widget<M> + 'static,
{
    fn from(value: Scrollable<M, W>) -> Self {
        Box::new(value)
    }
}

impl<M, W> From<Scrollable<M, W>> for Element<M>
where
    M: Clone + 'static,
    W: Widget<M> + 'static,
{
    fn from(value: Scrollable<M, W>) -> Self {
        Element::new(value)
    }
}
