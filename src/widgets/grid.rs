use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{
    buffer::Buffer,
    geometry::{Rect, Unit, Vec2},
    widgets::{
        cache::{Cache, GridCache},
        layout::LayoutNode,
        widget::EventResult,
    },
};

use super::{Element, widget::Widget};

/// A layout widget that arranges children in a grid specified by rows and
/// columns.
///
/// The grid layout is defined by a sequence of columns (x-axis) and rows
/// (y-axis). Each row and column is sized according to a [`Unit`] (e.g. fixed
/// length, percentage).
///
/// # Coordinates
///
/// Children are placed using zero-based indices:
/// - `col` (x-axis): 0 is the leftmost column.
/// - `row` (y-axis): 0 is the topmost row.
///
/// # Visual representation
///
/// Each cell contains the column (x-axis) and row (y-axis) indices.
///
/// ```txt
///         Col 0    Col 1
///       +--------+--------+
/// Row 0 | (0, 0) | (1, 0) |
///       +--------+--------+
/// Row 1 | (0, 1) | (1, 1) |
///       +--------+--------+
/// ```
///
/// # Example
/// ```rust
/// use termint::prelude::*;
///
/// // Creates 3x2 grid
/// let mut grid = Grid::<()>::new(
///     // Columns sizes (x-axis)
///     vec![Unit::Length(3), Unit::Length(5), Unit::Fill(1)],
///     // Rows sizes (y-axis)
///     vec![Unit::Fill(1), Unit::Length(1)],
/// );
/// // Pushes a widget into 2nd column (middle) and 1nd row (topmost).
/// grid.push("Grid", 1, 0);
/// ```
#[derive(Debug)]
pub struct Grid<M: 'static = ()> {
    children: Vec<GridChild<M>>,
    rows: Vec<Unit>,
    cols: Vec<Unit>,
}

/// Internal struct representing a child widget in a specific grid cell.
#[derive(Debug)]
struct GridChild<M: 'static = ()> {
    pub child: Element<M>,
    pub row: usize,
    pub col: usize,
}

impl<M> Grid<M> {
    /// Creates a new [`Grid`] with the specified column and row definitions.
    ///
    /// Both `cols` and `rows` accept any iterable of types convertible into
    /// [`Unit`].
    ///
    /// # Example
    /// ```rust
    /// use termint::prelude::*;
    ///
    /// // Creates 3x2 grid with only fixed lengths
    /// let mut grid = Grid::<()>::new([3, 5, 3], [3, 1]);
    ///
    /// // Creates 3x3 grid which centers the middle column and row.
    /// let mut grid = Grid::<()>::new(
    ///     [Unit::Fill(1), Unit::Length(10), Unit::Fill(1)],
    ///     [Unit::Fill(1), Unit::Length(5), Unit::Fill(1)],
    /// );
    /// ```
    #[must_use]
    pub fn new<T1, T2>(cols: T1, rows: T2) -> Self
    where
        T1: IntoIterator,
        T1::Item: Into<Unit>,
        T2: IntoIterator,
        T2::Item: Into<Unit>,
    {
        Self {
            children: vec![],
            rows: rows.into_iter().map(|r| r.into()).collect(),
            cols: cols.into_iter().map(|c| c.into()).collect(),
        }
    }

    /// Creates a new empty [`Grid`] with no rows or columns.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Adds a new row definition to the bottom of the [`Grid`].
    pub fn row(&mut self, row: Unit) {
        self.rows.push(row);
    }

    /// Appends a new column definition to the right of the [`Grid`].
    pub fn col(&mut self, col: Unit) {
        self.cols.push(col);
    }

    /// Adds a child widget at the specified column and row.
    ///
    /// The `child` is any type convertible into [`Element`].
    ///
    /// The `col` (x-axis) and `row` (y-axis) are zero-based indices.
    pub fn push<T>(&mut self, child: T, col: usize, row: usize)
    where
        T: Into<Element<M>>,
    {
        self.children.push(GridChild {
            child: child.into(),
            row,
            col,
        })
    }
}

