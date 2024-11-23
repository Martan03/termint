use core::fmt;

use crate::{buffer::Buffer, enums::Wrap, geometry::Vec2};

/// Trait for text widgets to implement
/// Makes work with more [`Text`] widgets easier
pub trait Text {
    /// Renders [`Text`] on given position with given size but with offset
    /// Returns coords where rendered text ends
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
