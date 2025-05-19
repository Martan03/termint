use core::fmt;

use crate::{
    buffer::Buffer,
    enums::Wrap,
    geometry::{Rect, Vec2},
};

/// A trait implemented by all the widgets that render styled or formatted
/// text.
pub trait Text {
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

    /// Returns the formatted representation of the text as a `String`.
    fn get(&self) -> String;

    /// Returns the raw, unformatted string content.
    fn get_text(&self) -> &str;

    /// Returns ANSI escape sequences representing the current style
    /// (e.g., foreground/background colors, modifiers).
    fn get_mods(&self) -> String;
}

impl fmt::Debug for dyn Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Converted text")
    }
}
