/// Border sides definition
///
/// Combining sides:
/// Using binary or (`|`):
/// ```rust
/// use termint::widgets::border::Border;
///
/// // Border containing top and left side
/// let sides = Border::TOP | Border::LEFT;
/// ```
/// Using macro:
/// ```rust
/// use termint::{borders, widgets::border::Border};
///
/// // Border containing top and left side
/// let sides = borders!(TOP, LEFT);
/// ```
pub struct Border(u8);

#[allow(unused)]
impl Border {
    pub const TOP: u8 = 0b0001;
    pub const RIGHT: u8 = 0b0010;
    pub const BOTTOM: u8 = 0b0100;
    pub const LEFT: u8 = 0b1000;

    pub const NONE: u8 = 0b0000;
    pub const ALL: u8 = 0b1111;
}

/// BorderType enum
#[derive(Debug)]
pub enum BorderType {
    /// Simple line
    Normal,
    /// Line with rounded corners
    Rounded,
    /// Double line
    Double,
}

impl BorderType {
    /// Gets given border character of [`BorderType`]
    pub fn get(&self, border: u8) -> char {
        match self {
            BorderType::Normal => self.get_normal(border),
            BorderType::Rounded => self.get_rounded(border),
            BorderType::Double => self.get_double(border),
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
            _ => ' ',
        }
    }
}

/// Macro to combine [`Border`] sides
#[macro_export]
macro_rules! borders {
    ($($border:ident),*) => {
        $(Border::$border |)* 0
    };
}
