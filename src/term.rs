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
    small: Option<Box<dyn Widget>>,
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

    /// Sets small screen of the [`Term`], which is displayed if rendering
    /// cannot fit
    pub fn small_screen<T>(mut self, small_screen: T) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        self.small = Some(small_screen.into());
        self
    }

    /// Renders given widget on full screen with set padding. Displays small
    /// screen when cannot fit (only when `small_screen` is set)
    pub fn render<T>(&self, widget: T) -> Result<(), &'static str>
    where
        T: Widget + 'static,
    {
        let Some((w, h)) = Term::get_size() else {
            return Err("Cannot determine terminal size");
        };

        let pos = Coords::new(1 + self.padding.left, 1 + self.padding.top);
        let size = Coords::new(
            w.saturating_sub(self.padding.get_horizontal()),
            h.saturating_sub(self.padding.get_vertical()),
        );

        match &self.small {
            Some(small)
                if w < widget.width(&size) || h < widget.height(&size) =>
            {
                small.render(&pos, &size);
            }
            _ => widget.render(&pos, &size),
        };
        Ok(())
    }

    /// Gets size of the terminal
    pub fn get_size() -> Option<(usize, usize)> {
        term_size::dimensions()
    }
}
