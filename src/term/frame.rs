use crate::geometry::{Rect, Vec2};

/// Contains details about currently rendering frame.
///
/// The [`Frame`] is passed to [`Application::view`] and to the closure given
/// to [`Term::draw`] to provide context about the current frame. This ensures
/// that the UI can adapt to the size of the terminal, such as hide menu.
#[derive(Debug)]
pub struct Frame {
    area: Rect,
}

impl Frame {
    /// Creates a new frame with given area.
    pub(crate) fn new(area: Rect) -> Self {
        Self { area }
    }

    /// Gets the available rendering area of the terminal.
    ///
    /// This [`Rect`] accounts for the [`Padding`] set in the [`Term`].
    pub fn area(&self) -> &Rect {
        &self.area
    }

    /// Gets the size of the available rendering area of the terminal.
    pub fn size(&self) -> &Vec2 {
        self.area.size()
    }

    /// Gets the top-left position of the available rendering area of the
    /// terminal.
    pub fn pos(&self) -> &Vec2 {
        self.area.pos()
    }
}
