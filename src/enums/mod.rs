/// Defines enum for Color
mod color;
/// Defines enum for Cursor ANSI codes (changing position,...)
pub mod cursor;
/// Defines enum for modifier ANSI codes (bold, italic,...)
pub mod modifier;
/// Defines RGB structure for better work with colors
pub mod rgb;
/// Defines enum for wrap
pub mod wrap;

pub use color::*;
