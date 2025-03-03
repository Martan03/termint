use crate::{
    buffer::Buffer,
    geometry::{Unit, Vec2},
};

use super::{Element, Widget};

pub struct Table {
    rows: Vec<Vec<Element>>,
    widths: Vec<Unit>,
}

impl Table {
    /// Creates new [`Table`] with given rows and columns widths
    pub fn new<R, C, W>(rows: R, widths: W) -> Self
    where
        R: IntoIterator,
        R::Item: IntoIterator<Item = C>,
        C: Into<Element>,
        W: IntoIterator,
        W::Item: Into<Unit>,
    {
        Self {
            rows: rows
                .into_iter()
                .map(|r| r.into_iter().map(|i| i.into()).collect())
                .collect(),
            widths: widths.into_iter().map(|c| c.into()).collect(),
        }
    }

    /// Sets [`Table`] rows to the given value
    pub fn rows<R, C>(mut self, rows: R) -> Self
    where
        R: IntoIterator,
        R::Item: IntoIterator<Item = C>,
        C: Into<Element>,
    {
        self.rows = rows
            .into_iter()
            .map(|r| r.into_iter().map(|i| i.into()).collect())
            .collect();
        self
    }

    /// Sets columns widths of the [`Table`]
    pub fn widths<W>(mut self, widths: W) -> Self
    where
        W: IntoIterator,
        W::Item: Into<Unit>,
    {
        self.widths = widths.into_iter().map(|w| w.into()).collect();
        self
    }
}

impl Widget for Table {
    fn render(&self, buffer: &mut Buffer) {
        let widths = self.calc_widths(buffer.size());

        for row in self.rows.iter() {
            let mut row_height = 0;
            for (i, child) in row.iter().enumerate() {
                let mut size = Vec2::new(
                    widths.get(i).copied().unwrap_or_default(),
                    buffer.height(),
                );
                size.y = child.height(&size);
                row_height = row_height.max(size.y);
            }
        }
    }

    fn height(&self, size: &Vec2) -> usize {
        let widths = self.calc_widths(size);

        let mut total = 0;
        for row in self.rows.iter() {
            let mut row_height = 0;
            for (i, child) in row.iter().enumerate() {
                row_height = row_height.max(child.height(&Vec2::new(
                    widths.get(i).copied().unwrap_or_default(),
                    size.y,
                )));
            }
            total += row_height;
        }
        total
    }

    fn width(&self, size: &Vec2) -> usize {
        let mut total = 0;
        let mut fill = false;
        for width in self.widths.iter() {
            match width {
                Unit::Length(len) => total += len,
                Unit::Percent(p) => total += size.x * p / 100,
                Unit::Fill(_) => fill = true,
            }
        }
        if fill {
            return total.max(size.x);
        }
        total
    }
}

impl Table {
    /// Gets calculated column widths based on the given size
    fn calc_widths(&self, size: &Vec2) -> Vec<usize> {
        let mut calc_widths = Vec::new();
        let mut total = 0;

        let mut total_fills = 0;
        let mut fills = Vec::new();

        for width in self.widths.iter() {
            let csize = match width {
                Unit::Length(len) => *len,
                Unit::Percent(p) => size.x * p / 100,
                Unit::Fill(f) => {
                    total_fills += f;
                    fills.push(calc_widths.len());
                    calc_widths.push(0);
                    continue;
                }
            };
            total += csize;
            calc_widths.push(csize);
        }

        let mut left = size.x.saturating_sub(total);
        for f in fills {
            let fill = calc_widths[f];
            calc_widths[f] = left / total_fills * fill;
            left -= calc_widths[f];
            total_fills -= fill;
        }

        calc_widths
    }
}
