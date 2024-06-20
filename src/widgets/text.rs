use core::fmt;

use crate::{
    buffer::buffer::Buffer, enums::wrap::Wrap, geometry::coords::Coords,
};

/// Trait for text widgets to implement
/// Makes work with more [`Text`] widgets easier
pub trait Text {
    /// Renders [`Text`] on given position with given size but with offset
    /// Returns coords where rendered text ends
    fn render_offset(
        &self,
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<&Wrap>,
    ) -> Coords;

    /// Gets [`Text`] widget as string
    fn get(&self) -> String;

    /// Gets [`Text`] widget as string with all the positional things
    fn get_offset(
        &self,
        buffer: &mut Buffer,
        offset: usize,
        wrap: Option<&Wrap>,
    ) -> (String, Coords);

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
