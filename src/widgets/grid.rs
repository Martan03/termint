use crate::{
    buffer::Buffer,
    geometry::{Rect, Unit, Vec2},
};

use super::{widget::Widget, Element};

/// A layout widget that arranges children in a grid specified by rows and
/// columns.
///
/// Each row and column is defined by a [`Unit`], which you can read more about
/// in its documentation.
///
/// Children can be placed by specifying their zero-based column and row
/// indices.
///
/// # Example
/// ```rust
/// # use termint::{
/// #     geometry::{Rect, Unit},
/// #     widgets::{Grid, Widget},
/// #     term::Term,
/// # };
/// # fn example() -> Result<(), &'static str> {
/// let mut grid = Grid::new(
///     vec![Unit::Length(3), Unit::Length(5), Unit::Fill(1)],
///     vec![Unit::Fill(1), Unit::Length(1), Unit::Fill(1)],
/// );
/// grid.push("Grid", 1, 1);
///
/// let mut term = Term::new();
/// term.render(grid)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct Grid {
    children: Vec<GridChild>,
    rows: Vec<Unit>,
    cols: Vec<Unit>,
}

/// Internal struct representing a child widget in a specific grid cell.
#[derive(Debug)]
struct GridChild {
    pub child: Element,
    pub row: usize,
    pub col: usize,
}

impl Grid {
    /// Creates a new [`Grid`] from columns and rows specifications.
    ///
    /// Both `cols` and `rows` accept any iterable of types convertible into
    /// [`Unit`].
    ///
    /// # Example
    /// ```rust
    /// # use termint::{
    /// #     geometry::{Rect, Unit},
    /// #     widgets::{Grid, Widget},
    /// #     term::Term,
    /// # };
    /// let mut grid = Grid::new([3, 5, 3], [3, 1, 1]);
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

    /// Creates an new empty [`Grid`] with no rows or columns.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Adds a new row definition to the [`Grid`].
    pub fn row(&mut self, row: Unit) {
        self.rows.push(row);
    }

    /// Adds given column to current columns
    pub fn col(&mut self, col: Unit) {
        self.cols.push(col);
    }

    /// Adds child to the grid to given row and column
    #[deprecated(
        since = "0.6.0",
        note = "Kept for compatibility purposes; use `push` function instead"
    )]
    pub fn add_child<T>(&mut self, child: T, col: usize, row: usize)
    where
        T: Into<Element>,
    {
        self.children.push(GridChild {
            child: child.into(),
            row,
            col,
        })
    }

    /// Adds a child widget at the specified column and row.
    ///
    /// # Parameters
    /// - `child`: The widget to add (any type convertible to [`Element`])
    /// - `col`: Zero-based column index (x)
    /// - `row`: Zero-based row index (y)
    pub fn push<T>(&mut self, child: T, col: usize, row: usize)
    where
        T: Into<Element>,
    {
        self.children.push(GridChild {
            child: child.into(),
            row,
            col,
        })
    }
}

impl Widget for Grid {
    fn render(&self, buffer: &mut Buffer, rect: Rect) {
        if rect.is_empty() || self.children.is_empty() {
            return;
        }

        let (cols, rows) = self.get_sizes(&rect);

        for GridChild { child, row, col } in self.children.iter() {
            let crect = Rect::new(
                rect.x() + cols[*col].y,
                rect.y() + rows[*row].y,
                cols[*col].x,
                rows[*row].x,
            );
            child.render(buffer, crect);
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

    fn children(&self) -> Vec<&Element> {
        self.children.iter().map(|c| &c.child).collect()
    }
}

impl Grid {
    /// Gets sizes and starting positions of each row and column
    fn get_sizes(&self, rect: &Rect) -> (Vec<Vec2>, Vec<Vec2>) {
        (
            Self::get_size(&self.cols, rect.width()),
            Self::get_size(&self.rows, rect.height()),
        )
    }

    /// Gets sizes and positions of given units
    fn get_size(units: &[Unit], size: usize) -> Vec<Vec2> {
        let mut total = 0;
        let mut fills_total = 0;

        let mut sizes = Vec::new();
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
            sizes.push(Vec2::new(len, total));
            total += len;
        }

        if fills_total == 0 {
            return sizes;
        }

        let mut pos = 0;
        let remain = size.saturating_sub(total);
        for (i, row) in units.iter().enumerate() {
            match row {
                Unit::Fill(f) => {
                    sizes[i].x = remain * f / fills_total;
                    sizes[i].y = pos;
                    pos += sizes[i].x;
                }
                _ => {
                    sizes[i].y = pos;
                    pos += sizes[i].x;
                }
            }
        }

        sizes
    }
}

impl From<Grid> for Box<dyn Widget> {
    fn from(value: Grid) -> Self {
        Box::new(value)
    }
}

impl From<Grid> for Element {
    fn from(value: Grid) -> Self {
        Element::new(value)
    }
}
