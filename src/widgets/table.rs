use std::{
    cell::RefCell,
    cmp::{max, min},
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

mod row;
pub use row::Row;

mod table_state;
pub use table_state::TableState;

use crate::{
    buffer::{Buffer, Cell},
    enums::{Border, BorderType},
    geometry::Padding,
    prelude::{KeyModifiers, MouseButton, MouseEvent, Rect, Unit, Vec2},
    style::Style,
    term::backend::MouseEventKind,
    widgets::{Element, EventResult, LayoutNode, Spacer, Widget},
};

type TableHandler<M> = Box<dyn Fn(usize, usize) -> M>;

/// A widget that displays a table with configurable column widths, optional
/// header and scrollable row content.
///
/// Each row is represented by [`Row`], where each cell is represented by an
/// [`Element`], so cell can be any widget. Layout of the cells is controlled
/// by per-column widths and optional spacing between columns.
///
/// See `table` example for more in depth example of how to use the [`Table`]
/// widget.
///
/// # Example
///
/// This example assumes you have already created a function, which returns
/// all the rows. If you want to see more about how to create a row, visit the
/// [`Row`] documentation.
///
/// ```rust
/// use termint::prelude::*;
/// use std::{cell::RefCell, rc::Rc};
///
/// # fn get_people() -> Vec<Vec<Element>> { return vec![] }
/// let rows = get_people();
/// let widths = [Unit::Fill(3), Unit::Fill(1), Unit::Fill(3)];
/// let state = Rc::new(RefCell::new(TableState::new(0).selected(1)));
///
/// let table = Table::new(rows, widths, state)
///     // Optional header is a Row always visible at the top.
///     .header(vec!["Name", "Age", "Email"])
///     // Horizontal border below header.
///     .header_separator(BorderType::Double)
///     // Fixed spacing between columns.
///     .column_spacing(2)
///     // You can also style selected row, column and cell
///     .selected_row_style(Color::Cyan)
///     .selected_column_style(Style::new().bg(Color::Red))
///     .selected_cell_style(Modifier::BOLD)
///     // Or enable auto scrolling, which assures the selected row is visible
///     .auto_scroll();
/// ```
pub struct Table<M: 'static = ()> {
    header: Option<Row<M>>,
    header_separator: Option<BorderType>,
    rows: Vec<Row<M>>,
    selected_row_style: Style,
    selected_column_style: Style,
    selected_cell_style: Style,
    widths: Vec<Unit>,
    column_spacing: usize,
    state: Rc<RefCell<TableState>>,
    auto_scroll: bool,
    handle_scroll: bool,
    scroll_step: Vec2,
    force_scrollbar: bool,
    handlers: Vec<(MouseButton, TableHandler<M>)>,
    on_scroll_ver: Option<Box<dyn Fn(isize) -> M>>,
    on_scroll_hor: Option<Box<dyn Fn(isize) -> M>>,
    dummy: Element<M>,
}

struct TableMetrics {
    widths: Vec<usize>,
    heights: Vec<usize>,
    header_height: usize,
    scrollbar: bool,
    rect: Rect,
}

