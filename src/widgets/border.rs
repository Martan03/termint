use crate::borders;

/// Border sides definition
///
/// Combining sides:
/// Using binary or (`|`):
/// ```rust
/// # use termint::widgets::Border;
/// // Border containing top and left side
/// let sides = Border::TOP | Border::LEFT;
/// ```
/// Using macro:
/// ```rust
/// # use termint::{borders, widgets::Border};
/// // Border containing top and left side
/// let sides = borders!(TOP, LEFT);
/// ```
pub struct Border;

#[allow(unused)]
impl Border {
    pub const TOP: u8 = 0b0001;
    pub const RIGHT: u8 = 0b0010;
    pub const BOTTOM: u8 = 0b0100;
    pub const LEFT: u8 = 0b1000;

    pub const NONE: u8 = 0b0000;
    pub const ALL: u8 = 0b1111;
}

/// Border type enum
#[derive(Debug)]
pub enum BorderType {
    /// Simple line
    Normal,
    /// Line with rounded corners
    Rounded,
    /// Thicker simple line
    Thicker,
    /// Thick simple line
    Thick,
    /// Double line
    Double,
    /// Dashed line
    Dash,
}

impl BorderType {
    /// Gets given border character of [`BorderType`]
    pub fn get(&self, border: u8) -> char {
        match self {
            BorderType::Normal => self.get_normal(border),
            BorderType::Rounded => self.get_rounded(border),
            BorderType::Thicker => self.get_thicker(border),
            BorderType::Thick => self.get_thick(border),
            BorderType::Double => self.get_double(border),
            BorderType::Dash => self.get_dash(border),
        }
    }

    /// Gets given border character of Normal [`BorderType`]
    fn get_normal(&self, border: u8) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '─',
            Border::LEFT | Border::RIGHT => '│',
            _ if border == (Border::TOP | Border::LEFT) => '┌',
            _ if border == (Border::TOP | Border::RIGHT) => '┐',
            _ if border == (Border::BOTTOM | Border::LEFT) => '└',
            _ if border == (Border::BOTTOM | Border::RIGHT) => '┘',
            _ if border == borders!(LEFT, TOP, BOTTOM) => '├',
            _ if border == borders!(RIGHT, TOP, BOTTOM) => '┤',
            _ if border == borders!(TOP, LEFT, RIGHT) => '┬',
            _ if border == borders!(BOTTOM, LEFT, RIGHT) => '┴',
            _ if border == borders!(TOP, BOTTOM, LEFT, RIGHT) => '┼',
            _ => ' ',
        }
    }

    /// Gets given border character of Rounded [`BorderType`]
    fn get_rounded(&self, border: u8) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '─',
            Border::LEFT | Border::RIGHT => '│',
            _ if border == (Border::TOP | Border::LEFT) => '╭',
            _ if border == (Border::TOP | Border::RIGHT) => '╮',
            _ if border == (Border::BOTTOM | Border::LEFT) => '╰',
            _ if border == (Border::BOTTOM | Border::RIGHT) => '╯',
            _ if border == borders!(LEFT, TOP, BOTTOM) => '├',
            _ if border == borders!(RIGHT, TOP, BOTTOM) => '┤',
            _ if border == borders!(TOP, LEFT, RIGHT) => '┬',
            _ if border == borders!(BOTTOM, LEFT, RIGHT) => '┴',
            _ if border == borders!(TOP, BOTTOM, LEFT, RIGHT) => '┼',
            _ => ' ',
        }
    }

    /// Gets given border character of Thick [`BorderType`]
    fn get_thicker(&self, border: u8) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '━',
            Border::LEFT | Border::RIGHT => '┃',
            _ if border == (Border::TOP | Border::LEFT) => '┏',
            _ if border == (Border::TOP | Border::RIGHT) => '┓',
            _ if border == (Border::BOTTOM | Border::LEFT) => '┗',
            _ if border == (Border::BOTTOM | Border::RIGHT) => '┛',
            _ if border == borders!(LEFT, TOP, BOTTOM) => '┣',
            _ if border == borders!(RIGHT, TOP, BOTTOM) => '┫',
            _ if border == borders!(TOP, LEFT, RIGHT) => '┳',
            _ if border == borders!(BOTTOM, LEFT, RIGHT) => '┻',
            _ if border == borders!(TOP, BOTTOM, LEFT, RIGHT) => '╋',
            _ => ' ',
        }
    }

    /// Gets given border character of Dash [`BorderType`]
    fn get_thick(&self, border: u8) -> char {
        match border {
            Border::TOP => '▀',
            Border::BOTTOM => '▄',
            Border::LEFT => '▌',
            Border::RIGHT => '▐',
            _ if border == (Border::TOP | Border::LEFT) => '▛',
            _ if border == (Border::TOP | Border::RIGHT) => '▜',
            _ if border == (Border::BOTTOM | Border::LEFT) => '▙',
            _ if border == (Border::BOTTOM | Border::RIGHT) => '▟',
            _ => ' ',
        }
    }

    /// Gets given border character of Double [`BorderType`]
    fn get_double(&self, border: u8) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '═',
            Border::LEFT | Border::RIGHT => '║',
            _ if border == (Border::TOP | Border::LEFT) => '╔',
            _ if border == (Border::TOP | Border::RIGHT) => '╗',
            _ if border == (Border::BOTTOM | Border::LEFT) => '╚',
            _ if border == (Border::BOTTOM | Border::RIGHT) => '╝',
            _ if border == borders!(LEFT, TOP, BOTTOM) => '╠',
            _ if border == borders!(RIGHT, TOP, BOTTOM) => '╣',
            _ if border == borders!(TOP, LEFT, RIGHT) => '╦',
            _ if border == borders!(BOTTOM, LEFT, RIGHT) => '╩',
            _ if border == borders!(TOP, BOTTOM, LEFT, RIGHT) => '╬',
            _ => ' ',
        }
    }

    /// Gets given border character of Dash [`BorderType`]
    fn get_dash(&self, border: u8) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '╌',
            Border::LEFT | Border::RIGHT => '╎',
            _ if border == (Border::TOP | Border::LEFT) => '┌',
            _ if border == (Border::TOP | Border::RIGHT) => '┐',
            _ if border == (Border::BOTTOM | Border::LEFT) => '└',
            _ if border == (Border::BOTTOM | Border::RIGHT) => '┘',
            _ if border == borders!(LEFT, TOP, BOTTOM) => '├',
            _ if border == borders!(RIGHT, TOP, BOTTOM) => '┤',
            _ if border == borders!(TOP, LEFT, RIGHT) => '┬',
            _ if border == borders!(BOTTOM, LEFT, RIGHT) => '┴',
            _ if border == borders!(TOP, BOTTOM, LEFT, RIGHT) => '┼',
            _ => ' ',
        }
    }
}
