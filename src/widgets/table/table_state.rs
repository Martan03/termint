/// State of the [`Table`] widget, including scroll offset, selected index and
/// selected column.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TableState {
    pub offset: usize,
    pub selected: Option<usize>,
    pub selected_column: Option<usize>,
}

impl TableState {
    /// Creates a new [`TableState`] with the given scroll offset, no
    /// selected item and no selected column.
    #[must_use]
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            ..Default::default()
        }
    }

    /// Sets the selected index to the given value
    #[must_use]
    pub fn selected<T>(mut self, selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.selected = selected.into();
        self
    }

    /// Sets the selected column to the given value
    #[must_use]
    pub fn selected_column<T>(mut self, selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.selected_column = selected.into();
        self
    }
}
