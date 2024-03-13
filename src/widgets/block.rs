use std::{
    cmp::{max, min},
    iter::repeat,
};

use crate::{
    borders,
    enums::{cursor::Cursor, fg::Fg},
    geometry::{
        constrain::Constrain, coords::Coords, direction::Direction,
        padding::Padding,
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
    fn render(&self, pos: &Coords, size: &Coords) {
        self.render_border(pos, size);

        print!("{}", self.title.get_mods());
        self.title.render_offset(
            &Coords::new(pos.x + 1, pos.y),
            &Coords::new(size.x.saturating_sub(2), 1),
            0,
            None,
        );
        print!("\x1b[0m");

        let (width, height) = self.border_size();
        let top = ((self.borders & Border::TOP) != 0
            || !self.title.get_text().is_empty()) as usize;
        let left = ((self.borders & Border::LEFT) != 0) as usize;
        self.layout.render(
            &Coords::new(pos.x + left, pos.y + top),
            &Coords::new(
                size.x.saturating_sub(width),
                size.y.saturating_sub(height),
            ),
        );
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
    fn render_border(&self, pos: &Coords, size: &Coords) {
        let right_top = Coords::new(pos.x + size.x - 1, pos.y);
        let left_bottom = Coords::new(pos.x, size.y + pos.y - 1);

        print!("{}", self.border_color);
        self.render_ver_border(size.y, pos, Border::LEFT);
        self.render_ver_border(size.y, &right_top, Border::RIGHT);
        self.render_hor_border(size.x, pos, Border::TOP);
        self.render_hor_border(size.x, &left_bottom, Border::BOTTOM);

        if size.x > 1 && size.y > 1 {
            self.render_corner(pos, borders!(TOP, LEFT));
            self.render_corner(&right_top, borders!(TOP, RIGHT));
            self.render_corner(&left_bottom, borders!(BOTTOM, LEFT));
            self.render_corner(
                &Coords::new(right_top.x, left_bottom.y),
                borders!(BOTTOM, RIGHT),
            );
        }
    }

    /// Render horizontal border
    fn render_hor_border(&self, width: usize, pos: &Coords, border: u8) {
        if (self.borders & border) != 0 {
            let hor = self.border_type.get(border);
            let hor_border: String = repeat(hor).take(width).collect();
            print!("{}{hor_border}", Cursor::Pos(pos.x, pos.y));
        }
    }

    /// Render vertical border
    fn render_ver_border(&self, height: usize, pos: &Coords, border: u8) {
        if (self.borders & border) != 0 {
            let ver = self.border_type.get(border);
            for y in 0..height {
                print!("{}{ver}", Cursor::Pos(pos.x, pos.y + y));
            }
        }
    }

    /// Renders corner of [`Block`] border if needed based on `border` value
    fn render_corner(&self, pos: &Coords, border: u8) {
        let c = self.border_type.get(border);
        if (self.borders & border) == border {
            print!("{}{c}", Cursor::Pos(pos.x, pos.y));
        }
    }

    /// Gets border size
    fn border_size(&self) -> (usize, usize) {
        let title = (!self.title.get_text().is_empty()) as usize;
        (
            self._border_size(borders!(LEFT, RIGHT)),
            min(self._border_size(borders!(TOP, BOTTOM)) + title, 2),
        )
    }

    /// Gets border size in given border positions
    fn _border_size(&self, border: u8) -> usize {
        let val = self.borders & border;
        if val == border {
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
