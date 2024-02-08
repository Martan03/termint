use crate::{
    geometry::{coords::Coords, padding::Padding},
    widgets::widget::Widget,
};

/// [`Term`] implements full screen rendering with option to set padding
///
/// ## Usage:
/// ```
/// # use termint::{
/// #    term::Term, widgets::{block::Block, span::StrSpanExtension}
/// # };
///
/// let main = Block::new().title("Example".to_span());
/// // Creates new Term with padding 1 on every side
/// let mut term = Term::new().padding(1);
/// // Renders block over full screen
/// term.render(main);
///
/// // Term with zero padding on top and bottom and one on right and left
/// term = term.padding((0, 1));
/// // Term with padding 0 on top, 1 on right, 2 on bottom, 3 on left
/// term = term.padding((0, 1, 2, 3));
/// ```
#[derive(Debug, Default)]
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

    /// Renders given widget on full screen with set padding
    pub fn render<T>(&self, widget: T) -> Result<(), &'static str>
    where
        T: Widget + 'static,
    {
        if let Some((w, h)) = Term::get_size() {
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

    /// Gets size of the terminal
    pub fn get_size() -> Option<(usize, usize)> {
        term_size::dimensions()
    }
}
