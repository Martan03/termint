use crate::{
    buffer::Buffer,
    geometry::{Coords, Rect, Unit},
};

use super::widget::Widget;

/// Contains grid child and row and column in which it's located
#[derive(Debug)]
struct GridChild {
    pub child: Box<dyn Widget>,
    pub row: usize,
    pub col: usize,
}

/// Creates layout by specifying columns and rows
///
/// ## Example usage without using Term:
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     geometry::{Rect, Unit},
/// #     widgets::{Grid, Widget},
/// # };
/// let mut grid = Grid::new(
///     vec![Unit::Length(3), Unit::Length(5), Unit::Fill(1)],
///     vec![Unit::Fill(1), Unit::Length(1), Unit::Fill(1)],
/// );
///
/// grid.add_child("Grid", 1, 1);
///
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 15, 6));
/// grid.render(&mut buffer);
/// buffer.render();
/// ```
#[derive(Debug, Default)]
pub struct Grid {
    children: Vec<GridChild>,
    rows: Vec<Unit>,
    cols: Vec<Unit>,
}

impl Grid {
    /// Creates new [`Grid`] with given rows and columns
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

    /// Creates new empty [`Grid`]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Adds given row to current rows
    pub fn row(&mut self, row: Unit) {
        self.rows.push(row);
    }

    /// Adds given column to current columns
    pub fn col(&mut self, col: Unit) {
        self.cols.push(col);
    }

    /// Adds child to the grid to given row and column
    pub fn add_child<T>(&mut self, child: T, col: usize, row: usize)
    where
        T: Into<Box<dyn Widget>>,
    {
        self.children.push(GridChild {
            child: child.into(),
            row,
            col,
        })
    }
}

impl Widget for Grid {
    fn render(&self, buffer: &mut Buffer) {
        if buffer.area() == 0 || self.children.is_empty() {
            return;
        }

        let (cols, rows) = self.get_sizes(buffer);

        for GridChild { child, row, col } in self.children.iter() {
            let mut cbuffer = buffer.get_subset(Rect::new(
                buffer.x() + cols[*col].y,
                buffer.y() + rows[*row].y,
                cols[*col].x,
                rows[*row].x,
            ));
            child.render(&mut cbuffer);
            buffer.union(cbuffer);
        }
    }

    fn height(&self, size: &Coords) -> usize {
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

    fn width(&self, size: &Coords) -> usize {
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
}

impl Grid {
    /// Gets sizes and starting positions of each row and column
    fn get_sizes(&self, buffer: &mut Buffer) -> (Vec<Coords>, Vec<Coords>) {
        (
            Self::get_size(&self.cols, buffer.width()),
            Self::get_size(&self.rows, buffer.height()),
        )
    }

    /// Gets sizes and positions of given units
    fn get_size(units: &[Unit], size: usize) -> Vec<Coords> {
        let mut total = 0;
        let mut fills = 0;

        let mut sizes = Vec::new();
        for unit in units {
            match unit {
                Unit::Length(len) => {
                    sizes.push(Coords::new(*len, 0));
                    total += len;
                }
                Unit::Percent(p) => {
                    let len = size * p / 100;
                    sizes.push(Coords::new(len, 0));
                    total += len;
                }
                Unit::Fill(f) => {
                    sizes.push(Coords::new(0, 0));
                    fills += f;
                }
            }
        }

        let mut pos = 0;
        let remain = size.saturating_sub(total);
        for (i, row) in units.iter().enumerate() {
            match row {
                Unit::Fill(f) => {
                    sizes[i].x = remain * f / fills;
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
