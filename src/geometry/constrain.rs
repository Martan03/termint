/// [`Constrain`] enum contains some constrains for when adjusting layout
#[derive(Debug)]
pub enum Constrain {
    /// Actual size
    Length(usize),
    /// Percentage size of the parent widget
    Percent(usize),
    /// Minimum size of the widget
    Min(usize),
    /// Fills rest of the space (other widgets will not have space left)
    Fill,
}