impl<M: Clone> Table<M> {
    /// Creates new [`Table`] with given rows and columns widths.
    ///
    /// The `rows` can be any type convertible into an iterator of [`Row`]s.
    ///
    /// The `widths` accepts any type convertible into an iterator of
    /// [`Unit`]s.
    ///
    /// ```rust
    /// use termint::prelude::*;
    /// use std::{cell::RefCell, rc::Rc};
    ///
    /// # fn example() -> Table<()> {
    /// let rows = [
    ///     vec!["First", "Second"],
    ///     vec!["Third", "Fourth"],
    /// ];
    /// let widths = [Unit::Fill(1), Unit::Length(6)];
    /// let state = Rc::new(RefCell::new(TableState::default()));
    /// let table = Table::new(rows, widths, state);
    /// # table
    /// # }
    /// ```
    #[must_use]
    pub fn new<R, W>(
        rows: R,
        widths: W,
        state: Rc<RefCell<TableState>>,
    ) -> Self
    where
        R: IntoIterator,
        R::Item: Into<Row<M>>,
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
            handle_scroll: true,
            scroll_step: Vec2::new(1, 1),
            force_scrollbar: false,
            handlers: vec![],
            on_scroll_ver: None,
            on_scroll_hor: None,
            dummy: Element::new(Spacer),
        }
    }

    /// Adds given header to the [`Table`].
    ///
    /// Header is displayed at the top of the [`Table`] and always visible.
    ///
    /// The `header` is type convertible into [`Row`].
    #[must_use]
    pub fn header<H>(mut self, header: H) -> Self
    where
        H: Into<Row<M>>,
    {
        self.header = Some(header.into());
        self
    }

    /// Sets the header separator of the [`Table`].
    ///
    /// Header separator is a horizontal border below the header and together
    /// with the header is at the top of the [`Table`] and always visible.
    #[must_use]
    pub fn header_separator(mut self, separator: BorderType) -> Self {
        self.header_separator = Some(separator);
        self
    }

    /// Sets [`Table`] rows to the given value.
    ///
    /// The `rows` can be any type convertible into an iterator of [`Row`]s.
    #[must_use]
    pub fn rows<R, C>(mut self, rows: R) -> Self
    where
        R: IntoIterator,
        R::Item: IntoIterator<Item = C>,
        C: Into<Element<M>>,
    {
        self.rows = rows
            .into_iter()
            .map(|r| r.into_iter().map(|i| i.into()).collect())
            .collect();
        self
    }

    /// Sets the style of the selected row.
    ///
    /// It will overwrite the style of the [`Row`] and the individual cells.
    /// It also has priority over the selected column style.
    ///
    /// The `style` can be any type convertible to [`Style`].
    #[must_use]
    pub fn selected_row_style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.selected_row_style = style.into();
        self
    }

    /// Sets the style of the selected column.
    ///
    /// This styles the whole column and will overwrite [`Row`] styles and
    /// styles of the individual cells. Selected row style has higher priority
    /// though.
    ///
    /// The `style` can be any type convertible to [`Style`].
    #[must_use]
    pub fn selected_column_style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.selected_column_style = style.into();
        self
    }

    /// Sets the style of the selected cell.
    ///
    /// It has priority over any other style, including selected row and
    /// and selected column styles.
    ///
    /// The `style` can be any type convertible to [`Style`].
    #[must_use]
    pub fn selected_cell_style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.selected_cell_style = style.into();
        self
    }

    /// Sets the widths of the [`Table`] columns.
    ///
    /// The `widths` accepts any type convertible into an iterator of
    /// [`Unit`]s.
    #[must_use]
    pub fn widths<W>(mut self, widths: W) -> Self
    where
        W: IntoIterator,
        W::Item: Into<Unit>,
    {
        self.widths = widths.into_iter().map(|w| w.into()).collect();
        self
    }

    /// Sets the spacing in between [`Table`] columns.
    #[must_use]
    pub fn column_spacing(mut self, space: usize) -> Self {
        self.column_spacing = space;
        self
    }

    /// Enables automatic scrolling to ensure the selected item is visible.
    ///
    /// It always makes sure that the currently selected row is in the view.
    /// This allows for much easier scrolling implementation, where it's
    /// enough to change only the selected row.
    #[must_use]
    pub fn auto_scroll(mut self) -> Self {
        self.auto_scroll = true;
        self
    }

    /// Enables or disables automatic mouse scroll handling.
    ///
    /// By default table handles the mouse scroll events by changing the row
    /// and column selection depending on the vertical, respectively
    /// horizontal scroll events.
    ///
    /// The step size of the scrolling can be configured using
    /// [`Table::scroll_distance`](crate::widgets::Table::scroll_step),
    /// [`Table::scroll_distance_x`](crate::widgets::Table::scroll_step_x)
    /// and [`Table::scroll_distance_y`](crate::widgets::Table::scroll_step_y).
    #[must_use]
    pub fn scrollable(mut self, enabled: bool) -> Self {
        self.handle_scroll = enabled;
        self
    }

    /// Sets the number of rows/columns to scroll per mouse wheel step for both
    /// horizontal and vertical scrolling.
    ///
    /// It is mainly used in automatic mouse scroll handling, but the step
    /// size also determines the value returned in the Message if custom
    /// scroll handler is used.
    ///
    /// Default is `1`.
    #[must_use]
    pub fn scroll_step(mut self, size: usize) -> Self {
        self.scroll_step.x = size;
        self.scroll_step.y = size;
        self
    }

    /// Sets the number of columns to scroll per mouse wheel step.
    ///
    /// It is mainly used in automatic mouse scroll handling, but the step
    /// size also determines the value returned in the Message if custom
    /// scroll handler is used.
    ///
    /// Default is `1`.
    #[must_use]
    pub fn scroll_step_x(mut self, distance: usize) -> Self {
        self.scroll_step.x = distance;
        self
    }

    /// Sets the number of rows to scroll per mouse wheel step.
    ///
    /// It is mainly used in automatic mouse scroll handling, but the step
    /// size also determines the value returned in the Message if custom
    /// scroll handler is used.
    ///
    /// Default is `1`.
    #[must_use]
    pub fn scroll_step_y(mut self, distance: usize) -> Self {
        self.scroll_step.y = distance;
        self
    }

    /// Forces scrollbar to be always visible. By default the scrollbar hides
    /// when the content doesn't overflow.
    #[must_use]
    pub fn force_scrollbar(mut self) -> Self {
        self.force_scrollbar = true;
        self
    }

    /// Sets the response to the left mouse click event.
    ///
    /// This will overwrite any click responses already set.
    ///
    /// The `response` is closure accepting two `usize` values - column and
    /// row clicked - and returns the corresponding message.
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn on_click<F>(self, response: F) -> Self
    where
        F: Fn(usize, usize) -> M + 'static,
    {
        self.on_press(MouseButton::Left, response)
    }

    /// Sets the response to the button click event.
    ///
    /// This will overwrite any click responses already set for the given
    /// button.
    ///
    /// The `response` is closure accepting two `usize` values - column and
    /// row clicked - and returns the corresponding message.
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn on_press<F>(mut self, button: MouseButton, response: F) -> Self
    where
        F: Fn(usize, usize) -> M + 'static,
    {
        self.handlers.retain(|(b, _)| *b != button);
        self.handlers.push((button, Box::new(response)));
        self
    }

    /// Sets the response to the vertical mouse scroll event.
    ///
    /// This disables the default vertical scroll handler, so only the given
    /// response will be used.
    ///
    /// The `response` is closure accepting a `isize` value - scroll offset
    /// based on the scroll direction and the set vertical scroll step size.
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn on_scroll<F>(mut self, response: F) -> Self
    where
        F: Fn(isize) -> M + 'static,
    {
        self.on_scroll_ver = Some(Box::new(response));
        self
    }

    /// Sets the response to the horizontal mouse scroll event.
    ///
    /// This disables the default horizontal scroll handler, so only the given
    /// response will be used.
    ///
    /// The `response` is closure accepting a `isize` value - scroll offset
    /// based on the scroll direction and the set horizontal scroll step size.
    ///
    /// **Note:** This requires mouse capture to be enabled. You can do that by
    /// calling [`Term::with_mouse`](crate::term::Term::with_mouse) on
    /// [`Term`](crate::term::Term) struct or
    /// [`enable_mouse_capture`](crate::term::enable_mouse_capture) when not
    /// using  the [`Term`](crate::term::Term).
    #[must_use]
    pub fn on_scroll_horizontal<F>(mut self, response: F) -> Self
    where
        F: Fn(isize) -> M + 'static,
    {
        self.on_scroll_hor = Some(Box::new(response));
        self
    }
}

