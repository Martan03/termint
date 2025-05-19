use bitflags::bitflags;

use crate::borders;

bitflags! {
    /// A set of flags representing individual border sides.
    ///
    /// This type allows you to specify one or more border sides (top, right,
    /// bottom, left) for rendering widgets with borders. You can combine sides
    /// using bitwise OR (`|`) or the [`borders!`] macro for better
    /// readability.
    ///
    /// # Predefined constants:
    /// - [`Border::TOP`] – top side
    /// - [`Border::LEFT`] – left side
    /// - [`Border::RIGHT`] – right side
    /// - [`Border::BOTTOM`] – bottom side
    /// - [`Border::NONE`] - no sides
    /// - [`Border::ALL`] – all sides
    ///
    /// # Examples
    ///
    /// ## Using binary OR
    /// ```rust
    /// # use termint::enums::Border;
    /// let sides = Border::TOP | Border::LEFT;
    /// ```
    ///
    /// ## Using the `borders!` macro
    /// ```rust
    /// # use termint::borders;
    /// let sides = borders!(TOP, LEFT);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Border: u8 {
        const TOP = 0b0001;
        const RIGHT = 0b0010;
        const BOTTOM = 0b0100;
        const LEFT = 0b1000;

        const NONE = 0b0000;
        const ALL = 0b1111;
    }
}

/// Defines the visual style of a border.
///
/// This enum specifies how borders are drawn. You can use different types of
/// borders to adjust the style, such as with rounded corners, double lines or
/// more.
#[derive(Debug, Default)]
pub enum BorderType {
    /// Simple line
    #[default]
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
    /// Returns the corresponding character for the given border sides, based
    /// on the current [`BorderType`].
    ///
    /// This is used internally for rendering, but may also be useful for
    /// custom widgets.
    ///
    /// # Example
    /// ```rust
    /// # use termint::enums::{Border, BorderType};
    /// let border_type = BorderType::Rounded;
    /// let ch = border_type.get(Border::TOP | Border::LEFT);
    /// assert_eq!(ch, '╭');
    /// ```
    pub fn get(&self, border: Border) -> char {
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
    fn get_normal(&self, border: Border) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '─',
            Border::LEFT | Border::RIGHT => '│',
            b if b == (Border::TOP | Border::LEFT) => '┌',
            b if b == (Border::TOP | Border::RIGHT) => '┐',
            b if b == (Border::BOTTOM | Border::LEFT) => '└',
            b if b == (Border::BOTTOM | Border::RIGHT) => '┘',
            b if b == borders!(LEFT, TOP, BOTTOM) => '├',
            b if b == borders!(RIGHT, TOP, BOTTOM) => '┤',
            b if b == borders!(TOP, LEFT, RIGHT) => '┬',
            b if b == borders!(BOTTOM, LEFT, RIGHT) => '┴',
            b if b == borders!(TOP, BOTTOM, LEFT, RIGHT) => '┼',
            _ => ' ',
        }
    }

    /// Gets given border character of Rounded [`BorderType`]
    fn get_rounded(&self, border: Border) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '─',
            Border::LEFT | Border::RIGHT => '│',
            b if b == (Border::TOP | Border::LEFT) => '╭',
            b if b == (Border::TOP | Border::RIGHT) => '╮',
            b if b == (Border::BOTTOM | Border::LEFT) => '╰',
            b if b == (Border::BOTTOM | Border::RIGHT) => '╯',
            b if b == borders!(LEFT, TOP, BOTTOM) => '├',
            b if b == borders!(RIGHT, TOP, BOTTOM) => '┤',
            b if b == borders!(TOP, LEFT, RIGHT) => '┬',
            b if b == borders!(BOTTOM, LEFT, RIGHT) => '┴',
            b if b == borders!(TOP, BOTTOM, LEFT, RIGHT) => '┼',
            _ => ' ',
        }
    }

    /// Gets given border character of Thick [`BorderType`]
    fn get_thicker(&self, border: Border) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '━',
            Border::LEFT | Border::RIGHT => '┃',
            b if b == (Border::TOP | Border::LEFT) => '┏',
            b if b == (Border::TOP | Border::RIGHT) => '┓',
            b if b == (Border::BOTTOM | Border::LEFT) => '┗',
            b if b == (Border::BOTTOM | Border::RIGHT) => '┛',
            b if b == borders!(LEFT, TOP, BOTTOM) => '┣',
            b if b == borders!(RIGHT, TOP, BOTTOM) => '┫',
            b if b == borders!(TOP, LEFT, RIGHT) => '┳',
            b if b == borders!(BOTTOM, LEFT, RIGHT) => '┻',
            b if b == borders!(TOP, BOTTOM, LEFT, RIGHT) => '╋',
            _ => ' ',
        }
    }

    /// Gets given border character of Dash [`BorderType`]
    fn get_thick(&self, border: Border) -> char {
        match border {
            Border::TOP => '▀',
            Border::BOTTOM => '▄',
            Border::LEFT => '▌',
            Border::RIGHT => '▐',
            b if b == (Border::TOP | Border::LEFT) => '▛',
            b if b == (Border::TOP | Border::RIGHT) => '▜',
            b if b == (Border::BOTTOM | Border::LEFT) => '▙',
            b if b == (Border::BOTTOM | Border::RIGHT) => '▟',
            _ => ' ',
        }
    }

    /// Gets given border character of Double [`BorderType`]
    fn get_double(&self, border: Border) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '═',
            Border::LEFT | Border::RIGHT => '║',
            b if b == (Border::TOP | Border::LEFT) => '╔',
            b if b == (Border::TOP | Border::RIGHT) => '╗',
            b if b == (Border::BOTTOM | Border::LEFT) => '╚',
            b if b == (Border::BOTTOM | Border::RIGHT) => '╝',
            b if b == borders!(LEFT, TOP, BOTTOM) => '╠',
            b if b == borders!(RIGHT, TOP, BOTTOM) => '╣',
            b if b == borders!(TOP, LEFT, RIGHT) => '╦',
            b if b == borders!(BOTTOM, LEFT, RIGHT) => '╩',
            b if b == borders!(TOP, BOTTOM, LEFT, RIGHT) => '╬',
            _ => ' ',
        }
    }

    /// Gets given border character of Dash [`BorderType`]
    fn get_dash(&self, border: Border) -> char {
        match border {
            Border::TOP | Border::BOTTOM => '╌',
            Border::LEFT | Border::RIGHT => '╎',
            b if b == (Border::TOP | Border::LEFT) => '┌',
            b if b == (Border::TOP | Border::RIGHT) => '┐',
            b if b == (Border::BOTTOM | Border::LEFT) => '└',
            b if b == (Border::BOTTOM | Border::RIGHT) => '┘',
            b if b == borders!(LEFT, TOP, BOTTOM) => '├',
            b if b == borders!(RIGHT, TOP, BOTTOM) => '┤',
            b if b == borders!(TOP, LEFT, RIGHT) => '┬',
            b if b == borders!(BOTTOM, LEFT, RIGHT) => '┴',
            b if b == borders!(TOP, BOTTOM, LEFT, RIGHT) => '┼',
            _ => ' ',
        }
    }
}
