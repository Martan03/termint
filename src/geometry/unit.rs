/// Size unit enum, containing only some of the options from [`Constraint`]
///
/// [`Unit`] is currently used for the [`Grid`] widget, since it doesn't
/// support some of the [`Constraint`] options. This might be changed in the
/// future.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Unit {
    /// Actual size
    Length(usize),
    /// Percentage size of the parent widget
    Percent(usize),
    /// Fills rest of the space (space is divided by all widgets with fill)
    Fill(usize),
}
