use core::fmt;

use crate::{
    buffer::Buffer,
    enums::Wrap,
    geometry::{Padding, Rect, Vec2},
    prelude::TextAlign,
    text::Line,
};

/// A trait implemented by all the widgets that render styled or formatted
/// text.
pub trait Text<'a> {
    /// Renders the [`Text`] into the given buffer within the provided [`Rect`]
    /// bounds, starting at the given offset and applying the specified
    /// wrapping strategy.
    ///
    /// Returns the final position where the rendering ends.
    ///
    /// # Example
    /// ```rust
    /// # use termint::{
    /// #     geometry::Rect, text::Text, widgets::ToSpan,
    /// #     enums::Wrap, buffer::Buffer
    /// # };
    /// let span = "Hello, Termint!".to_span();
    ///
    /// let rect = Rect::new(1, 1, 20, 1);
    /// let mut buffer = Buffer::empty(rect);
    ///
    /// // Renders text with offset of 3 with word wrapping
    /// span.render_offset(&mut buffer, rect, 3, Some(Wrap::Word));
    /// ```
    fn render_offset(
        &self,
        buffer: &mut Buffer,
        rect: Rect,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2;

    /// Appends the lines of the [`Text`] into the given lines.
    ///
    /// It tries to append the text to the last line first before adding a new
    /// line.
    ///
    /// Returns `true` when the entire text fit within the size, otherwise
    /// returns `false`.
    fn append_lines(
        &'a self,
        lines: &mut Vec<Line<'a>>,
        size: Vec2,
        wrap: Option<Wrap>,
    ) -> bool;

    /// Returns the formatted representation of the text as a `String`.
    fn get(&self) -> String;

    /// Returns the raw, unformatted string content.
    fn get_text(&self) -> &str;

    /// Returns ANSI escape sequences representing the current style
    /// (e.g., foreground/background colors, modifiers).
    fn get_mods(&self) -> String;
}

/// Generic text render function, which uses the `Text::append_lines` to get
/// the lines and render them.
pub fn text_render<'a>(
    text: &'a dyn Text<'a>,
    buffer: &mut Buffer,
    mut rect: Rect,
    ellipsis: &str,
    align: TextAlign,
) {
    let mut lines = vec![];
    let fit = text.append_lines(&mut lines, *rect.size(), None);

    if !fit {
        lines
            .last_mut()
            .map(|l| l.add_ellipsis(rect.width(), ellipsis));
    }

    for line in lines {
        line.render(buffer, rect, align);
        rect = rect.inner(Padding::top(1));
    }
}

impl<'a> fmt::Debug for dyn Text<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Converted text")
    }
}
