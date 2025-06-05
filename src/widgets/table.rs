use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

use crate::{
    buffer::Buffer,
    enums::{Border, BorderType},
    geometry::{Padding, Rect, Unit, Vec2},
    style::Style,
    widgets::cache::Cache,
};

use super::{Element, Widget};

mod row;
pub use row::Row;

mod table_state;
pub use table_state::TableState;

/// A widget that displays a table with configurable column widths, optional
/// header and scrollable row content.
///
/// Each cell is represented by an [`Element`], so cell can be any widget.
/// Layout of the cells is controlled by per-column widths and optional spacing
/// between columns.
///
/// # Example
/// ```rust
/// # use std::{cell::RefCell, rc::Rc};
/// # use termint::{
/// #    term::Term, geometry::Unit, enums::BorderType,
/// #    widgets::{Element, Table, TableState}
/// # };
/// # fn get_people() -> Vec<Vec<Element>> { return vec![] }
/// # fn example() -> Result<(), &'static str> {
/// let rows = get_people();
/// let widths = [Unit::Fill(3), Unit::Fill(1), Unit::Fill(3)];
/// let state = Rc::new(RefCell::new(TableState::new(0).selected(1)));
///
/// let table = Table::new(rows, widths, state)
///     .header(vec!["Name", "Age", "Email"])
///     .header_separator(BorderType::Double)
///     .column_spacing(2);
///
/// let mut term = Term::new();
/// term.render(table)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Table {
    header: Option<Row>,
    header_separator: Option<BorderType>,
    rows: Vec<Row>,
    selected_row_style: Style,
    selected_column_style: Style,
    selected_cell_style: Style,
    widths: Vec<Unit>,
    column_spacing: usize,
    state: Rc<RefCell<TableState>>,
    auto_scroll: bool,
}

impl Table {
    /// Creates new [`Table`] with given rows and columns widths
    #[must_use]
    pub fn new<R, W>(
        rows: R,
        widths: W,
        state: Rc<RefCell<TableState>>,
    ) -> Self
    where
        R: IntoIterator,
        R::Item: Into<Row>,
        W: IntoIterator,
        W::Item: Into<Unit>,
    {
        Self {
            header: None,
            header_separator: None,
            rows: rows.into_iter().map(Into::into).collect(),
            selected_row_style: Style::default(),
            selected_column_style: Style::default(),
            selected_cell_style: Style::default(),
            widths: widths.into_iter().map(|c| c.into()).collect(),
            column_spacing: 1,
            state,
            auto_scroll: false,
        }
    }

    /// Adds given header to the [`Table`]
    #[must_use]
    pub fn header<H>(mut self, header: H) -> Self
    where
        H: Into<Row>,
    {
        self.header = Some(header.into());
        self
    }

    /// Sets the header separator of the [`Table`]
    #[must_use]
    pub fn header_separator(mut self, separator: BorderType) -> Self {
        self.header_separator = Some(separator);
        self
    }

    /// Sets [`Table`] rows to the given value
    #[must_use]
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

    /// Sets the selected row style
    #[must_use]
    pub fn selected_row_style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.selected_row_style = style.into();
        self
    }

    /// Sets the selected column style
    #[must_use]
    pub fn selected_column_style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.selected_column_style = style.into();
        self
    }

    /// Sets the selected cell style
    #[must_use]
    pub fn selected_cell_style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.selected_cell_style = style.into();
        self
    }

    /// Sets columns widths of the [`Table`]
    #[must_use]
    pub fn widths<W>(mut self, widths: W) -> Self
    where
        W: IntoIterator,
        W::Item: Into<Unit>,
    {
        self.widths = widths.into_iter().map(|w| w.into()).collect();
        self
    }

    /// Sets the column spacing of the [`Table`]
    #[must_use]
    pub fn column_spacing(mut self, space: usize) -> Self {
        self.column_spacing = space;
        self
    }

    /// Enables automatic scrolling to ensure the selected item is visible.
    #[must_use]
    pub fn auto_scroll(mut self) -> Self {
        self.auto_scroll = true;
        self
    }
}

