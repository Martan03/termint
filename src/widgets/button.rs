use crate::{
    buffer::Buffer,
    geometry::Padding,
    prelude::{MouseEvent, Rect, Vec2},
    style::Style,
    term::backend::{MouseButton, MouseEventKind},
    widgets::{cache::Cache, widget::EventResult, Element, Spacer, Widget},
};

pub struct Button<M: 'static> {
    child: Element<M>,
    padding: Padding,
    style: Style,
    on_click: Option<M>,
}

impl<M> Button<M> {
    /// Creates new [`Button`] wrapping the given child widget
    #[must_use]
    pub fn new<T>(child: T) -> Self
    where
        T: Into<Element<M>>,
    {
        Self {
            child: child.into(),
            padding: Default::default(),
            style: Default::default(),
            on_click: None,
        }
    }

    /// Sets the [`Padding`] of the [`Button`]
    #[must_use]
    pub fn padding<P>(mut self, padding: P) -> Self
    where
        P: Into<Padding>,
    {
        self.padding = padding.into();
        self
    }

    /// Sets the base [`Style`] of the [`Button`]
    #[must_use]
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the response Message of the on click handler.
    #[must_use]
    pub fn on_click(mut self, response: M) -> Self {
        self.on_click = Some(response);
        self
    }
}

impl<M: Clone + 'static> Button<M> {
    /// Creates empty [`Button`]
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
            MouseEventKind::Down(MouseButton::Left) => self
                .on_click
                .clone()
                .map(EventResult::Response)
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