impl<M: Clone + 'static> Widget<M> for Grid<M> {
    fn render(
        &self,
        buffer: &mut Buffer,
        layout: &LayoutNode,
        cache: &mut Cache,
    ) {
        if layout.area.is_empty() || self.children.is_empty() {
            return;
        }

        for (i, GridChild { child, .. }) in self.children.iter().enumerate() {
            child.render(buffer, &layout.children[i], &mut cache.children[i]);
        }
    }

    fn height(&self, size: &Vec2) -> usize {
        let mut height = 0;
        for row in self.rows.iter() {
            match row {
                Unit::Length(len) => height += len,
                Unit::Percent(p) => height += size.y * p / 100,
                _ => {}
            }
        }
        height
    }

    fn width(&self, size: &Vec2) -> usize {
        let mut width = 0;
        for col in self.cols.iter() {
            match col {
                Unit::Length(len) => width += len,
                Unit::Percent(p) => width += size.y * p / 100,
                _ => {}
            }
        }
        width
    }

    fn children(&self) -> Vec<&Element<M>> {
        self.children.iter().map(|c| &c.child).collect()
    }

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.rows.hash(&mut hasher);
        self.cols.hash(&mut hasher);

        hasher.finish()
    }

    fn layout(&self, node: &mut LayoutNode, area: Rect) {
        if !node.is_dirty && !node.has_dirty_child && node.area == area {
            return;
        }

        node.area = area;
        node.is_dirty = false;
        node.has_dirty_child = false;

        let (cols, cols_pos) = Self::get_size(&self.cols, area.width());
        let (rows, rows_pos) = Self::get_size(&self.rows, area.height());

        for (i, item) in self.children.iter().enumerate() {
            let crect = Rect::new(
                area.x() + cols_pos[item.col],
                area.y() + rows_pos[item.row],
                cols[item.col],
                rows[item.row],
            );
            item.child.layout(&mut node.children[i], crect);
        }
    }

    fn on_event(
        &self,
        area: Rect,
        cache: &mut Cache,
        event: &crate::prelude::MouseEvent,
    ) -> EventResult<M> {
        if !area.contains_pos(&event.pos) {
            return EventResult::None;
        }

        let (cols, cols_pos, rows, rows_pos) = self.get_sizes(&area, cache);
        for (i, GridChild { child, row, col }) in
            self.children.iter().enumerate()
        {
            let crect = Rect::new(
                area.x() + cols_pos[*col],
                area.y() + rows_pos[*row],
                cols[*col],
                rows[*row],
            );
            let m = child.on_event(crect, &mut cache.children[i], event);
            if !m.is_none() {
                return m;
            }
        }
        EventResult::None
    }
}

impl<M> Grid<M> {
    /// Gets sizes and starting positions of each row and column
    fn get_sizes(
        &self,
        rect: &Rect,
        cache: &mut Cache,
    ) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
        match self.get_cache(rect, cache) {
            Some((cols, rows)) => {
                let cols_pos = Self::get_positions(&cols);
                let rows_pos = Self::get_positions(&rows);
                (cols, cols_pos, rows, rows_pos)
            }
            None => {
                let cols = Self::get_size(&self.cols, rect.width());
                let rows = Self::get_size(&self.rows, rect.height());
                self.create_cache(rect, cache, &cols.0, &rows.0);
                (cols.0, cols.1, rows.0, rows.1)
            }
        }
    }

    /// Gets sizes and positions of given units
    fn get_size(units: &[Unit], size: usize) -> (Vec<usize>, Vec<usize>) {
        let mut total = 0;
        let mut fills_total = 0;

        let mut sizes = Vec::new();
        let mut positions = Vec::new();
        let mut fills = Vec::new();
        for unit in units {
            let len = match unit {
                Unit::Length(len) => *len,
                Unit::Percent(p) => size * p / 100,
                Unit::Fill(f) => {
                    fills_total += f;
                    fills.push(sizes.len());
                    *f
                }
            };
            sizes.push(len);
            positions.push(total);
            total += len;
        }

        if fills_total == 0 {
            return (sizes, positions);
        }

        let mut pos = 0;
        let remain = size.saturating_sub(total);
        for (i, row) in units.iter().enumerate() {
            match row {
                Unit::Fill(f) => {
                    sizes[i] = remain * f / fills_total;
                    positions[i] = pos;
                    pos += sizes[i];
                }
                _ => {
                    positions[i] = pos;
                    pos += sizes[i];
                }
            }
        }

        (sizes, positions)
    }

    fn get_positions(sizes: &[usize]) -> Vec<usize> {
        let mut total = 0;
        let mut pos = vec![];
        for size in sizes {
            pos.push(total);
            total += size;
        }
        pos
    }

    fn get_cache(
        &self,
        rect: &Rect,
        cache: &mut Cache,
    ) -> Option<(Vec<usize>, Vec<usize>)> {
        let lcache = cache.local::<GridCache>()?;
        if !lcache.same_key(rect.size(), &self.cols, &self.rows) {
            return None;
        }
        Some((lcache.col_sizes.clone(), lcache.row_sizes.clone()))
    }

    fn create_cache(
        &self,
        rect: &Rect,
        cache: &mut Cache,
        cols: &[usize],
        rows: &[usize],
    ) {
        let lcache =
            GridCache::new(*rect.size(), self.cols.clone(), self.rows.clone())
                .sizes(cols.to_owned(), rows.to_owned());
        cache.local = Some(Box::new(lcache));
    }
}

impl<M> Default for Grid<M> {
    fn default() -> Self {
        Self {
            children: Default::default(),
            rows: Default::default(),
            cols: Default::default(),
        }
    }
}

impl<M: Clone + 'static> From<Grid<M>> for Box<dyn Widget<M>> {
    fn from(value: Grid<M>) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Grid<M>> for Element<M> {
    fn from(value: Grid<M>) -> Self {
        Element::new(value)
    }
}
