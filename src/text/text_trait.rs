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
pub trait Text {
    /// Appends the lines of the [`Text`] into the given lines.
    ///
    /// It tries to append the text to the last line first before adding a new
    /// line.
    ///
    /// The `size` represents the whole size allocated for the lines, so
    /// widget takes into account how many lines are already in the `lines`.
    ///
    /// Returns `true` when the entire text fit within the size, otherwise
    /// returns `false`.
    ///
    /// # Example
    /// ```rust
    /// use termint::{prelude::*, text::Text};
    /// # fn get_text() -> Span { Span::new("Example") }
    ///
    /// let text = get_text();
    ///
    /// let mut lines = vec![];
    /// text.append_lines(&mut lines, &Vec2::new(20, 3), Some(Wrap::Word));
    /// ```
    fn append_lines<'a>(
        &'a self,
        lines: &mut Vec<Line<'a>>,
        size: &Vec2,
        wrap: Option<Wrap>,
    ) -> bool;

    /// Returns the formatted representation of the text as a `String`.
    fn get(&self) -> String;

    /// Returns the raw, unformatted string content.
    fn get_text(&self) -> &str;

    /// Gets the set [`TextAlign`]ment of the text.
    fn get_align(&self) -> TextAlign;
}

/// Generic text render function, which uses the `Text::append_lines` to get
/// the lines and render them.
pub fn text_render(
    text: &dyn Text,
    buffer: &mut Buffer,
    mut rect: Rect,
    ellipsis: &str,
    align: TextAlign,
) {
    let mut lines = vec![];
    let fit = text.append_lines(&mut lines, rect.size(), None);

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

impl fmt::Debug for dyn Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Converted text")
    }
}
