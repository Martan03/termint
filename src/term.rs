use termal::raw::term_size;

use crate::{
    buffer::Buffer,
    geometry::{Padding, Rect, Vec2},
    widgets::{cache::Cache, Element, Widget},
};

/// [`Term`] implements full screen rendering with option to set padding
///
/// ## Usage:
/// ```
/// # use termint::{
/// #    term::Term, widgets::{Block, ToSpan}
/// # };
///
/// let main = Block::vertical().title("Example".to_span());
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
    prev: Option<Buffer>,
    prev_widget: Option<Element>,
    small: Option<Element>,
    cache: Cache,
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
        T: Into<Element>,
    {
        self.small = Some(small_screen.into());
        self
    }

    /// Renders given widget on full screen with set padding. Displays small
    /// screen when cannot fit (only when `small_screen` is set)
    pub fn render<T>(&mut self, widget: T) -> Result<(), &'static str>
    where
        T: Into<Element>,
    {
        let Some((w, h)) = Term::get_size() else {
            return Err("Cannot determine terminal size");
        };

        let widget = widget.into();
        let pos = Vec2::new(1 + self.padding.left, 1 + self.padding.top);
        let size = Vec2::new(
            w.saturating_sub(self.padding.get_horizontal()),
            h.saturating_sub(self.padding.get_vertical()),
        );

        let rect = Rect::from_coords(pos, size);
        let mut buffer = Buffer::empty(rect);
        match &self.small {
            Some(small)
                if w < widget.width(&size) || h < widget.height(&size) =>
            {
                self.cache.diff(&small);
                small.render(&mut buffer, rect, &mut self.cache);
            }
            _ => {
                self.cache.diff(&widget);
                widget.render(&mut buffer, rect, &mut self.cache);
            }
        };
        self.prev_widget = Some(widget);

        match &self.prev {
            Some(prev) => buffer.render_diff(prev),
            None => buffer.render(),
        }
        self.prev = Some(buffer);

        Ok(())
    }

    /// Rerenders the lastly rendered widget
    pub fn rerender(&mut self) -> Result<(), &'static str> {
        let Some(widget) = &self.prev_widget else {
            return Err("Cannot rerender: no previous rendering");
        };

        let Some((w, h)) = Term::get_size() else {
            return Err("Cannot determine terminal size");
        };

        let pos = Vec2::new(1 + self.padding.left, 1 + self.padding.top);
        let size = Vec2::new(
            w.saturating_sub(self.padding.get_horizontal()),
            h.saturating_sub(self.padding.get_vertical()),
        );

        let rect = Rect::from_coords(pos, size);
        let mut buffer = Buffer::empty(rect);
        match &self.small {
            Some(small)
                if w < widget.width(&size) || h < widget.height(&size) =>
            {
                self.cache.diff(&small);
                small.render(&mut buffer, rect, &mut self.cache);
            }
            _ => {
                self.cache.diff(&widget);
                widget.render(&mut buffer, rect, &mut self.cache);
            }
        };

        match &self.prev {
            Some(prev) => buffer.render_diff(prev),
            None => buffer.render(),
        }
        self.prev = Some(buffer);

        Ok(())
    }

    /// Gets size of the terminal
    pub fn get_size() -> Option<(usize, usize)> {
        term_size().ok().map(|s| (s.char_width, s.char_height))
    }
}