impl Widget for Table {
    fn render(&self, buffer: &mut Buffer, rect: Rect, _cache: &mut Cache) {
        let mut widths = self.calc_widths(rect.width());
        let header_height = self.calc_header_height(&rect, &widths);

        let mut crect = rect.clone();
        crect = crect.inner(Padding::top(header_height));
        if !self.fits(crect.size(), &widths) {
            // TODO: recalculate header height
            crect = crect.inner(Padding::right(1));
            widths = self.calc_widths(crect.width());
            let srect = Rect::new(rect.right(), crect.y(), 1, crect.height());
            self.render_scrollbar(buffer, &srect);
        }

        self.render_header(buffer, &rect, header_height, &widths);

        if self.auto_scroll {
            self.scroll_offset(crect.size(), &widths);
        }

        let selected = self.state.borrow().selected;

        let mut pos = *crect.pos();
        let mut row_rect = None;
        for i in self.state.borrow().offset..self.rows.len() {
            if rect.bottom() < pos.y {
                break;
            }

            let row_height =
                Self::row_height(rect.height(), &self.rows[i], &widths);
            if row_height == 0 {
                continue;
            }

            let mut size = Vec2::new(crect.width(), row_height);
            let rrect = Rect::from_coords(pos, size);
            if let Some(id) = selected {
                if id == i {
                    row_rect = Some(rrect);
                }
            }
            buffer.set_area_style(self.rows[i].style, rrect);

            for (j, child) in self.rows[i].cells.iter().enumerate() {
                size.x = widths.get(j).copied().unwrap_or_default();
                let crect = Rect::from_coords(pos, size);
                child.render(buffer, crect);
                pos.x += size.x + self.column_spacing;
            }

            pos.x = rect.x();
            pos.y += row_height;
        }

        if let Some(row_rect) = row_rect {
            buffer.set_area_style(self.selected_row_style, row_rect);
        }

        let crect = rect.inner(Padding::top(header_height));
        self.set_sel_style(buffer, &crect, &widths, row_rect);
    }

    fn height(&self, size: &Vec2) -> usize {
        let widths = self.calc_widths(size.x);
        let height: usize = self
            .rows
            .iter()
            .map(|r| Self::row_height(size.y, r, &widths))
            .sum();
        height
            + self.header.is_some() as usize
            + self.header_separator.is_some() as usize
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

    fn children(&self) -> Vec<&Element> {
        self.rows.iter().flat_map(|row| row.cells.iter()).collect()
    }
}

impl Table {
    fn calc_header_height(&self, rect: &Rect, widths: &[usize]) -> usize {
        let mut header_height = self.header_separator.is_some() as usize;
        if let Some(header) = &self.header {
            header_height += Self::row_height(rect.height(), header, &widths);
        }
        header_height
    }

