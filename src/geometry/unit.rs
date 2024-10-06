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

impl From<usize> for Unit {
    fn from(value: usize) -> Self {
        Self::Length(value)
    }
}

impl From<f64> for Unit {
    fn from(value: f64) -> Self {
        if !(0.0..=1.0).contains(&value) {
            panic!("Float in range from 0 to 1 is expected");
        }
        Self::Percent((value * 100.0) as usize)
    }
}
