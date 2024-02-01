use crate::{
    geometry::{coords::Coords, padding::Padding},
    widgets::widget::Widget,
};

#[derive(Debug)]
pub struct Term {
    padding: Padding,
}

impl Term {
    /// Creates new [`Term`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets [`Padding`] of the [`Term`] to given value
    pub fn padding<T: Into<Padding>>(mut self, padding: T) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn render<T>(&self, widget: T) -> Result<(), &'static str>
    where
        T: Widget + 'static,
    {
        if let Some((w, h)) = term_size::dimensions() {
            widget.render(
                &Coords::new(1 + self.padding.left, 1 + self.padding.top),
                &Coords::new(
                    w.saturating_sub(self.padding.get_horizontal()),
                    h.saturating_sub(self.padding.get_vertical()),
                ),
            );
            Ok(())
        } else {
            Err("Cannot determine terminal size")
        }
    }
}

impl Default for Term {
    fn default() -> Self {
        Self {
            padding: Default::default(),
        }
    }
}
