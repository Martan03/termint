/// Size constraints
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Constraint {
    /// Actual size
    Length(usize),
    /// Percentage size of the parent widget
    Percent(usize),
    /// Minimum size of the widget, widget expands to fit content
    Min(usize),
    /// Maximum size of the widget, widget expands to fit content
    Max(usize),
    /// Minimum and maximum size of the widget, widget expands to fit content
    MinMax(usize, usize),
    /// Fills rest of the space (space is divided by all widgets with fill)
    Fill,
}
