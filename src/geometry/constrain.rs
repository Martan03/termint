/// [`Constrain`] enum contains some constrains for when adjusting layout
#[derive(Debug, PartialEq)]
pub enum Constrain {
    /// Actual size
    Length(usize),
    /// Percentage size of the parent widget
    Percent(usize),
    /// Minimum size of the widget, widget expands to fit content
    Min(usize),
    /// Fills rest of the space (space is divided by all widgets with fill)
    Fill,
}