impl<M: Clone + 'static> Widget<M> for Table<M> {
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        let mut rect = layout.area;
        if rect.is_empty() || self.rows.is_empty() {
            return;
        }

        let snode = &layout.children[0];
        rect.size.x = rect.width().saturating_sub(snode.area.width());
        if snode.area.width() > 0 {
            self.render_scrollbar(buffer, &snode.area);
        }

        let mut cid = 1;
        let header_height =
            self.render_header(buffer, layout, &mut rect, &mut cid);

        let offset = self.state.borrow().offset;
        let selected = self.state.borrow().selected;
        let selected_col = self.state.borrow().selected_column;

        let mut col_x = 0;
        let mut col_w = 0;

        if let Some(col) = selected_col {
            let mut base_cid = 1;
            if self.header.is_none() {
                base_cid += offset * self.widths.len();
            }

            if let Some(cnode) = layout.children.get(base_cid + col) {
                col_x = cnode.area.x();
                col_w = cnode.area.width();
            }
        }

        let mut row_rect = None;
        cid += offset * self.widths.len();
        for (i, row) in self.rows.iter().enumerate().skip(offset) {
            if rect.is_empty() {
                break;
            }

            let row_height = layout.children[cid].area.height();
            self.render_clipped(buffer, layout, row, &mut rect, &mut cid);

            if let Some(sel) = selected
                && sel == i
            {
                let mut rrect = rect;
                rrect.size.y = rrect.size.y.min(row_height);
                row_rect = Some(rrect);
            }

            rect = rect.inner(Padding::top(row_height));
        }

        let crect =
            layout.area.inner((header_height, snode.area.width(), 0, 0));
        self.set_sel_style(buffer, &crect, col_x, col_w, row_rect);
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

    fn children(&self) -> Vec<&Element<M>> {
        std::iter::once(&self.dummy)
            .chain(self.header.iter().flat_map(|h| h.cells.iter()))
            .chain(self.rows.iter().flat_map(|r| r.cells.iter()))
            .collect()
    }

    fn layout_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.header_separator.hash(&mut hasher);
        self.widths.hash(&mut hasher);
        self.column_spacing.hash(&mut hasher);
        self.force_scrollbar.hash(&mut hasher);
        self.state.borrow().hash(&mut hasher);

        hasher.finish()
    }

    fn layout(&self, node: &mut LayoutNode, area: Rect) {
        if !node.is_dirty && !node.has_dirty_child && node.area == area {
            return;
        }

        node.area = area;
        node.is_dirty = false;
        node.has_dirty_child = false;
        if area.is_empty() || self.rows.is_empty() {
            return;
        }

        let metrics = self.calc_metrics(area);
        node.children[0].area = Rect::new(
            area.right(),
            area.y(),
            metrics.scrollbar as usize,
            area.height(),
        );

        if self.auto_scroll {
            self.scroll_offset(metrics.rect.size(), &metrics.heights);
        }

        let offset = self.state.borrow().offset;
        let mut cur_y = metrics.rect.y();
        let mut child_idx = 1;

        if let Some(header_row) = &self.header {
            let mut cur_x = metrics.rect.x();
            let sep_height = self.header_separator.is_some() as usize;

            let height = metrics.header_height.saturating_sub(sep_height);
            for (col_idx, cell) in header_row.cells.iter().enumerate() {
                let cell_area = Rect::new(
                    cur_x,
                    area.y(),
                    metrics.widths[col_idx],
                    height,
                );
                cell.layout(&mut node.children[child_idx], cell_area);
                cur_x += metrics.widths[col_idx] + self.column_spacing;
                child_idx += 1;
            }
        }

        for (i, row) in self.rows.iter().enumerate() {
            let row_height = metrics.heights[i];

            let is_visible = i >= offset && cur_y <= metrics.rect.bottom();
            let mut cur_x = metrics.rect.x();

            for (j, cell) in row.cells.iter().enumerate() {
                let cnode = &mut node.children[child_idx];
                if is_visible {
                    let w = metrics.widths[j];
                    let cell_area = Rect::new(cur_x, cur_y, w, row_height);
                    cell.layout(cnode, cell_area);
                    cur_x += w + self.column_spacing;
                } else {
                    cnode.area = Rect::default();
                    cnode.is_dirty = false;
                    cnode.has_dirty_child = false;
                }
                child_idx += 1;
            }

            if is_visible {
                cur_y += row_height;
            }
        }
    }

    fn on_event(&self, node: &LayoutNode, e: &MouseEvent) -> EventResult<M> {
        if !node.area.contains_pos(&e.pos) {
            return EventResult::None;
        }

        let m = self.on_event_header(node, e);
        if !m.is_none() {
            return m;
        }

        let offset = self.state.borrow().offset;
        let mut cid = 1 + offset * self.widths.len();
        if self.header.is_some() {
            cid += self.widths.len();
        }

        let scrollbar = node.children[0].area.width();
        let width = node.area.width().saturating_sub(scrollbar);
        for (row_id, row) in self.rows.iter().enumerate().skip(offset) {
            let mut rrect = node.children[cid].area;
            rrect.size.x = width;
            if !rrect.contains_pos(&e.pos) {
                cid += self.widths.len();
                continue;
            }

            let m = self.on_event_row(node, e, &mut cid, row_id, row);
            if !m.is_none() {
                return m;
            }
        }
        EventResult::None
    }
}

