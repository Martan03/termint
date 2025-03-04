use std::{cell::RefCell, rc::Rc};

use crate::{
    buffer::Buffer,
    geometry::{Rect, Unit, Vec2},
    text::Text,
};

use super::{Element, ListState, Widget};

#[derive(Debug)]
pub struct Table {
    header: Option<Vec<Box<dyn Text>>>,
    rows: Vec<Vec<Element>>,
    widths: Vec<Unit>,
    column_spacing: usize,
    state: Rc<RefCell<ListState>>,
}

impl Table {
    /// Creates new [`Table`] with given rows and columns widths
    pub fn new<R, C, W>(
        rows: R,
        widths: W,
        state: Rc<RefCell<ListState>>,
    ) -> Self
    where
        R: IntoIterator,
        R::Item: IntoIterator<Item = C>,
        C: Into<Element>,
        W: IntoIterator,
        W::Item: Into<Unit>,
    {
        Self {
            header: None,
            rows: rows
                .into_iter()
                .map(|r| r.into_iter().map(|i| i.into()).collect())
                .collect(),
            widths: widths.into_iter().map(|c| c.into()).collect(),
            column_spacing: 1,
            state,
        }
    }

    /// Adds given header to the [`Table`]
    pub fn header<H>(mut self, header: H) -> Self
    where
        H: IntoIterator,
        H::Item: Into<Box<dyn Text>>,
    {
        self.header = Some(header.into_iter().map(|h| h.into()).collect());
        self
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

    /// Sets the column spacing of the [`Table`]
    pub fn column_spacing(mut self, space: usize) -> Self {
        self.column_spacing = space;
        self
    }
}

impl Widget for Table {
    fn render(&self, buffer: &mut Buffer) {
        let widths = self.calc_widths(buffer.size());
        self.render_header(buffer, &widths);

        let mut pos = *buffer.pos();
        pos.y += self.header.is_some() as usize;
        for i in self.state.borrow().offset..self.rows.len() {
            if buffer.bottom() < pos.y {
                break;
            }

            let row_height =
                Self::row_height(buffer.height(), &self.rows[i], &widths);
            if row_height == 0 {
                continue;
            }

            let mut size = Vec2::new(0, row_height);
            for (j, child) in self.rows[i].iter().enumerate() {
                size.x = widths.get(j).copied().unwrap_or_default();
                let mut cbuffer = buffer.subset(Rect::from_coords(pos, size));
                child.render(&mut cbuffer);
                buffer.merge(cbuffer);
                pos.x += size.x + self.column_spacing;
            }

            pos.x = buffer.x();
            pos.y += row_height;
        }
    }

    fn height(&self, size: &Vec2) -> usize {
        let widths = self.calc_widths(size);
        self.rows
            .iter()
            .map(|r| Self::row_height(size.y, r, &widths))
            .sum()
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
        total + self.column_spacing * (self.widths.len() - 1)
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
                    calc_widths.push(*f);
                    continue;
                }
            };
            total += csize;
            calc_widths.push(csize);
        }

        total = total
            .saturating_sub(self.column_spacing * (calc_widths.len() - 1));
        let mut left = size.x.saturating_sub(total);
        for f in fills {
            let fill = calc_widths[f];
            calc_widths[f] = left / total_fills * fill;
            left -= calc_widths[f];
            total_fills -= fill;
        }

        calc_widths
    }

    fn render_header(&self, buffer: &mut Buffer, widths: &[usize]) {
        let Some(header) = &self.header else {
            return;
        };

        let mut pos = *buffer.pos();
        for (i, child) in header.iter().enumerate() {
            let width = widths.get(i).copied().unwrap_or_default();
            if width == 0 {
                continue;
            }

            let mut cbuffer =
                buffer.subset(Rect::from_coords(pos, Vec2::new(width, 1)));
            child.render_offset(&mut cbuffer, 0, None);
            buffer.merge(cbuffer);
            pos.x += width;
        }

        // buffer.set_str(
        //     "─".repeat(buffer.width()),
        //     &Vec2::new(buffer.x(), buffer.y() + 1),
        // );
    }

    fn row_height(height: usize, row: &[Element], widths: &[usize]) -> usize {
        let mut row_height = 0;
        for (i, child) in row.iter().enumerate() {
            let width = widths.get(i).copied().unwrap_or_default();
            if width == 0 {
                continue;
            }

            let height = child.height(&Vec2::new(width, height));
            row_height = row_height.max(height);
        }
        row_height
    }
}
