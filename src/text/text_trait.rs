use core::fmt;

use crate::{buffer::Buffer, enums::Wrap, geometry::Vec2};

/// Trait for text widgets to implement.
///
/// Makes work with more [`Text`] widgets easier.
pub trait Text {
    /// Renders [`Text`] to the buffer, starting at the given offset and using
    /// the given wrap style.
    ///
    /// Returns position where rendered text ends.
    ///
    /// ### Example
    /// ```rust
    /// use termint::{
    ///     text::Text, widgets::ToSpan, enums::Wrap, buffer::Buffer
    /// };
    ///
    /// let span = "Hello, Termint!".to_span();
    /// let mut buffer = Buffer::empty((1, 1, 20, 1));
    ///
    /// // Renders text with offset of 3 with word wrapping
    /// span.render_offset(&mut buffer, 3, Some(Wrap::Word));
    /// ```
    fn render_offset(
        &self,
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<Wrap>,
    ) -> Vec2;

    /// Gets [`Text`] widget as string
    fn get(&self) -> String;

    /// Gets text of the [`Text`]
    fn get_text(&self) -> &str;

    /// Gets [`Text`] ansi codes (fg, bg, mods) in String
    fn get_mods(&self) -> String;
}

impl fmt::Debug for dyn Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Converted text")
    }
}
