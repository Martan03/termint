use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

use crate::{
    buffer::{Buffer, Cell},
    enums::{Border, BorderType},
    geometry::{Padding, Rect, Unit, Vec2},
    style::Style,
    widgets::cache::{Cache, TableCache},
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
/// # fn example() -> Result<(), termint::Error> {
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
    force_scrollbar: bool,
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
            force_scrollbar: false,
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

    /// Forces scrollbar to be always visible. By default the scrollbar hides
    /// when the content doesn't overflow.
    #[must_use]
    pub fn force_scrollbar(mut self) -> Self {
        self.force_scrollbar = true;
        self
    }
}

impl Widget for Table {
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        if rect.is_empty() || self.rows.is_empty() {
            return;
        }

        let (widths, heights, header_height, scrollbar) =
            self.get_sizes(&rect, cache);
        let mut crect = rect.inner(Padding::top(header_height));
        if scrollbar {
            crect = crect.inner(Padding::right(1));
            let srect = Rect::new(rect.right(), crect.y(), 1, crect.height());
            self.render_scrollbar(buffer, &srect);
        }

        self.render_header(buffer, &rect, cache, header_height, &widths);
        if self.auto_scroll {
            self.scroll_offset(crect.size(), &heights);
        }

        let selected = self.state.borrow().selected;

        let mut pos = *crect.pos();
        let mut row_rect = None;

        let mut id = 0;
        if self.header.is_some() {
            id += self.widths.len();
        }

        for (i, height) in
            heights.iter().enumerate().skip(self.state.borrow().offset)
        {
            if rect.bottom() < pos.y {
                break;
            }

            let row_height = *height;
            if row_height == 0 {
                continue;
            }

            let row = &self.rows[i];
            let rrect = Rect::new(pos.x, pos.y, crect.width(), row_height);

            if pos.y + row_height > rect.bottom() + 1 {
                self.render_last(buffer, rrect, cache, &mut id, row, &widths);
            } else {
                buffer.set_area_style(row.style, rrect);
                self.render_row(buffer, &rrect, cache, &mut id, row, &widths);
            }

            pos.x = rect.x();
            pos.y += row_height;
            if let Some(id) = selected {
                if id == i {
                    row_rect = Some(rrect);
                }
            }
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
        let mut result = Vec::new();

        if let Some(header) = &self.header {
            result.extend(header.cells.iter());
        }

        for row in &self.rows {
            result.extend(row.cells.iter());
        }

        result
    }
}

impl Table {
    fn calc_header_height(&self, rect: &Rect, widths: &[usize]) -> usize {
        let mut header_height = self.header_separator.is_some() as usize;
        if let Some(header) = &self.header {
            header_height += Self::row_height(rect.height(), header, widths);
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

    fn calc_heights(&self, widths: &[usize]) -> (Vec<usize>, usize) {
        let mut total = 0;
        let mut heights = vec![];
        for child in self.rows.iter() {
            let height = Self::row_height(1, child, widths);
            total += height;
            heights.push(height);
        }
        (heights, total)
    }

    /// Gets sizes of each row and column and whether scrollbar is needed.
    fn get_sizes(
        &self,
        rect: &Rect,
        cache: &mut Cache,
    ) -> (Vec<usize>, Vec<usize>, usize, bool) {
        if let Some(sizes) = self.get_cache(rect, cache) {
            return sizes;
        };

        let mut w = self.calc_widths(rect.width());
        let mut header = self.calc_header_height(rect, &w);

        let mut crect = rect.inner(Padding::top(header));
        let (mut h, total) = self.calc_heights(&w);
        let scrollbar = self.force_scrollbar || crect.height() < total;
        if scrollbar {
            crect = rect.inner(Padding::right(1));
            w = self.calc_widths(crect.width());
            header = self.calc_header_height(&crect, &w);
            (h, _) = self.calc_heights(&w);
        }

        self.create_cache(rect, cache, &w, &h, header, scrollbar);
        (w, h, header, scrollbar)
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
            buffer.set_char('│', &bar_pos);
            // buffer.set_fg(self.scrollbar_fg, &bar_pos);
            bar_pos.y += 1;
        }

        bar_pos = Vec2::new(rect.right(), rect.y() + thumb_offset);
        for _ in 0..thumb_size {
            buffer.set_char('┃', &bar_pos);
            // buffer.set_fg(self.thumb_fg, &bar_pos);
            bar_pos.y += 1;
        }
    }

    fn render_row(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        cache: &mut Cache,
        id: &mut usize,
        row: &Row,
        widths: &[usize],
    ) {
        let mut pos = *rect.pos();
        let mut size = *rect.size();
        for (i, child) in row.cells.iter().enumerate() {
            size.x = widths.get(i).copied().unwrap_or_default();
            if size.x != 0 {
                let crect = Rect::from_coords(pos, size);
                child.render(buffer, crect, &mut cache.children[*id]);
                pos.x += size.x;
            }

            pos.x += self.column_spacing;
            *id += 1;
        }
    }

    fn render_last(
        &self,
        buffer: &mut Buffer,
        mut rect: Rect,
        cache: &mut Cache,
        id: &mut usize,
        row: &Row,
        widths: &[usize],
    ) {
        let mut cell = Cell::empty();
        cell.style(row.style);
        let mut rb = Buffer::filled(rect, cell);
        let height = rect.height() - (rect.bottom() - rect.bottom());
        rb.merge(buffer.subset(Rect::from_coords(
            *rect.pos(),
            Vec2::new(rect.width(), height),
        )));
        self.render_row(&mut rb, &rect, cache, id, row, widths);
        rect.size.y = height;
        rb = rb.subset(rect);
        buffer.merge(rb);
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
        cache: &mut Cache,
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
            child.render(buffer, crect, &mut cache.children[i]);
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
    fn scroll_offset(&self, size: &Vec2, heights: &[usize]) {
        let Some(selected) = self.state.borrow().selected else {
            return;
        };

        let offset = self.state.borrow().offset;
        if selected < offset {
            self.state.borrow_mut().offset = selected;
            return;
        }

        let mut height = heights[selected];
        for i in (offset..selected).rev() {
            height += heights[i];
            if height > size.y {
                self.state.borrow_mut().offset = i + 1;
                break;
            }
        }
    }

    fn get_cache(
        &self,
        rect: &Rect,
        cache: &mut Cache,
    ) -> Option<(Vec<usize>, Vec<usize>, usize, bool)> {
        let lcache = cache.local::<TableCache>()?;
        if !lcache.same_key(rect.size(), &self.widths) {
            return None;
        }
        Some((
            lcache.col_sizes.clone(),
            lcache.row_sizes.clone(),
            lcache.header_height,
            lcache.scrollbar,
        ))
    }

    fn create_cache(
        &self,
        rect: &Rect,
        cache: &mut Cache,
        cols: &[usize],
        rows: &[usize],
        header_height: usize,
        scrollbar: bool,
    ) {
        let lcache = TableCache::new(*rect.size(), self.widths.clone())
            .sizes(cols.to_owned(), rows.to_owned())
            .scrollbar(scrollbar)
            .header_height(header_height);
        cache.local = Some(Box::new(lcache));
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
