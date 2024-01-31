use crate::{enums::wrap::Wrap, geometry::coords::Coords};

/// Trait for text widgets to implement
/// Makes work with more [`Text`] widgets easier
pub trait Text {
    /// Renders [`Text`] on given position with given size but with offset
    /// Returns coords where rendered text ends
    fn render_offset(
        &self,
        pos: &Coords,
        size: &Coords,
        offset: usize,
        wrap: &Wrap,
    ) -> Coords;

    /// Gets [`Text`] widget as string
    fn get(&self) -> String;

    /// Gets text of the [`Text`]
    fn get_text(&self) -> &str;

    /// Gets [`Text`] ansi codes (fg, bg, mods) in String
    fn get_mods(&self) -> String;
}