impl<M: Clone + 'static> Table<M> {
    fn calc_metrics(&self, area: Rect) -> TableMetrics {
        let mut widths = self.calc_widths(area.width());
        let mut header_height = self.calc_header_height(&area, &widths);
        let mut inner_rect = area.inner(Padding::top(header_height));
        let (mut heights, total) = self.calc_heights(&widths);

        let scrollbar = self.force_scrollbar || inner_rect.height() < total;
        if scrollbar {
            inner_rect = area.inner(Padding::right(1));
            widths = self.calc_widths(inner_rect.width());
            header_height = self.calc_header_height(&inner_rect, &widths);
            (heights, _) = self.calc_heights(&widths);
            inner_rect = inner_rect.inner(Padding::top(header_height));
        }

        TableMetrics {
            widths,
            heights,
            header_height,
            scrollbar,
            rect: inner_rect,
        }
    }

    fn calc_header_height(&self, rect: &Rect, widths: &[usize]) -> usize {
        let mut header_height = self.header_separator.is_some() as usize;
        if let Some(header) = &self.header {
            header_height += Self::row_height(rect.height(), header, widths);
        }
        header_height
    }

    /// Gets calculated column widths based on the given size
    fn calc_widths(&self, mut width: usize) -> Vec<usize> {
        width -= self.column_spacing * self.widths.len().saturating_sub(1);
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

    fn render_clipped(
        &self,
        buffer: &mut Buffer,
        layout: &LayoutNode,
        row: &Row<M>,
        rect: &mut Rect,
        cid: &mut usize,
    ) {
        let row_height = layout.children[*cid].area.height();
        if rect.height() >= row_height {
            self.render_row(buffer, layout, row, cid);
            return;
        }

        let mut cell = Cell::empty();
        cell.style(row.style);

        let mut trect = *rect;
        trect.size.y = row_height;
        let mut temp_buffer = Buffer::filled(trect, cell);
        temp_buffer.merge(buffer.subset(*rect));

        self.render_row(&mut temp_buffer, layout, row, cid);
        buffer.merge(temp_buffer.subset(*rect));
    }

    fn render_row(
        &self,
        buffer: &mut Buffer,
        layout: &LayoutNode,
        row: &Row<M>,
        cid: &mut usize,
    ) {
        for cell in row.cells.iter() {
            let cnode = &layout.children[*cid];
            cell.render(buffer, cnode);
            *cid += 1;
        }
    }

    fn on_event_row(
        &self,
        node: &LayoutNode,
        event: &MouseEvent,
        id: &mut usize,
        row_id: usize,
        row: &Row<M>,
    ) -> EventResult<M> {
        for (i, child) in row.cells.iter().enumerate() {
            let cnode = &node.children[*id];
            *id += 1;
            if !cnode.area.contains_pos(&event.pos) {
                continue;
            }

            return child
                .on_event(cnode, event)
                .or_else(|| self.handle_mouse(i, row_id, event));
        }
        EventResult::None
    }

    fn set_sel_style(
        &self,
        buffer: &mut Buffer,
        rect: &Rect,
        col_x: usize,
        col_w: usize,
        rrect: Option<Rect>,
    ) {
        if let Some(r) = rrect {
            buffer.set_area_style(self.selected_row_style, r);
        }

        if col_w == 0 || self.state.borrow().selected_column.is_none() {
            return;
        };

        let column_rect = Rect::new(col_x, rect.y(), col_w, rect.height());
        buffer.set_area_style(self.selected_column_style, column_rect);

        if let Some(r) = rrect {
            buffer.set_area_style(
                self.selected_cell_style,
                r.intersection(&column_rect),
            )
        }
    }

    fn render_header(
        &self,
        buffer: &mut Buffer,
        node: &LayoutNode,
        rect: &mut Rect,
        cid: &mut usize,
    ) -> usize {
        let Some(header) = &self.header else {
            return 0;
        };

        let mut height = node.children[*cid].area.height();
        let mut most_right = rect.x();
        for child in header.cells.iter() {
            let cnode = &node.children[*cid];
            child.render(buffer, cnode);
            most_right = cnode.area.right();
            *cid += 1;
        }

        if let Some(separator) = &self.header_separator {
            let w = most_right.saturating_sub(rect.x() + 1);
            let line = separator.get(Border::TOP).to_string().repeat(w);
            buffer.set_str(line, &Vec2::new(rect.x(), rect.y() + height));
            height += 1;
        }

        *rect = rect.inner(Padding::top(height));
        height
    }

    fn on_event_header(
        &self,
        node: &LayoutNode,
        event: &MouseEvent,
    ) -> EventResult<M> {
        let Some(header) = &self.header else {
            return EventResult::None;
        };

        let mut cid = 1;
        for child in header.cells.iter() {
            let m = child.on_event(&node.children[cid], event);
            if !m.is_none() {
                return m;
            }
            cid += 1;
        }
        EventResult::None
    }

    fn row_height(height: usize, row: &Row<M>, widths: &[usize]) -> usize {
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

    fn handle_mouse(
        &self,
        x: usize,
        y: usize,
        event: &MouseEvent,
    ) -> EventResult<M> {
        use MouseEventKind::*;

        match &event.kind {
            Down(button) => self
                .handlers
                .iter()
                .find(|(b, _)| b == button)
                .map(|(_, m)| EventResult::Response(m(x, y)))
                .unwrap_or(EventResult::None),
            ScrollDown if event.modifiers.contains(KeyModifiers::SHIFT) => {
                self.move_col(self.scroll_step.x as isize)
            }
            ScrollUp if event.modifiers.contains(KeyModifiers::SHIFT) => {
                self.move_col(-(self.scroll_step.x as isize))
            }
            ScrollDown => self.move_row(self.scroll_step.y as isize),
            ScrollUp => self.move_row(-(self.scroll_step.y as isize)),
            ScrollLeft => self.move_col(-(self.scroll_step.x as isize)),
            ScrollRight => self.move_col(self.scroll_step.x as isize),
            _ => EventResult::None,
        }
    }

    fn move_row(&self, delta: isize) -> EventResult<M> {
        let scroll = || {
            let mut s = self.state.borrow_mut();
            s.selected = s
                .selected
                .map(|sel| Self::move_selection(sel, delta, self.rows.len()));
        };
        self.handle_scroll(&self.on_scroll_ver, scroll, delta)
    }

    fn move_col(&self, delta: isize) -> EventResult<M> {
        let scroll = || {
            let mut s = self.state.borrow_mut();
            s.selected_column = s.selected_column.map(|sel| {
                Self::move_selection(sel, delta, self.widths.len())
            });
        };
        self.handle_scroll(&self.on_scroll_hor, scroll, delta)
    }

    fn handle_scroll<F>(
        &self,
        handler: &Option<Box<dyn Fn(isize) -> M>>,
        scroll: F,
        delta: isize,
    ) -> EventResult<M>
    where
        F: Fn(),
    {
        if let Some(handler) = handler {
            return EventResult::Response(handler(delta));
        }

        if !self.handle_scroll {
            return EventResult::None;
        }
        scroll();
        EventResult::Consumed
    }

    fn move_selection(current: usize, delta: isize, max: usize) -> usize {
        if delta < 0 {
            current.saturating_sub(delta.unsigned_abs())
        } else {
            (current + delta as usize).min(max.saturating_sub(1))
        }
    }
}

impl<M: Clone + 'static> From<Table<M>> for Box<dyn Widget<M>> {
    fn from(value: Table<M>) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Table<M>> for Element<M> {
    fn from(value: Table<M>) -> Self {
        Element::new(value)
    }
}
