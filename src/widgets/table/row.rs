use crate::{
    style::Style,
    widgets::{Element, ToSpan},
};

/// Represents a single row in a [`Table`] widget.
///
/// A [`Row`] consists of a list of [`Element`]s (one per column) and base
/// style of the row.
///
/// # Example
/// ```rust
/// # use termint::{widgets::Row, enums::Color};
/// let row = Row::new(["First", "Second", "Third"]).style(Color::Red);
/// let row: Row = ["First", "Second", "Third"].into_iter().collect();
/// ```
#[derive(Debug, Default)]
pub struct Row {
    pub(crate) cells: Vec<Element>,
    pub(crate) style: Style,
}

impl Row {
    /// Creates a new [`Row`] from the given cells.
    ///
    /// You can provide any type that can be converted into an iterator of any
    /// type, that can be converted into [`Element`].
    ///
    /// # Example
    /// ```rust
    /// # use termint::{widgets::{Row, ToSpan}, enums::Color};
    /// let row = Row::new(["First", "Second", "Third"]);
    /// let row = Row::new(vec![
    ///     "First".fg(Color::Red),
    ///     "Second".fg(Color::Green),
    ///     "Third".fg(Color::Blue),
    /// ]);
    /// ```
    #[must_use]
    pub fn new<T>(cells: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Element>,
    {
        Self {
            cells: cells.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Sets the base style of the [`Row`]
    #[must_use]
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.style = style.into();
        self
    }
}

impl<I> FromIterator<I> for Row
where
    I: Into<Element>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self::new(iter)
    }
}

impl<T> From<Vec<T>> for Row
where
    T: Into<Element>,
{
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().map(Into::into).collect()
    }
}

impl<T> From<&Vec<T>> for Row
where
    for<'a> &'a T: ToSpan,
{
    fn from(vec: &Vec<T>) -> Self {
        vec.iter()
            .map(|item| Element::from(item.to_span()))
            .collect()
    }
}
