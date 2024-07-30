use std::cmp::max;

use crate::{
    borders,
    buffer::Buffer,
    enums::Color,
    geometry::{Constraint, Coords, Direction, Padding, Rect},
    widgets::span::Span,
};

use super::{
    border::{Border, BorderType},
    layout::Layout,
    text::Text,
    widget::Widget,
};

/// [`Layout`] widget with border around it
///
/// ## Example usage:
/// ```rust
/// # use termint::{
/// #     buffer::Buffer,
/// #     enums::Color,
/// #     geometry::{Constraint, Rect},
/// #     widgets::{Block, BorderType, StrSpanExtension, Widget},
/// # };
/// // Creates block with title Termint in red
/// // with double line border in lightgray
/// // Block layout will be horizontal
/// let mut main = Block::horizontal()
///     .title("Termint".fg(Color::Red))
///     .border_type(BorderType::Double)
///     .border_color(Color::LightGray);
///
/// // Adds two block widgets as children for demonstration
/// let block1 = Block::vertical().title("Sub block");
/// main.add_child(block1, Constraint::Percent(50));
/// let block2 = Block::vertical().title("Another");
/// main.add_child(block2, Constraint::Percent(50));
///
/// // Renders main block using buffer
/// let mut buffer = Buffer::empty(Rect::new(1, 1, 30, 8));
/// main.render(&mut buffer);
/// buffer.render();
/// ```
#[derive(Debug)]
pub struct Block {
    title: Box<dyn Text>,
    borders: u8,
    border_type: BorderType,
    border_color: Color,
    layout: Layout,
}

impl Block {
    /// Creates new [`Block`] that flexes in given [`Direction`]
    /// with no title and all borders
    pub fn new(direction: Direction) -> Self {
        Self {
            layout: Layout::new(direction),
            ..Default::default()
        }
    }

    /// Creates new [`Block`] with horizontal layout
    /// with no title and all borders
    pub fn horizontal() -> Self {
        Self {
            layout: Layout::horizontal(),
            ..Default::default()
        }
    }

    /// Creates new [`Block`] with horizontal layout
    /// with no title and all borders
    pub fn vertical() -> Self {
        Self {
            layout: Layout::vertical(),
            ..Default::default()
        }
    }

    /// Sets [`Text`] as a title of the [`Block`]
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Box<dyn Text>>,
    {
        self.title = title.into();
        self
    }

    /// Sets which [`Block`] borders should be displayed
    pub fn borders(mut self, borders: u8) -> Self {
        self.borders = borders;
        self
    }

    /// Sets type of the border of the [`Block`]
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Sets [`Block`] border color
    pub fn border_color(mut self, color: Color) -> Self {
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
    pub fn add_child<T>(&mut self, child: T, constrain: Constraint)
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

        let mut tbuffer = buffer.get_subset(Rect::new(
            buffer.x() + 1,
            buffer.y(),
            buffer.width().saturating_sub(2),
            1,
        ));
        _ = self.title.render_offset(&mut tbuffer, 0, None);
        buffer.union(tbuffer);

        let (width, height) = self.border_size();
        let top = ((self.borders & Border::TOP) != 0
            || !self.title.get_text().is_empty()) as usize;
        let left = ((self.borders & Border::LEFT) != 0) as usize;

        let mut cbuffer = buffer.get_subset(Rect::new(
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
            border_color: Color::default(),
            layout: Default::default(),
        }
    }
}

impl Block {
    /// Renders [`Block`] border
    fn render_border(&self, buffer: &mut Buffer) {
        let rt = Coords::new(buffer.x() + buffer.width() - 1, buffer.y());
        let lb = Coords::new(buffer.x(), buffer.height() + buffer.y() - 1);

        self.render_ver_border(buffer, buffer.left(), Border::LEFT);
        self.render_ver_border(buffer, buffer.right(), Border::RIGHT);
        self.render_hor_border(buffer, buffer.top(), Border::TOP);
        self.render_hor_border(buffer, buffer.bottom(), Border::BOTTOM);

        if buffer.width() == 0 || buffer.height() == 0 {
            return;
        }

        self.render_corner(buffer, &buffer.pos(), borders!(TOP, LEFT));
        self.render_corner(buffer, &rt, borders!(TOP, RIGHT));
        self.render_corner(buffer, &lb, borders!(BOTTOM, LEFT));
        self.render_corner(
            buffer,
            &Coords::new(rt.x, lb.y),
            borders!(BOTTOM, RIGHT),
        );
    }

    /// Adds horizontal border to the string
    fn render_hor_border(&self, buffer: &mut Buffer, y: usize, border: u8) {
        if (self.borders & border) != 0 {
            let hor = self.border_type.get(border);

            for x in buffer.x()..buffer.width() + buffer.x() {
                let coords = Coords::new(x, y);
                buffer.set_val(hor, &coords);
                buffer.set_fg(self.border_color, &coords);
            }
        }
    }

    /// Adds vertical border to the string
    fn render_ver_border(&self, buffer: &mut Buffer, x: usize, border: u8) {
        if (self.borders & border) != 0 {
            let ver = self.border_type.get(border);

            for y in buffer.y()..buffer.height() + buffer.y() {
                let coords = Coords::new(x, y);
                buffer.set_val(ver, &coords);
                buffer.set_fg(self.border_color, &coords);
            }
        }
    }

    /// Adds corner of [`Block`] border to the string
    fn render_corner(&self, buffer: &mut Buffer, pos: &Coords, border: u8) {
        if (self.borders & border) == border {
            let c = self.border_type.get(border);
            buffer.set_val(c, pos);
            buffer.set_fg(self.border_color, pos);
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
