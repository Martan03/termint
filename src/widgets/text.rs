use crate::geometry::coords::Coords;

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
    ) -> Coords;

    /// Gets height of the [`Text`] with given offset
    /// Returns offset where text ends
    fn height_offset(&self, size: &Coords, offset: usize) -> usize;

    /// Gets width of the [`Text`] with given offset
    /// Returns offset where text ends
    fn width_offset(&self, size: &Coords, offset: usize) -> usize;

    /// Gets [`Text`] ansi codes (fg, bg, mods) in String
    fn get_mods(&self) -> String;
}
