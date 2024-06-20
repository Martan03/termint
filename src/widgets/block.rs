use std::{cmp::max, iter::repeat};

use crate::{
    borders,
    buffer::buffer::Buffer,
    enums::{cursor::Cursor, fg::Fg},
    geometry::{
        constrain::Constrain, coords::Coords, direction::Direction,
        padding::Padding, rect::Rect,
    },
    widgets::span::Span,
};

use super::{
    border::{Border, BorderType},
    layout::Layout,
    text::Text,
    widget::Widget,
};

/// [`Layout`] widget with addition of optional border, title and styles
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     enums::fg::Fg,
/// #        geometry::{
/// #         constrain::Constrain, coords::Coords, direction::Direction,
/// #     },
/// #     widgets::{
/// #         block::Block, border::BorderType, span::StrSpanExtension,
/// #         widget::Widget,
/// #     },
/// # };
/// // Creates block with title Termint in red
/// // with double line border in lightgray
/// // Block layout will be horizontal
/// let mut main = Block::new()
///     .title("Termint".fg(Fg::Red))
///     .direction(Direction::Horizontal)
///     .border_type(BorderType::Double)
///     .border_color(Fg::LightGray);
///
/// // Adds two Block widgets as children for demonstration
/// let mut block1 = Block::new().title("Sub block".to_span());
/// main.add_child(block1, Constrain::Percent(50));
///
/// let mut block2 = Block::new().title("Another".to_span());
/// main.add_child(block2, Constrain::Percent(50));
///
/// // Renders block on coordinates 1, 1, with width 30 and height 8
/// main.render(&Coords::new(1, 1), &Coords::new(30, 8));
/// ```
#[derive(Debug)]
pub struct Block {
    title: Box<dyn Text>,
    borders: u8,
    border_type: BorderType,
    border_color: Fg,
    layout: Layout,
}

impl Block {
    /// Creates new [`Block`] with no title and all borders
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets [`Text`] as a title of [`Block`]
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Box<dyn Text>>,
    {
        self.title = title.into();
        self
    }

    /// Sets on which sides border of the [`Block`] should be rendered
    pub fn borders(mut self, borders: u8) -> Self {
        self.borders = borders;
        self
    }

    /// Sets [`BorderType`] of the [`Block`]
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Sets [`Block`] border color
    pub fn border_color(mut self, color: Fg) -> Self {
        self.border_color = color;
        self
    }

    /// Sets [`Direction`] of the [`Block`]'s [`Layout`]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout = self.layout.direction(direction);
        self
    }

    /// Sets [`Padding`] of the [`Block`]'s [`Layout`]
    pub fn padding<T: Into<Padding>>(mut self, padding: T) -> Self {
        self.layout = self.layout.padding(padding);
        self
    }

    /// Makes [`Block`] center its content
    pub fn center(mut self) -> Self {
        self.layout = self.layout.center();
        self
    }

    /// Adds child to the [`Block`]'s [`Layout`]
    pub fn add_child<T>(&mut self, child: T, constrain: Constrain)
    where
        T: Into<Box<dyn Widget>>,
    {
        self.layout.add_child(child, constrain);
    }
}

impl Widget for Block {
    /// Renders [`Block`] with selected borders and title
    fn render(&self, buffer: &mut Buffer) {
        self.render_border(buffer);

        let mut res = String::new();
        res.push_str(&self.title.get_mods());

        let mut tbuffer = Buffer::empty(Rect::new(
            buffer.x() + 1,
            buffer.y(),
            buffer.width().saturating_sub(2),
            1,
        ));
        let (title_str, _) = self.title.get_offset(&mut tbuffer, 0, None);
        res.push_str(&title_str);
        res.push_str("\x1b[0m");

        let (width, height) = self.border_size();
        let top = ((self.borders & Border::TOP) != 0
            || !self.title.get_text().is_empty()) as usize;
        let left = ((self.borders & Border::LEFT) != 0) as usize;

        let mut cbuffer = Buffer::empty(Rect::new(
            buffer.x() + left,
            buffer.y() + top,
            buffer.width().saturating_sub(width),
            buffer.height().saturating_sub(height),
        ));
        self.layout.render(&mut cbuffer);
        buffer.union(cbuffer);
    }

