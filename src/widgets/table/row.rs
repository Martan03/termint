use crate::{
    style::Style,
    widgets::{Element, ToSpan},
};

/// Represents a single row in a [`crate::widgets::Table`] widget.
///
/// A [`Row`] consists of a list of [`Element`]s (one per column) and base
/// style of the row.
///
/// # Example
///
/// Row can be constructed from iterator of types convertable to [`Element`].
///
/// ```rust
/// use termint::{prelude::*, widgets::Grad};
///
/// // Row can be created from iterator over strings.
/// let row: Row<()> = ["First", "Second", "Third"].into_iter().collect();
///
/// // You can style the entire row.
/// let row = Row::<()>::new(["First", "Second", "Third"]).style(Color::Red);
///
/// // Or you can create row from elements.
/// let row = Row::<()>::new([
///     Element::new("First".fg(Color::Green)),
///     Element::new("Second".modifier(Modifier::BOLD)),
///     Element::new(Grad::new("Third", (0, 220, 255), (200, 60, 255))),
/// ]);
/// ```
#[derive(Debug)]
pub struct Row<M: 'static> {
    pub(crate) cells: Vec<Element<M>>,
    pub(crate) style: Style,
}

impl<M> Row<M> {
    /// Creates a new [`Row`] from the given cells.
    ///
    /// You can provide any type that can be converted into an iterator of any
    /// type, that can be converted into [`Element`].
    ///
    /// # Example
    /// ```rust
    /// # use termint::{widgets::{Row, ToSpan}, enums::Color};
    /// let row = Row::<()>::new(["First", "Second", "Third"]);
    /// let row = Row::<()>::new(vec![
    ///     "First".fg(Color::Red),
    ///     "Second".fg(Color::Green),
    ///     "Third".fg(Color::Blue),
    /// ]);
    /// ```
    #[must_use]
    pub fn new<T>(cells: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Element<M>>,
    {
        Self {
            cells: cells.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Sets the base style of the entire [`Row`].
    ///
    /// Accepts any type that is convertible to [`Style`] (read style docs for
    /// more details).
    #[must_use]
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.style = style.into();
        self
    }
}

impl<M> Default for Row<M> {
    fn default() -> Self {
        Self {
            cells: Default::default(),
            style: Default::default(),
        }
    }
}

impl<M, I> FromIterator<I> for Row<M>
where
    I: Into<Element<M>>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self::new(iter)
    }
}

impl<M, T> From<Vec<T>> for Row<M>
where
    T: Into<Element<M>>,
{
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().map(Into::into).collect()
    }
}

impl<M, T> From<&Vec<T>> for Row<M>
where
    M: Clone + 'static,
    for<'a> &'a T: ToSpan,
{
    fn from(vec: &Vec<T>) -> Self {
        vec.iter()
            .map(|item| Element::from(item.to_span()))
            .collect()
    }
}
