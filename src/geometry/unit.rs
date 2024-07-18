/// [`Constrain`] enum contains some constrains for when adjusting layout
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Unit {
    /// Actual size
    Length(usize),
    /// Percentage size of the parent widget
    Percent(usize),
    /// Fills rest of the space (space is divided by all widgets with fill)
    Fill(usize),
}