    fn height(&self, size: &Coords) -> usize {
        let (width, height) = self.border_size();
        let size = Coords::new(
            size.x.saturating_sub(width),
            size.y.saturating_sub(height),
        );
        height + self.layout.height(&size)
    }

    fn width(&self, size: &Coords) -> usize {
        let (width, height) = self.border_size();
        let size = Coords::new(
            size.x.saturating_sub(width),
            size.y.saturating_sub(height),
        );
        max(self.layout.width(&size), self.title.get_text().len()) + width
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            title: Box::new(Span::new("")),
            borders: Border::ALL,
            border_type: BorderType::Normal,
            border_color: Fg::Default,
            layout: Default::default(),
        }
    }
}

impl Block {
    /// Renders [`Block`] border
    fn render_border(&self, buffer: &mut Buffer) -> String {
        let mut border = String::new();
        let rt = Coords::new(buffer.x() + buffer.width() - 1, buffer.y());
        let lb = Coords::new(buffer.x(), buffer.height() + buffer.y() - 1);

        border.push_str(&self.border_color.to_string());
        self.ver_border(
            &mut border,
            buffer.height(),
            buffer.pos_ref(),
            Border::LEFT,
        );
        self.ver_border(&mut border, buffer.height(), &rt, Border::RIGHT);
        self.hor_border(
            &mut border,
            buffer.width(),
            buffer.pos_ref(),
            Border::TOP,
        );
        self.hor_border(&mut border, buffer.width(), &lb, Border::BOTTOM);

        if buffer.width() > 1 && buffer.height() > 1 {
            self.corner(&mut border, buffer.pos_ref(), borders!(TOP, LEFT));
            self.corner(&mut border, &rt, borders!(TOP, RIGHT));
            self.corner(&mut border, &lb, borders!(BOTTOM, LEFT));
            self.corner(
                &mut border,
                &Coords::new(rt.x, lb.y),
                borders!(BOTTOM, RIGHT),
            );
        }
        border
    }

    /// Adds horizontal border to the string
    fn hor_border(
        &self,
        res: &mut String,
        width: usize,
        pos: &Coords,
        border: u8,
    ) {
        if (self.borders & border) != 0 {
            let hor = self.border_type.get(border);
            let hor_border: String = repeat(hor).take(width).collect();
            res.push_str(&Cursor::Pos(pos.x, pos.y).to_string());
            res.push_str(&hor_border);
        }
    }

    /// Adds vertical border to the string
    fn ver_border(
        &self,
        res: &mut String,
        height: usize,
        pos: &Coords,
        border: u8,
    ) {
        if (self.borders & border) != 0 {
            let ver = self.border_type.get(border);
            for y in 0..height {
                res.push_str(&Cursor::Pos(pos.x, pos.y + y).to_string());
                res.push(ver);
            }
        }
    }

    /// Adds corner of [`Block`] border to the string
    fn corner(&self, res: &mut String, pos: &Coords, border: u8) {
        let c = self.border_type.get(border);
        if (self.borders & border) == border {
            res.push_str(&Cursor::Pos(pos.x, pos.y).to_string());
            res.push(c);
        }
    }

    /// Gets border size
    fn border_size(&self) -> (usize, usize) {
        (self.hor_border_size(), self.ver_border_size())
    }

    /// Gets horizontal border size
    fn hor_border_size(&self) -> usize {
        let border = borders!(LEFT, RIGHT);
        let val = self.borders & border;
        if val == border {
            2
        } else if val != 0 {
            1
        } else {
            0
        }
    }

    /// Gets vertical border size and acounting title as well
    fn ver_border_size(&self) -> usize {
        let border = borders!(TOP, BOTTOM);
        let val = self.borders & border;
        if val == border
            || (val == Border::BOTTOM && !self.title.get_text().is_empty())
        {
            2
        } else if val != 0 {
            1
        } else {
            0
        }
    }
}

// From implementations
impl From<Block> for Box<dyn Widget> {
    fn from(value: Block) -> Self {
        Box::new(value)
    }
}
