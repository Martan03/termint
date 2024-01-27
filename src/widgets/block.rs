use std::iter::repeat;

use crate::{
    enums::{cursor::Cursor, fg::Fg},
    geometry::{constrain::Constrain, coords::Coords, direction::Direction},
    widgets::span::Span,
};

use super::{
    border::{Border, BorderType},
    layout::Layout,
    widget::Widget,
};

/// Layout widget with addition of border and title
///
/// ## Example usage:
/// ```rust
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
/// main.add_child(Box::new(block1), Constrain::Percent(50));
///
/// let mut block2 = Block::new().title("Another".to_span());
/// main.add_child(Box::new(block2), Constrain::Percent(50));
///
/// // Renders block on coordinates 1, 1, with width 30 and height 8
/// main.render(&Coords::new(1, 1), &Coords::new(30, 8));
/// ```
#[derive(Debug)]
pub struct Block {
    title: Span,
    borders: u8,
    border_type: BorderType,
    border_color: Fg,
    layout: Layout,
}

impl Block {
    /// Creates new [`Block`] with no title and all borders
    pub fn new() -> Self {
        Self {
            title: Span::new(""),
            borders: Border::ALL,
            border_type: BorderType::Normal,
            border_color: Fg::Default,
            layout: Layout::vertical(),
        }
    }

    /// Sets title of [`Block`]
    pub fn title(mut self, title: Span) -> Self {
        self.title = title;
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

    /// Adds child to the [`Block`]'s [`Layout`]
    pub fn add_child(&mut self, child: Box<dyn Widget>, constrain: Constrain) {
        self.layout.add_child(child, constrain);
    }

    /// Renders corner of [`Block`] border if needed based on `border` value
    fn render_corner(&self, x: usize, y: usize, border: u8) {
        let c = self.border_type.get(border);
        if (self.borders & border) == border {
            println!("{}{c}", Cursor::Pos(x, y));
        }
    }
}

impl Widget for Block {
    /// Renders [`Block`] with selected borders and title
    fn render(&self, pos: &Coords, size: &Coords) {
        print!("{}", self.border_color);
        let ver = self.border_type.get(Border::LEFT);
        if (self.borders & Border::LEFT) != 0 {
            for y in 0..size.y {
                println!("{}{ver}", Cursor::Pos(pos.x, pos.y + y));
            }
        }
        if (self.borders & Border::RIGHT) != 0 {
            for y in 0..size.y {
                println!(
                    "{}{ver}",
                    Cursor::Pos(pos.x + size.x - 1, pos.y + y)
                );
            }
        }

        let hor = self.border_type.get(Border::TOP);
        let hor_border: String = repeat(hor).take(size.x).collect();
        if (self.borders & Border::TOP) != 0 {
            println!("{}{hor_border}", Cursor::Pos(pos.x, pos.y));
        }
        if (self.borders & Border::BOTTOM) != 0 {
            println!("{}{hor_border}", Cursor::Pos(pos.x, pos.y + size.y - 1));
        }

        if size.x != 1 && size.y != 1 {
            self.render_corner(pos.x, pos.y, Border::TOP | Border::LEFT);
            self.render_corner(
                pos.x + size.x - 1,
                pos.y,
                Border::TOP | Border::RIGHT,
            );
            self.render_corner(
                pos.x,
                pos.y + size.y - 1,
                Border::BOTTOM | Border::LEFT,
            );
            self.render_corner(
                pos.x + size.x - 1,
                pos.y + size.y - 1,
                Border::BOTTOM | Border::RIGHT,
            );
        }

        self.title.render(
            &Coords::new(pos.x + 1, pos.y),
            &Coords::new(size.x.saturating_sub(1), 1),
        );

        self.layout.render(
            &Coords::new(pos.x + 1, pos.y + 1),
            &Coords::new(size.x - 2, size.y - 2),
        );
    }
}
