/// Stores the state of the [`Table`].
///
/// It includes:
/// - Scroll offset = based on rows, e.g. 5 means 5th row is first visible.
/// - Selected = optional ID of the selected row (starting from 0)
/// - Selected column = optional ID of the selected column (starting from 0)
///
/// # Example
///
/// This creates table state with scroll offset of 3 rows, 5th row and 2nd
/// column as selected.
///
/// ```rust
/// use termint::prelude::*;
/// use std::{cell::RefCell, rc::Rc};
///
/// // Create the state itself
/// let state = TableState::new(3).selected(5).selected_column(2);
///
/// // Wrap in Rc<RefCell<..>> for the table
/// let wrapped_state = Rc::new(RefCell::new(state));
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TableState {
    pub offset: usize,
    pub selected: Option<usize>,
    pub selected_column: Option<usize>,
}

impl TableState {
    /// Creates new [`TableState`] with the given scroll offset.
    ///
    /// This creates the state with no selected row or column.
    #[must_use]
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            ..Default::default()
        }
    }

    /// Creates new [`TableState`] with the given selected row.
    ///
    /// Scroll offset is set to 0 and no column is selected.
    ///
    /// The `selected` is any type convertable to `Option<usize>`.
    pub fn with_selected<T>(selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        Self {
            selected: selected.into(),
            ..Default::default()
        }
    }

    /// Creates new [`TableState`] with the given selected column.
    ///
    /// Scroll offset is set to 0 and no row is selected.
    ///
    /// The `selected` is any type convertable to `Option<usize>`.
    pub fn with_selected_column<T>(selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        Self {
            selected_column: selected.into(),
            ..Default::default()
        }
    }

    /// Sets the selected row index.
    ///
    /// The `selected` is any type convertable to `Option<usize>`. This allows
    /// unselecting the row as well.
    #[must_use]
    pub fn selected<T>(mut self, selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.selected = selected.into();
        self
    }

    /// Sets the selected column index.
    ///
    /// The `selected` is any type convertable to `Option<usize>`. This allows
    /// unselecting the column as well.
    #[must_use]
    pub fn selected_column<T>(mut self, selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.selected_column = selected.into();
        self
    }
}