    /// Gets calculated column widths based on the given size
    fn calc_widths(&self, width: usize) -> Vec<usize> {
        let mut calc_widths = Vec::new();
        let mut total = 0;

        let mut total_fills = 0;
        let mut fills = Vec::new();

        for w in self.widths.iter() {
            let csize = match w {
                Unit::Length(len) => *len,
                Unit::Percent(p) => width * p / 100,
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
        let mut left = width.saturating_sub(total);
        for f in fills {
            let fill = calc_widths[f];
            calc_widths[f] = left / total_fills * fill;
            left -= calc_widths[f];
            total_fills -= fill;
        }

        calc_widths
    }

    /// Renders [`Table`] scrollbar
    fn render_scrollbar(&self, buffer: &mut Buffer, rect: &Rect) {
        let rat = self.rows.len() as f32 / rect.height() as f32;
        let thumb_size = max(
            1,
            min((rect.height() as f32 / rat).round() as usize, rect.height()),
        );
        let thumb_offset = min(
            (self.state.borrow().offset as f32 / rat) as usize,
            rect.height() - thumb_size,
        );

        let mut bar_pos = Vec2::new(rect.right(), rect.y());
        for _ in 0..rect.height() {
            buffer.set_val('│', &bar_pos);
            // buffer.set_fg(self.scrollbar_fg, &bar_pos);
            bar_pos.y += 1;
        }

        bar_pos = Vec2::new(rect.right(), rect.y() + thumb_offset);
        for _ in 0..thumb_size {
            buffer.set_val('┃', &bar_pos);
            // buffer.set_fg(self.thumb_fg, &bar_pos);
            bar_pos.y += 1;
        }
    }

    fn set_sel_style(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        widths: &[usize],
        rrect: Option<Rect>,
    ) {
        let Some(selected) = self.state.borrow().selected_column else {
            return;
        };

        let mut x = rect.x();
        for (i, width) in widths.iter().enumerate() {
            if i == selected {
                let crect = Rect::new(x, rect.y(), *width, rect.height());
                buffer.set_area_style(self.selected_column_style, crect);
                if let Some(rrect) = rrect {
                    buffer.set_area_style(self.selected_row_style, rrect);
                    buffer.set_area_style(
                        self.selected_cell_style,
                        rrect.intersection(&crect),
                    )
                }
                return;
            }
            x += width;
        }
    }

    fn render_header(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        height: usize,
        widths: &[usize],
    ) {
        let Some(header) = &self.header else {
            return;
        };

        let height =
            height.saturating_sub(self.header_separator.is_some() as usize);
        let mut pos = *rect.pos();
        for (i, child) in header.cells.iter().enumerate() {
            let width = widths.get(i).copied().unwrap_or_default();
            if width == 0 {
                continue;
            }

            let crect = Rect::from_coords(pos, Vec2::new(width, height));
            child.render(buffer, crect);
            pos.x += width + self.column_spacing;
        }

        if let Some(separator) = &self.header_separator {
            let line =
                separator.get(Border::TOP).to_string().repeat(rect.width());
            buffer.set_str(line, &Vec2::new(rect.x(), rect.y() + height));
        }
    }

    fn row_height(height: usize, row: &Row, widths: &[usize]) -> usize {
        let mut row_height = 0;
        for (i, child) in row.cells.iter().enumerate() {
            let width = widths.get(i).copied().unwrap_or_default();
            if width == 0 {
                continue;
            }

            let height = child.height(&Vec2::new(width, height));
            row_height = row_height.max(height);
        }
        row_height
    }

    /// Automatically scrolls so the selected item is visible
    fn scroll_offset(&self, size: &Vec2, widths: &[usize]) {
        let Some(selected) = self.state.borrow().selected else {
            return;
        };

        if selected < self.state.borrow().offset {
            self.state.borrow_mut().offset = selected;
            return;
        }

        while !self.is_visible(
            selected,
            self.state.borrow().offset,
            size,
            widths,
        ) {
            self.state.borrow_mut().offset += 1;
        }
    }

    /// Checks if item is visible with given offset
    fn is_visible(
        &self,
        item: usize,
        offset: usize,
        size: &Vec2,
        widths: &[usize],
    ) -> bool {
        let mut height = 0;
        for i in offset..self.rows.len() {
            height += Self::row_height(size.y, &self.rows[i], widths);
            if height > size.y {
                return false;
            }

            if i == item {
                return true;
            }
        }
        false
    }

    /// Checks if list fits to the visible area
    fn fits(&self, size: &Vec2, widths: &[usize]) -> bool {
        self.is_visible(self.rows.len() - 1, 0, size, widths)
    }
}

impl From<Table> for Box<dyn Widget> {
    fn from(value: Table) -> Self {
        Box::new(value)
    }
}

impl From<Table> for Element {
    fn from(value: Table) -> Self {
        Element::new(value)
    }
}
