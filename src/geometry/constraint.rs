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
    Fill(usize),
}

impl From<usize> for Constraint {
    fn from(value: usize) -> Self {
        Self::Length(value)
    }
}

impl From<(usize, usize)> for Constraint {
    fn from((min, max): (usize, usize)) -> Self {
        Self::MinMax(min, max)
    }
}

impl From<f64> for Constraint {
    fn from(value: f64) -> Self {
        if !(0.0..=1.0).contains(&value) {
            panic!("Float in range from 0 to 1 is expected");
        }
        Self::Percent((value * 100.0) as usize)
    }
}
